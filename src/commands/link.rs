use anyhow::{bail, Result};
use rusqlite::Connection;
use serde::Serialize;

use crate::core::types::Link;

// ── slug → page_id resolution ────────────────────────────────

/// Resolve a slug to its integer page ID. Returns an error if the page doesn't exist.
fn resolve_page_id(db: &Connection, slug: &str) -> Result<i64> {
    db.query_row("SELECT id FROM pages WHERE slug = ?1", [slug], |row| {
        row.get(0)
    })
    .map_err(|_| anyhow::anyhow!("page not found: {slug}"))
}

// ── gbrain link ──────────────────────────────────────────────

/// Create a typed temporal link between two pages, or close an existing one.
///
/// If a link already exists matching (from, to, relationship) and `valid_until`
/// is provided, the existing link's `valid_until` is updated (close scenario).
/// Otherwise a new link row is inserted.
pub fn run(
    db: &Connection,
    from: &str,
    to: &str,
    relationship: &str,
    valid_from: Option<String>,
    valid_until: Option<String>,
) -> Result<()> {
    let from_id = resolve_page_id(db, from)?;
    let to_id = resolve_page_id(db, to)?;

    // Close scenario: existing link + valid_until supplied → update.
    if let Some(ref until) = valid_until {
        let rows = db.execute(
            "UPDATE links SET valid_until = ?1 \
             WHERE from_page_id = ?2 AND to_page_id = ?3 AND relationship = ?4 \
               AND valid_until IS NULL",
            rusqlite::params![until, from_id, to_id, relationship],
        )?;

        if rows > 0 {
            println!("Closed link {from} → {to} ({relationship}) valid_until={until}");
            return Ok(());
        }
    }

    // Create scenario: insert a new link row.
    db.execute(
        "INSERT INTO links (from_page_id, to_page_id, relationship, valid_from, valid_until) \
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![from_id, to_id, relationship, valid_from, valid_until],
    )?;

    println!("Linked {from} → {to} ({relationship})");
    Ok(())
}

// ── gbrain link-close ────────────────────────────────────────

/// Close a temporal link interval by its database ID.
pub fn close(db: &Connection, link_id: u64, valid_until: &str) -> Result<()> {
    let rows = db.execute(
        "UPDATE links SET valid_until = ?1 WHERE id = ?2",
        rusqlite::params![valid_until, link_id],
    )?;

    if rows == 0 {
        bail!("link not found: id {link_id}");
    }

    println!("Closed link {link_id} valid_until={valid_until}");
    Ok(())
}

// ── gbrain links ─────────────────────────────────────────────

/// Serialisable link row for JSON output.
#[derive(Debug, Serialize)]
struct LinkRow {
    id: i64,
    to_slug: String,
    relationship: String,
    valid_from: Option<String>,
    valid_until: Option<String>,
}

/// List all outbound links for a page.
pub fn links(db: &Connection, slug: &str, _temporal: Option<String>, json: bool) -> Result<()> {
    let from_id = resolve_page_id(db, slug)?;

    let mut stmt = db.prepare(
        "SELECT l.id, p.slug, l.relationship, l.valid_from, l.valid_until \
         FROM links l JOIN pages p ON l.to_page_id = p.id \
         WHERE l.from_page_id = ?1 \
         ORDER BY l.created_at DESC",
    )?;

    let rows: Vec<LinkRow> = stmt
        .query_map([from_id], |row| {
            Ok(LinkRow {
                id: row.get(0)?,
                to_slug: row.get(1)?,
                relationship: row.get(2)?,
                valid_from: row.get(3)?,
                valid_until: row.get(4)?,
            })
        })?
        .filter_map(Result::ok)
        .collect();

    if json {
        println!("{}", serde_json::to_string_pretty(&rows)?);
    } else {
        if rows.is_empty() {
            println!("No outbound links for {slug}");
        }
        for r in &rows {
            let validity = format_validity(&r.valid_from, &r.valid_until);
            println!(
                "[{}] → {} ({}){}",
                r.id, r.to_slug, r.relationship, validity
            );
        }
    }

    Ok(())
}

// ── gbrain unlink ────────────────────────────────────────────

