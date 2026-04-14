// Types defined ahead of consumers (db.rs, search.rs, etc.) — remove when wired.
#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

// ── Page ──────────────────────────────────────────────────────

/// Core knowledge page — the unit of storage in a GigaBrain database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub slug: String,
    #[serde(rename = "type")]
    pub page_type: String,
    pub title: String,
    pub summary: String,
    pub compiled_truth: String,
    pub timeline: String,
    pub frontmatter: HashMap<String, String>,
    pub wing: String,
    pub room: String,
    pub version: i64,
    pub created_at: String,
    pub updated_at: String,
}

// ── Link ──────────────────────────────────────────────────────

/// Typed temporal cross-reference between two pages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: i64,
    pub from_slug: String,
    pub to_slug: String,
    pub relationship: String,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub created_at: String,
}

// ── Tag ───────────────────────────────────────────────────────

/// A single tag attached to a page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub page_id: i64,
    pub tag: String,
}

// ── TimelineEntry ─────────────────────────────────────────────

/// A structured timeline row for a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub id: i64,
    pub page_id: i64,
    pub date: String,
    pub source: String,
    pub summary: String,
    pub detail: String,
    pub created_at: String,
}

// ── SearchResult ──────────────────────────────────────────────

/// A single result from FTS5, vector, or hybrid search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub score: f64,
    pub wing: String,
}

// ── KnowledgeGap ──────────────────────────────────────────────

/// An unanswered query detected by the brain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGap {
    pub id: i64,
    pub query_hash: String,
    pub context: String,
    pub confidence_score: Option<f64>,
    pub sensitivity: String,
    pub resolved_at: Option<String>,
    pub detected_at: String,
}

// ── IngestRecord ──────────────────────────────────────────────

/// An entry in the idempotency audit trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestRecord {
    pub id: i64,
    pub ingest_key: String,
    pub source_type: String,
    pub source_ref: String,
    pub pages_updated: String,
    pub summary: String,
    pub completed_at: String,
}

// ── SearchMergeStrategy ───────────────────────────────────────

/// How hybrid search merges FTS5 and vector result sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchMergeStrategy {
    SetUnion,
    Rrf,
}

impl SearchMergeStrategy {
    pub fn from_config(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "rrf" => Self::Rrf,
            _ => Self::SetUnion,
        }
    }
}

// ── Errors ────────────────────────────────────────────────────

/// Optimistic concurrency control error.
#[derive(Debug, Error)]
pub enum OccError {
    #[error("conflict: page updated elsewhere (current version: {current_version})")]
    Conflict { current_version: i64 },
}

/// Database-layer errors surfaced by `src/core/`.
#[derive(Debug, Error)]
pub enum DbError {
    #[error("page not found: {slug}")]
    NotFound { slug: String },

    #[error("path not found: {path}")]
    PathNotFound { path: String },

    #[error("OCC conflict: {0}")]
    Occ(#[from] OccError),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("schema error: {message}")]
    Schema { message: String },
}
