use std::sync::{Arc, Mutex};

use rmcp::model::*;
use rmcp::schemars;
use rmcp::tool;
use rmcp::{ServerHandler, ServiceExt};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::commands::get::get_page;
use crate::core::fts::search_fts;
use crate::core::markdown;
use crate::core::palace;
use crate::core::search::hybrid_search;
use crate::core::types::SearchError;

type DbRef = Arc<Mutex<Connection>>;

const MAX_SLUG_LEN: usize = 512;
const MAX_CONTENT_LEN: usize = 1_048_576; // 1 MB
const MAX_LIMIT: u32 = 1000;

fn validate_slug(slug: &str) -> Result<(), rmcp::Error> {
    if slug.is_empty() {
        return Err(rmcp::Error::new(
            ErrorCode(-32602),
            "invalid slug: must not be empty".to_string(),
            None,
        ));
    }
    if slug.len() > MAX_SLUG_LEN {
        return Err(rmcp::Error::new(
            ErrorCode(-32602),
            format!("invalid slug: exceeds maximum length of {MAX_SLUG_LEN} characters"),
            None,
        ));
    }
    if !slug.bytes().all(|b| {
        b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'/' || b == b'_' || b == b'-'
    }) {
        return Err(rmcp::Error::new(
            ErrorCode(-32602),
            "invalid slug: allowed characters are [a-z0-9/_-]".to_string(),
            None,
        ));
    }
    Ok(())
}

fn validate_content(content: &str) -> Result<(), rmcp::Error> {
    if content.len() > MAX_CONTENT_LEN {
        return Err(rmcp::Error::new(
            ErrorCode(-32602),
            format!(
                "content too large: {} bytes exceeds maximum of {MAX_CONTENT_LEN} bytes",
                content.len()
            ),
            None,
        ));
    }
    Ok(())
}

fn map_db_error(e: rusqlite::Error) -> rmcp::Error {
    if let rusqlite::Error::SqliteFailure(ref err, ref msg) = e {
        // SQLITE_CONSTRAINT_UNIQUE (extended code 2067)
        if err.extended_code == 2067 {
            return rmcp::Error::new(
                ErrorCode(-32009),
                format!(
                    "conflict: {}",
                    msg.as_deref().unwrap_or("unique constraint violation")
                ),
                None,
            );
        }
        // FTS5 parse/syntax errors surface as SQLITE_ERROR with "fts5" in message
        if let Some(ref msg_str) = msg {
            if msg_str.contains("fts5") {
                return rmcp::Error::new(
                    ErrorCode(-32602),
                    format!("invalid search query: {msg_str}"),
                    None,
                );
            }
        }
    }
    rmcp::Error::new(ErrorCode(-32003), format!("database error: {e}"), None)
}

fn map_search_error(e: SearchError) -> rmcp::Error {
    match e {
        SearchError::Sqlite(sqlite_err) => map_db_error(sqlite_err),
        SearchError::Internal { message } => {
            rmcp::Error::new(ErrorCode(-32003), format!("search error: {message}"), None)
        }
    }
}

#[derive(Clone)]
pub struct GigaBrainServer {
    db: DbRef,
}