/// Remove a cross-reference entirely.
pub fn unlink(db: &Connection, from: &str, to: &str, relationship: Option<String>) -> Result<()> {
    let from_id = resolve_page_id(db, from)?;
    let to_id = resolve_page_id(db, to)?;

    let rows = if let Some(ref rel) = relationship {
        db.execute(
            "DELETE FROM links WHERE from_page_id = ?1 AND to_page_id = ?2 AND relationship = ?3",
            rusqlite::params![from_id, to_id, rel],
        )?
    } else {
        db.execute(
            "DELETE FROM links WHERE from_page_id = ?1 AND to_page_id = ?2",
            rusqlite::params![from_id, to_id],
        )?
    };

    if rows == 0 {
        bail!("no matching link found between {from} and {to}");
    }

    println!("Removed {rows} link(s) {from} → {to}");
    Ok(())
}

// ── gbrain backlinks ─────────────────────────────────────────

/// List backlinks (inbound links) for a page.
pub fn backlinks(db: &Connection, slug: &str, _temporal: Option<String>, json: bool) -> Result<()> {
    let to_id = resolve_page_id(db, slug)?;

    let mut stmt = db.prepare(
        "SELECT l.id, p.slug, l.relationship, l.valid_from, l.valid_until \
         FROM links l JOIN pages p ON l.from_page_id = p.id \
         WHERE l.to_page_id = ?1 \
         ORDER BY l.created_at DESC",
    )?;

    let rows: Vec<LinkRow> = stmt
        .query_map([to_id], |row| {
            Ok(LinkRow {
                id: row.get(0)?,
                to_slug: row.get(1)?,
                relationship: row.get(2)?,
                valid_from: row.get(3)?,
                valid_until: row.get(4)?,
            })
        })?
        .filter_map(Result::ok)
        .collect();

    if json {
        println!("{}", serde_json::to_string_pretty(&rows)?);
    } else {
        if rows.is_empty() {
            println!("No backlinks for {slug}");
        }
        for r in &rows {
            let validity = format_validity(&r.valid_from, &r.valid_until);
            println!(
                "[{}] ← {} ({}){}",
                r.id, r.to_slug, r.relationship, validity
            );
        }
    }

    Ok(())
}

/// Format validity range for display.
fn format_validity(from: &Option<String>, until: &Option<String>) -> String {
    match (from, until) {
        (Some(f), Some(u)) => format!(" [{f}..{u}]"),
        (Some(f), None) => format!(" [{f}..]"),
        (None, Some(u)) => format!(" [..{u}]"),
        (None, None) => String::new(),
    }
}

// ── helper: read a Link struct back from DB ──────────────────

