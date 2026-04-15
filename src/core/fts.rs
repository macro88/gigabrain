use rusqlite::Connection;

use super::types::{SearchError, SearchResult};

/// FTS5 full-text search over the `page_fts` virtual table.
///
/// Returns at most `limit` results ranked by BM25 score (most relevant first).
/// When `wing_filter` is provided, only pages in that wing are returned.
/// Returns an empty vec on no matches (not an error).
pub fn search_fts(
    query: &str,
    wing_filter: Option<&str>,
    conn: &Connection,
    limit: usize,
) -> Result<Vec<SearchResult>, SearchError> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let mut sql = String::from(
        "SELECT p.slug, p.title, p.summary, -bm25(page_fts) AS score, p.wing \
         FROM page_fts \
         JOIN pages p ON p.id = page_fts.rowid \
         WHERE page_fts MATCH ?1",
    );

    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    params.push(Box::new(trimmed.to_owned()));

    if let Some(wing) = wing_filter {
        sql.push_str(" AND p.wing = ?2");
        params.push(Box::new(wing.to_owned()));
        sql.push_str(" ORDER BY bm25(page_fts) LIMIT ?3");
    } else {
        // bm25() returns negative values; ascending order = most relevant first.
        sql.push_str(" ORDER BY bm25(page_fts) LIMIT ?2");
    }
    params.push(Box::new(limit as i64));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(SearchResult {
            slug: row.get(0)?,
            title: row.get(1)?,
            summary: row.get(2)?,
            score: row.get(3)?,
            wing: row.get(4)?,
        })
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::db;

    fn open_test_db() -> Connection {
        let dir = tempfile::TempDir::new().unwrap();
        let db_path = dir.path().join("test_brain.db");
        let conn = db::open(db_path.to_str().unwrap()).unwrap();
        // Leak TempDir so the DB file stays alive for the test.
        std::mem::forget(dir);
        conn
    }

    fn insert_page(
        conn: &Connection,
        slug: &str,
        title: &str,
        wing: &str,
        summary: &str,
        compiled_truth: &str,
    ) {
        conn.execute(
            "INSERT INTO pages (slug, type, title, summary, compiled_truth, \
                                timeline, frontmatter, wing, room, version) \
             VALUES (?1, 'concept', ?2, ?3, ?4, '', '{}', ?5, '', 1)",
            rusqlite::params![slug, title, summary, compiled_truth, wing],
        )
        .unwrap();
    }

    // ── search on empty DB ──────────────────────────────────────

    #[test]
    fn search_on_empty_db_returns_empty_vec() {
        let conn = open_test_db();
        let results = search_fts("anything", None, &conn, 1000).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn search_with_empty_query_returns_empty_vec() {
        let conn = open_test_db();
        insert_page(&conn, "test/a", "Test A", "test", "summary", "content");
        let results = search_fts("", None, &conn, 1000).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn search_with_whitespace_query_returns_empty_vec() {
        let conn = open_test_db();
        insert_page(&conn, "test/a", "Test A", "test", "summary", "content");
        let results = search_fts("   ", None, &conn, 1000).unwrap();
        assert!(results.is_empty());
    }

    // ── basic keyword match ─────────────────────────────────────

    #[test]
    fn search_finds_page_by_content_keyword() {
        let conn = open_test_db();
        insert_page(
            &conn,
            "concepts/ml",
            "Machine Learning",
            "concepts",
            "ML overview",
            "Machine learning is a branch of artificial intelligence.",
        );

        let results = search_fts("machine learning", None, &conn, 1000).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].slug, "concepts/ml");
        assert_eq!(results[0].title, "Machine Learning");
        assert!(results[0].score > 0.0);
    }

    #[test]
    fn search_finds_page_by_title_keyword() {
        let conn = open_test_db();
        insert_page(
            &conn,
            "people/alice",
            "Alice Johnson",
            "people",
            "Engineer at Acme",
            "Works on distributed systems.",
        );

        let results = search_fts("alice", None, &conn, 1000).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].slug, "people/alice");
    }

    #[test]
    fn search_returns_no_match_for_absent_term() {
        let conn = open_test_db();
        insert_page(
            &conn,
            "concepts/ml",
            "Machine Learning",
            "concepts",
            "ML overview",
            "Machine learning is a branch of artificial intelligence.",
        );

        let results = search_fts("zzzznonexistent", None, &conn, 1000).unwrap();
        assert!(results.is_empty());
    }

    // ── wing filter ─────────────────────────────────────────────

    #[test]
    fn search_with_wing_filter_returns_only_matching_wing() {
        let conn = open_test_db();
        insert_page(
            &conn,
            "people/alice",
            "Alice",
            "people",
            "Engineer",
            "Expert in fundraising and venture capital.",
        );
        insert_page(
            &conn,
            "companies/acme",
            "Acme Corp",
            "companies",
            "Startup",
            "A startup focused on fundraising technology.",
        );

        let results = search_fts("fundraising", Some("companies"), &conn, 1000).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].slug, "companies/acme");
    }

    #[test]
    fn search_without_wing_filter_returns_all_matching_pages() {
        let conn = open_test_db();
        insert_page(
            &conn,
            "people/alice",
            "Alice",
            "people",
            "Engineer",
            "Expert in fundraising and venture capital.",
        );
        insert_page(
            &conn,
            "companies/acme",
            "Acme Corp",
            "companies",
            "Startup",
            "A startup focused on fundraising technology.",
        );

        let results = search_fts("fundraising", None, &conn, 1000).unwrap();
        assert_eq!(results.len(), 2);
    }

    // ── BM25 ranking ────────────────────────────────────────────

    #[test]
    fn search_results_are_ranked_by_relevance() {
        let conn = open_test_db();
        // Page with term appearing many times should rank higher
        insert_page(
            &conn,
            "concepts/ai-deep",
            "AI Deep Dive",
            "concepts",
            "Deep AI study",
            "Artificial intelligence is transforming everything. \
             Intelligence research continues. Artificial intelligence systems \
             are being deployed everywhere. Intelligence is key.",
        );
        insert_page(
            &conn,
            "concepts/ai-intro",
            "AI Introduction",
            "concepts",
            "Brief AI mention",
            "A brief note about intelligence in computing.",
        );

        let results = search_fts("intelligence", None, &conn, 1000).unwrap();
        assert_eq!(results.len(), 2);
        // Higher score should come first
        assert!(results[0].score >= results[1].score);
    }

    // ── result struct correctness ───────────────────────────────

    #[test]
    fn search_result_contains_correct_fields() {
        let conn = open_test_db();
        insert_page(
            &conn,
            "people/bob",
            "Bob Smith",
            "people",
            "Bob is a researcher",
            "Bob works on quantum computing research.",
        );

        let results = search_fts("quantum", None, &conn, 1000).unwrap();
        assert_eq!(results.len(), 1);

        let r = &results[0];
        assert_eq!(r.slug, "people/bob");
        assert_eq!(r.title, "Bob Smith");
        assert_eq!(r.summary, "Bob is a researcher");
        assert_eq!(r.wing, "people");
        assert!(r.score > 0.0);
    }
}
