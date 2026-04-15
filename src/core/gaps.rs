//! Knowledge gap detection — log, list, and resolve unanswered queries.

use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use thiserror::Error;

use super::types::KnowledgeGap;

#[derive(Debug, Error)]
pub enum GapsError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("gap not found: id {id}")]
    NotFound { id: i64 },
}

/// Log a knowledge gap using the SHA-256 of the query for idempotency.
///
/// Uses `INSERT OR IGNORE` against the UNIQUE index on `query_hash`
/// so duplicate queries produce exactly one gap row.
pub fn log_gap(
    query: &str,
    context: &str,
    confidence_score: Option<f64>,
    conn: &Connection,
) -> Result<(), GapsError> {
    let hash = sha256_hex(query);
    conn.execute(
        "INSERT OR IGNORE INTO knowledge_gaps (query_hash, context, confidence_score, sensitivity) \
         VALUES (?1, ?2, ?3, 'internal')",
        params![hash, context, confidence_score],
    )?;
    Ok(())
}

/// List knowledge gaps, optionally including resolved ones.
pub fn list_gaps(
    resolved: bool,
    limit: usize,
    conn: &Connection,
) -> Result<Vec<KnowledgeGap>, GapsError> {
    let sql = if resolved {
        "SELECT id, query_hash, context, confidence_score, sensitivity, resolved_at, detected_at \
         FROM knowledge_gaps ORDER BY detected_at DESC LIMIT ?1"
    } else {
        "SELECT id, query_hash, context, confidence_score, sensitivity, resolved_at, detected_at \
         FROM knowledge_gaps WHERE resolved_at IS NULL ORDER BY detected_at DESC LIMIT ?1"
    };

    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map(params![limit as i64], |row| {
        Ok(KnowledgeGap {
            id: row.get(0)?,
            query_hash: row.get(1)?,
            context: row.get(2)?,
            confidence_score: row.get(3)?,
            sensitivity: row.get(4)?,
            resolved_at: row.get(5)?,
            detected_at: row.get(6)?,
        })
    })?;

    let mut gaps = Vec::new();
    for row in rows {
        gaps.push(row?);
    }
    Ok(gaps)
}

/// Mark a gap as resolved by linking it to the page that answered the query.
pub fn resolve_gap(id: i64, resolved_by_slug: &str, conn: &Connection) -> Result<(), GapsError> {
    let rows = conn.execute(
        "UPDATE knowledge_gaps SET \
             resolved_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), \
             resolved_by_slug = ?1 \
         WHERE id = ?2 AND resolved_at IS NULL",
        params![resolved_by_slug, id],
    )?;
    if rows == 0 {
        return Err(GapsError::NotFound { id });
    }
    Ok(())
}

fn sha256_hex(data: &str) -> String {
    let digest = Sha256::digest(data.as_bytes());
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        hex.push_str(&format!("{byte:02x}"));
    }
    hex
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::db;

    fn open_test_db() -> Connection {
        db::open(":memory:").expect("open db")
    }

    #[test]
    fn log_gap_inserts_a_row() {
        let conn = open_test_db();
        log_gap(
            "who invented quantum socks",
            "query context",
            Some(0.1),
            &conn,
        )
        .unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM knowledge_gaps", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);

        let sensitivity: String = conn
            .query_row(
                "SELECT sensitivity FROM knowledge_gaps LIMIT 1",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(sensitivity, "internal");
    }

    #[test]
    fn duplicate_query_is_idempotent() {
        let conn = open_test_db();
        log_gap("same query twice", "", None, &conn).unwrap();
        log_gap("same query twice", "", None, &conn).unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM knowledge_gaps", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn list_gaps_returns_only_unresolved_by_default() {
        let conn = open_test_db();
        log_gap("unresolved query", "", None, &conn).unwrap();
        log_gap("resolved query", "", None, &conn).unwrap();

        // Resolve the second gap
        let id: i64 = conn
            .query_row(
                "SELECT id FROM knowledge_gaps WHERE query_hash = ?1",
                [sha256_hex("resolved query")],
                |row| row.get(0),
            )
            .unwrap();
        resolve_gap(id, "answers/quantum", &conn).unwrap();

        let unresolved = list_gaps(false, 100, &conn).unwrap();
        assert_eq!(unresolved.len(), 1);
        assert!(unresolved[0].resolved_at.is_none());

        let all = list_gaps(true, 100, &conn).unwrap();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn resolve_gap_sets_resolved_at() {
        let conn = open_test_db();
        log_gap("test query", "", None, &conn).unwrap();

        let id: i64 = conn
            .query_row("SELECT id FROM knowledge_gaps LIMIT 1", [], |row| {
                row.get(0)
            })
            .unwrap();
        resolve_gap(id, "people/alice", &conn).unwrap();

        let resolved_at: Option<String> = conn
            .query_row(
                "SELECT resolved_at FROM knowledge_gaps WHERE id = ?1",
                [id],
                |row| row.get(0),
            )
            .unwrap();
        assert!(resolved_at.is_some());

        let slug: String = conn
            .query_row(
                "SELECT resolved_by_slug FROM knowledge_gaps WHERE id = ?1",
                [id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(slug, "people/alice");
    }

    #[test]
    fn resolve_gap_returns_error_for_unknown_id() {
        let conn = open_test_db();
        let result = resolve_gap(9999, "people/alice", &conn);
        assert!(result.is_err());
    }
}