/// Read a link by its database ID, resolving page IDs back to slugs.
#[allow(dead_code)]
pub fn get_link(db: &Connection, link_id: i64) -> Result<Link> {
    let link = db.query_row(
        "SELECT l.id, pf.slug, pt.slug, l.relationship, l.context, \
                l.valid_from, l.valid_until, l.created_at \
         FROM links l \
         JOIN pages pf ON l.from_page_id = pf.id \
         JOIN pages pt ON l.to_page_id = pt.id \
         WHERE l.id = ?1",
        [link_id],
        |row| {
            Ok(Link {
                id: Some(row.get(0)?),
                from_slug: row.get(1)?,
                to_slug: row.get(2)?,
                relationship: row.get(3)?,
                context: row.get(4)?,
                valid_from: row.get(5)?,
                valid_until: row.get(6)?,
                created_at: row.get(7)?,
            })
        },
    )?;
    Ok(link)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::db;

    fn open_test_db() -> Connection {
        let dir = tempfile::TempDir::new().unwrap();
        let db_path = dir.path().join("test_brain.db");
        let conn = db::open(db_path.to_str().unwrap()).unwrap();
        std::mem::forget(dir);
        conn
    }

    fn insert_page(conn: &Connection, slug: &str, page_type: &str) {
        conn.execute(
            "INSERT INTO pages (slug, type, title, summary, compiled_truth, timeline, \
                                frontmatter, wing, room, version) \
             VALUES (?1, ?2, ?3, '', '', '', '{}', '', '', 1)",
            rusqlite::params![slug, page_type, slug],
        )
        .unwrap();
    }

    // ── create link ──────────────────────────────────────────

    #[test]
    fn create_link_inserts_row_into_links_table() {
        let conn = open_test_db();
        insert_page(&conn, "people/alice", "person");
        insert_page(&conn, "companies/acme", "company");

        run(
            &conn,
            "people/alice",
            "companies/acme",
            "works_at",
            Some("2024-01".to_string()),
            None,
        )
        .unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM links", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);

        let link = get_link(&conn, 1).unwrap();
        assert_eq!(link.from_slug, "people/alice");
        assert_eq!(link.to_slug, "companies/acme");
        assert_eq!(link.relationship, "works_at");
        assert_eq!(link.valid_from.as_deref(), Some("2024-01"));
        assert!(link.valid_until.is_none());
    }

    // ── close link ───────────────────────────────────────────

    #[test]
    fn close_link_sets_valid_until_on_existing_link() {
        let conn = open_test_db();
        insert_page(&conn, "people/alice", "person");
        insert_page(&conn, "companies/acme", "company");

        // Create
        run(
            &conn,
            "people/alice",
            "companies/acme",
            "works_at",
            Some("2024-01".to_string()),
            None,
        )
        .unwrap();

        // Close via the same command with --valid-until
        run(
            &conn,
            "people/alice",
            "companies/acme",
            "works_at",
            None,
            Some("2025-06".to_string()),
        )
        .unwrap();

        let link = get_link(&conn, 1).unwrap();
        assert_eq!(link.valid_until.as_deref(), Some("2025-06"));

        // Only one link row — close updated in place, didn't create a second
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM links", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    // ── link-close by ID ─────────────────────────────────────

    #[test]
    fn link_close_by_id_sets_valid_until() {
        let conn = open_test_db();
        insert_page(&conn, "people/bob", "person");
        insert_page(&conn, "companies/beta", "company");

        run(&conn, "people/bob", "companies/beta", "advises", None, None).unwrap();

        close(&conn, 1, "2025-12").unwrap();

        let link = get_link(&conn, 1).unwrap();
        assert_eq!(link.valid_until.as_deref(), Some("2025-12"));
    }

    #[test]
    fn link_close_returns_error_for_nonexistent_id() {
        let conn = open_test_db();
        let result = close(&conn, 999, "2025-01");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("link not found"));
    }

    // ── page not found ───────────────────────────────────────

    #[test]
    fn link_fails_when_from_page_does_not_exist() {
        let conn = open_test_db();
        insert_page(&conn, "companies/acme", "company");

        let result = run(
            &conn,
            "people/ghost",
            "companies/acme",
            "works_at",
            None,
            None,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("page not found"));
    }

    #[test]
    fn link_fails_when_to_page_does_not_exist() {
        let conn = open_test_db();
        insert_page(&conn, "people/alice", "person");

        let result = run(
            &conn,
            "people/alice",
            "companies/ghost",
            "works_at",
            None,
            None,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("page not found"));
    }

    // ── unlink ───────────────────────────────────────────────

    #[test]
    fn unlink_removes_link_row() {
        let conn = open_test_db();
        insert_page(&conn, "people/alice", "person");
        insert_page(&conn, "companies/acme", "company");

        run(
            &conn,
            "people/alice",
            "companies/acme",
            "works_at",
            None,
            None,
        )
        .unwrap();
        assert_eq!(
            conn.query_row("SELECT COUNT(*) FROM links", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            1
        );

        unlink(
            &conn,
            "people/alice",
            "companies/acme",
            Some("works_at".to_string()),
        )
        .unwrap();
        assert_eq!(
            conn.query_row("SELECT COUNT(*) FROM links", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            0
        );
    }

    // ── links / backlinks ────────────────────────────────────

    #[test]
    fn links_lists_outbound_and_backlinks_lists_inbound() {
        let conn = open_test_db();
        insert_page(&conn, "people/alice", "person");
        insert_page(&conn, "companies/acme", "company");

        run(
            &conn,
            "people/alice",
            "companies/acme",
            "works_at",
            None,
            None,
        )
        .unwrap();

        // links (outbound from alice) — should succeed without panic
        links(&conn, "people/alice", None, false).unwrap();
        // backlinks (inbound to acme) — should succeed without panic
        backlinks(&conn, "companies/acme", None, false).unwrap();
    }
}
