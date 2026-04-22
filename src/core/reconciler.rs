// Reconciler: filesystem walk → stat-diff → ingest/quarantine/delete.
//
// This module replaces `import_dir()` from `migrate.rs`. It handles:
// - Cold-start reconciliation on `gbrain serve` startup
// - On-demand sync via `gbrain collection sync`
// - Rename detection (native events, UUID match, content-hash uniqueness)
// - Delete-vs-quarantine classification via `has_db_only_state`
//
// Full implementation is deferred to later tasks. This is a minimal skeleton
// to keep the batch buildable.

#![allow(dead_code)]

use crate::core::collections::Collection;
use crate::core::file_state::FileStat;
use rusqlite::Connection;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

// ── Reconciliation Result ─────────────────────────────────────

/// Summary statistics from a reconciliation pass.
#[derive(Debug, Default, Clone)]
pub struct ReconcileStats {
    pub walked: usize,
    pub unchanged: usize,
    pub modified: usize,
    pub new: usize,
    pub missing: usize,
    pub native_renamed: usize,
    pub hash_renamed: usize,
    pub quarantined_ambiguous: usize,
    pub quarantined_db_state: usize,
    pub hard_deleted: usize,
}

// ── Reconcile (stub) ──────────────────────────────────────────

/// Reconcile a collection's filesystem state against the DB.
///
/// This is a minimal stub for task 5.1. Full implementation includes:
/// - Walk using `ignore::WalkBuilder` bounded to `root_fd` (task 5.2)
/// - Stat-diff: compare filesystem against `file_state` (task 4.3)
/// - Rename resolution: native events > UUID > content-hash (task 5.3)
/// - Delete-vs-quarantine classifier (task 5.4)
/// - Apply updates in batches of 500 (task 5.6)
/// - Per-phase logging (task 5.7)
///
/// For now, this returns empty stats and does nothing.
pub fn reconcile(
    _conn: &Connection,
    _collection: &Collection,
) -> Result<ReconcileStats, ReconcileError> {
    // Stub: return empty stats
    Ok(ReconcileStats::default())
}

/// Full-hash reconciliation: ignore stat fields, hash every file.
///
/// Used by:
/// - `gbrain collection sync --remap-root` (task 5.8)
/// - `gbrain collection restore` (task 5.8)
/// - Fresh attach (task 5.9)
/// - Periodic audit (task 4.6)
///
/// This is a stub for task 4.4.
pub fn full_hash_reconcile(
    _conn: &Connection,
    _collection_id: i64,
) -> Result<ReconcileStats, ReconcileError> {
    // Stub: return empty stats
    Ok(ReconcileStats::default())
}

// ── Stat Diff (stub) ──────────────────────────────────────────

/// Stat-diff result: classify files into changed/unchanged/new/missing sets.
#[derive(Debug, Default)]
pub struct StatDiff {
    pub unchanged: HashSet<PathBuf>,
    pub modified: HashMap<PathBuf, FileStat>,
    pub new: HashMap<PathBuf, FileStat>,
    pub missing: HashSet<PathBuf>,
}

/// Compare filesystem walk against `file_state`; yield changed/unchanged/new/missing sets.
///
/// Files are `unchanged` ONLY when ALL four stat fields match: mtime_ns, ctime_ns, size_bytes, inode.
/// Any mismatch → `modified` (will trigger re-hash).
///
/// This is a stub for task 4.3.
pub fn stat_diff(
    _conn: &Connection,
    _collection_id: i64,
    _root_path: &Path,
) -> Result<StatDiff, ReconcileError> {
    // Stub: return empty diff
    Ok(StatDiff::default())
}

// ── DB-Only State Predicate (stub) ────────────────────────────

/// Determine if a page has DB-only state (state that cannot be reconstructed from markdown).
///
/// A page has DB-only state if ANY of these are true:
/// 1. EXISTS a row in `links` where (`from_page_id = p.id` OR `to_page_id = p.id`) AND `source_kind = 'programmatic'`
/// 2. EXISTS a row in `assertions` where `page_id = p.id` AND `asserted_by != 'import'`
/// 3. EXISTS a row in `raw_data` where `page_id = p.id`
/// 4. EXISTS a row in `contradictions` where `page_id = p.id` OR `other_page_id = p.id`
/// 5. EXISTS a row in `knowledge_gaps` where `page_id = p.id`
///
/// This is a stub for task 5.4. Full implementation requires schema updates:
/// - `links.source_kind` column (task 5.4a)
/// - `knowledge_gaps.page_id` column (task 1.1b)
pub fn has_db_only_state(_conn: &Connection, _page_id: i64) -> Result<bool, ReconcileError> {
    // Stub: always return false (no quarantine protection until schema is complete)
    Ok(false)
}

// ── Error ─────────────────────────────────────────────────────

#[derive(Debug)]
pub enum ReconcileError {
    DbError(rusqlite::Error),
    IoError(std::io::Error),
    Other(String),
}

impl std::fmt::Display for ReconcileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DbError(e) => write!(f, "Database error: {}", e),
            Self::IoError(e) => write!(f, "I/O error: {}", e),
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ReconcileError {}

impl From<rusqlite::Error> for ReconcileError {
    fn from(e: rusqlite::Error) -> Self {
        Self::DbError(e)
    }
}

impl From<std::io::Error> for ReconcileError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconcile_stub_returns_empty_stats() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../schema.sql")).unwrap();
        conn.execute(
            "INSERT INTO collections (name, root_path) VALUES ('test', '/test')",
            [],
        )
        .unwrap();

        let collection = Collection {
            id: 1,
            name: "test".to_owned(),
            root_path: "/test".to_owned(),
            state: crate::core::collections::CollectionState::Active,
            writable: true,
            is_write_target: false,
            ignore_patterns: None,
            ignore_parse_errors: None,
            needs_full_sync: false,
            last_sync_at: None,
            created_at: "2024-01-01T00:00:00Z".to_owned(),
            updated_at: "2024-01-01T00:00:00Z".to_owned(),
        };

        let stats = reconcile(&conn, &collection).unwrap();
        assert_eq!(stats.walked, 0);
        assert_eq!(stats.unchanged, 0);
        assert_eq!(stats.modified, 0);
        assert_eq!(stats.new, 0);
        assert_eq!(stats.missing, 0);
    }

    #[test]
    fn has_db_only_state_stub_returns_false() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../schema.sql")).unwrap();

        let result = has_db_only_state(&conn, 1).unwrap();
        assert!(!result);
    }
}