impl GigaBrainServer {
    pub fn new(conn: Connection) -> Self {
        Self {
            db: Arc::new(Mutex::new(conn)),
        }
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BrainGetInput {
    /// Page slug to retrieve
    pub slug: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BrainPutInput {
    /// Page slug to create or update
    pub slug: String,
    /// Markdown content of the page
    pub content: String,
    /// Expected current version for optimistic concurrency control
    pub expected_version: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BrainQueryInput {
    /// Search query string
    pub query: String,
    /// Optional wing filter
    pub wing: Option<String>,
    /// Maximum results to return
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BrainSearchInput {
    /// FTS5 search query string
    pub query: String,
    /// Optional wing filter
    pub wing: Option<String>,
    /// Maximum results to return
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BrainListInput {
    /// Optional wing filter
    pub wing: Option<String>,
    /// Optional type filter
    pub page_type: Option<String>,
    /// Maximum results to return
    pub limit: Option<u32>,
}

#[tool(tool_box)]
impl GigaBrainServer {
    #[tool(description = "Get a page by slug")]
    fn brain_get(&self, #[tool(aggr)] input: BrainGetInput) -> Result<CallToolResult, rmcp::Error> {
        validate_slug(&input.slug)?;
        let db = self.db.lock().unwrap_or_else(|e| e.into_inner());
        match get_page(&db, &input.slug) {
            Ok(page) => {
                let rendered = markdown::render_page(&page);
                Ok(CallToolResult::success(vec![Content::text(rendered)]))
            }
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("page not found") {
                    Err(rmcp::Error::new(rmcp::model::ErrorCode(-32001), msg, None))
                } else {
                    Err(rmcp::Error::new(rmcp::model::ErrorCode(-32003), msg, None))
                }
            }
        }
    }

    #[tool(description = "Write or update a page")]
    fn brain_put(&self, #[tool(aggr)] input: BrainPutInput) -> Result<CallToolResult, rmcp::Error> {
        validate_slug(&input.slug)?;
        validate_content(&input.content)?;
        let db = self.db.lock().unwrap_or_else(|e| e.into_inner());

        let (frontmatter, body) = markdown::parse_frontmatter(&input.content);
        let (compiled_truth, timeline) = markdown::split_content(&body);
        let summary = markdown::extract_summary(&compiled_truth);
        let wing = palace::derive_wing(&input.slug);
        let room = palace::derive_room(&compiled_truth);
        let title = frontmatter
            .get("title")
            .cloned()
            .unwrap_or_else(|| input.slug.clone());
        let page_type = frontmatter
            .get("type")
            .cloned()
            .unwrap_or_else(|| "concept".to_string());
        let frontmatter_json = serde_json::to_string(&frontmatter).map_err(|e| {
            rmcp::Error::new(
                rmcp::model::ErrorCode(-32002),
                format!("parse error: {e}"),
                None,
            )
        })?;

        let now: String = db
            .query_row("SELECT strftime('%Y-%m-%dT%H:%M:%SZ', 'now')", [], |row| {
                row.get(0)
            })
            .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string());

        let existing_version: Option<i64> = match db
            .prepare("SELECT version FROM pages WHERE slug = ?1")
            .map_err(map_db_error)?
            .query_row([&input.slug], |row| row.get(0))
        {
            Ok(v) => Some(v),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => return Err(map_db_error(e)),
        };

        match existing_version {
            None => {
                // OCC: a client supplying expected_version on a non-existent page has stale
                // state — the page never existed at that version. Reject as a conflict.
                if let Some(n) = input.expected_version {
                    return Err(rmcp::Error::new(
                        rmcp::model::ErrorCode(-32009),
                        format!("conflict: page does not exist at version {n}"),
                        Some(serde_json::json!({ "current_version": null })),
                    ));
                }
                db.execute(
                    "INSERT INTO pages \
                         (slug, type, title, summary, compiled_truth, timeline, \
                          frontmatter, wing, room, version, \
                          created_at, updated_at, truth_updated_at, timeline_updated_at) \
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 1, ?10, ?10, ?10, ?10)",
                    rusqlite::params![
                        input.slug,
                        page_type,
                        title,
                        summary,
                        compiled_truth,
                        timeline,
                        frontmatter_json,
                        wing,
                        room,
                        now,
                    ],
                )
                .map_err(map_db_error)?;
                Ok(CallToolResult::success(vec![Content::text(format!(
                    "Created {} (version 1)",
                    input.slug
                ))]))
            }
            Some(current) => {
                let expected = input.expected_version.ok_or_else(|| {
                    rmcp::Error::new(
                        rmcp::model::ErrorCode(-32009),
                        format!(
                            "conflict: page already exists (current version: {current}). \
                             Provide expected_version to update."
                        ),
                        Some(serde_json::json!({ "current_version": current })),
                    )
                })?;

                let rows = db
                    .execute(
                        "UPDATE pages SET \
                             type = ?1, title = ?2, summary = ?3, \
                             compiled_truth = ?4, timeline = ?5, \
                             frontmatter = ?6, wing = ?7, room = ?8, \
                             version = version + 1, \
                             updated_at = ?9, truth_updated_at = ?9, timeline_updated_at = ?9 \
                         WHERE slug = ?10 AND version = ?11",
                        rusqlite::params![
                            page_type,
                            title,
                            summary,
                            compiled_truth,
                            timeline,
                            frontmatter_json,
                            wing,
                            room,
                            now,
                            input.slug,
                            expected,
                        ],
                    )
                    .map_err(map_db_error)?;

                if rows == 0 {
                    return Err(rmcp::Error::new(
                        rmcp::model::ErrorCode(-32009),
                        format!("conflict: page updated elsewhere (current version: {current})"),
                        Some(serde_json::json!({ "current_version": current })),
                    ));
                }

                Ok(CallToolResult::success(vec![Content::text(format!(
                    "Updated {} (version {})",
                    input.slug,
                    expected + 1
                ))]))
            }
        }
    }

    #[tool(description = "Hybrid semantic + FTS5 query")]
    fn brain_query(
        &self,
        #[tool(aggr)] input: BrainQueryInput,
    ) -> Result<CallToolResult, rmcp::Error> {
        let db = self.db.lock().unwrap_or_else(|e| e.into_inner());

        let limit = input.limit.unwrap_or(10).min(MAX_LIMIT) as usize;
        let results = hybrid_search(&input.query, input.wing.as_deref(), &db, limit)
            .map_err(map_search_error)?;

        let json = serde_json::to_string_pretty(&results)
            .map_err(|e| rmcp::Error::new(rmcp::model::ErrorCode(-32003), e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "FTS5 full-text search")]
    fn brain_search(
        &self,
        #[tool(aggr)] input: BrainSearchInput,
    ) -> Result<CallToolResult, rmcp::Error> {
        let db = self.db.lock().unwrap_or_else(|e| e.into_inner());

        let limit = input.limit.unwrap_or(50).min(MAX_LIMIT) as usize;
        let results = search_fts(&input.query, input.wing.as_deref(), &db, limit)
            .map_err(map_search_error)?;

        let json = serde_json::to_string_pretty(&results)
            .map_err(|e| rmcp::Error::new(rmcp::model::ErrorCode(-32003), e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List pages with optional filters")]
    fn brain_list(
        &self,
        #[tool(aggr)] input: BrainListInput,
    ) -> Result<CallToolResult, rmcp::Error> {
        let db = self.db.lock().unwrap_or_else(|e| e.into_inner());

        let limit = input.limit.unwrap_or(50).min(MAX_LIMIT);
        let mut sql = String::from("SELECT slug, type, summary FROM pages WHERE 1=1");
        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(ref w) = input.wing {
            sql.push_str(" AND wing = ?");
            params.push(Box::new(w.clone()));
        }
        if let Some(ref t) = input.page_type {
            sql.push_str(" AND type = ?");
            params.push(Box::new(t.clone()));
        }
        sql.push_str(" ORDER BY updated_at DESC LIMIT ?");
        params.push(Box::new(limit));

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            params.iter().map(|p| p.as_ref()).collect();
        let mut stmt = db.prepare(&sql).map_err(map_db_error)?;

        #[derive(Serialize)]
        struct ListEntry {
            slug: String,
            #[serde(rename = "type")]
            page_type: String,
            summary: String,
        }

        let rows = stmt
            .query_map(param_refs.as_slice(), |row| {
                Ok(ListEntry {
                    slug: row.get(0)?,
                    page_type: row.get(1)?,
                    summary: row.get(2)?,
                })
            })
            .map_err(map_db_error)?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row.map_err(map_db_error)?);
        }

        let json = serde_json::to_string_pretty(&entries)
            .map_err(|e| rmcp::Error::new(rmcp::model::ErrorCode(-32003), e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

#[tool(tool_box)]
impl ServerHandler for GigaBrainServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("GigaBrain personal knowledge brain".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

/// Run the MCP stdio server with the given database connection.
pub async fn run(conn: Connection) -> anyhow::Result<()> {
    let server = GigaBrainServer::new(conn);
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    let _service = server.serve(transport).await?;
    _service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::db;
    use serde_json::json;

    fn open_test_db() -> (tempfile::TempDir, Connection) {
        let dir = tempfile::TempDir::new().unwrap();
        let db_path = dir.path().join("server.db");
        let conn = db::open(db_path.to_str().unwrap()).unwrap();
        (dir, conn)
    }

    #[test]
    fn get_info_enables_tools_capability_and_exposes_core_tool_methods() {
        let (_dir, conn) = open_test_db();
        let server = GigaBrainServer::new(conn);
        let info = <GigaBrainServer as ServerHandler>::get_info(&server);

        let _tool_methods = (
            GigaBrainServer::brain_get
                as fn(&GigaBrainServer, BrainGetInput) -> Result<CallToolResult, rmcp::Error>,
            GigaBrainServer::brain_put
                as fn(&GigaBrainServer, BrainPutInput) -> Result<CallToolResult, rmcp::Error>,
            GigaBrainServer::brain_query
                as fn(&GigaBrainServer, BrainQueryInput) -> Result<CallToolResult, rmcp::Error>,
            GigaBrainServer::brain_search
                as fn(&GigaBrainServer, BrainSearchInput) -> Result<CallToolResult, rmcp::Error>,
            GigaBrainServer::brain_list
                as fn(&GigaBrainServer, BrainListInput) -> Result<CallToolResult, rmcp::Error>,
        );

        assert!(info.capabilities.tools.is_some());
    }

    #[test]
    fn brain_get_returns_not_found_error_code_for_missing_slug() {
        let (_dir, conn) = open_test_db();
        let server = GigaBrainServer::new(conn);

        let error = server
            .brain_get(BrainGetInput {
                slug: "definitely-does-not-exist".to_string(),
            })
            .unwrap_err();

        assert_eq!(error.code, ErrorCode(-32001));
    }

    #[test]
    fn brain_put_returns_occ_conflict_error_with_current_version_for_stale_write() {
        let (_dir, conn) = open_test_db();
        let server = GigaBrainServer::new(conn);

        server
            .brain_put(BrainPutInput {
                slug: "notes/test".to_string(),
                content: "---\ntitle: Test\ntype: note\n---\nInitial content\n".to_string(),
                expected_version: None,
            })
            .unwrap();

        let error = server
            .brain_put(BrainPutInput {
                slug: "notes/test".to_string(),
                content: "---\ntitle: Test\ntype: note\n---\nUpdated content\n".to_string(),
                expected_version: Some(0),
            })
            .unwrap_err();

        assert_eq!(error.code, ErrorCode(-32009));
        assert_eq!(error.data, Some(json!({ "current_version": 1 })));
    }

    #[test]
    fn brain_put_rejects_update_without_expected_version() {
        let (_dir, conn) = open_test_db();
        let server = GigaBrainServer::new(conn);

        server
            .brain_put(BrainPutInput {
                slug: "notes/occ".to_string(),
                content: "---\ntitle: Test\ntype: note\n---\nInitial\n".to_string(),
                expected_version: None,
            })
            .unwrap();

        let error = server
            .brain_put(BrainPutInput {
                slug: "notes/occ".to_string(),
                content: "---\ntitle: Test\ntype: note\n---\nSneaky overwrite\n".to_string(),
                expected_version: None,
            })
            .unwrap_err();

        assert_eq!(error.code, ErrorCode(-32009));
        assert_eq!(error.data, Some(json!({ "current_version": 1 })));
    }

    #[test]
    fn brain_get_rejects_invalid_slug() {
        let (_dir, conn) = open_test_db();
        let server = GigaBrainServer::new(conn);

        let error = server
            .brain_get(BrainGetInput {
                slug: "Invalid/SLUG!".to_string(),
            })
            .unwrap_err();

        assert_eq!(error.code, ErrorCode(-32602));
    }

    #[test]
    fn brain_put_rejects_oversized_content() {
        let (_dir, conn) = open_test_db();
        let server = GigaBrainServer::new(conn);

        let large_content = "x".repeat(1_048_577);
        let error = server
            .brain_put(BrainPutInput {
                slug: "test/large".to_string(),
                content: large_content,
                expected_version: None,
            })
            .unwrap_err();

        assert_eq!(error.code, ErrorCode(-32602));
    }

    #[test]
    fn brain_put_rejects_empty_slug() {
        let (_dir, conn) = open_test_db();
        let server = GigaBrainServer::new(conn);

        let error = server
            .brain_put(BrainPutInput {
                slug: "".to_string(),
                content: "content".to_string(),
                expected_version: None,
            })
            .unwrap_err();

        assert_eq!(error.code, ErrorCode(-32602));
    }

    #[test]
    fn brain_put_rejects_create_with_expected_version_when_page_does_not_exist() {
        let (_dir, conn) = open_test_db();
        let server = GigaBrainServer::new(conn);

        // Page does not exist; supplying expected_version is a client bug — reject as OCC conflict.
        let error = server
            .brain_put(BrainPutInput {
                slug: "notes/ghost".to_string(),
                content: "---\ntitle: Ghost\ntype: note\n---\nContent\n".to_string(),
                expected_version: Some(3),
            })
            .unwrap_err();

        assert_eq!(error.code, ErrorCode(-32009));
        assert_eq!(error.data, Some(json!({ "current_version": null })));
    }
}
