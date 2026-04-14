//! Inference — text embedding and vector search.
//!
//! **PLACEHOLDER CONTRACT (T14 incomplete):** The [`EmbeddingModel`] in this
//! module is a SHA-256 hash-based shim, **not** the BGE-small-en-v1.5 semantic
//! model. It produces a deterministic, L2-normalised 384-dimensional vector
//! derived from per-token SHA-256 digests — cosine similarity reflects hash
//! proximity, not semantic proximity.
//!
//! `candle-core`, `candle-nn`, `candle-transformers`, and `tokenizers` are
//! declared in `Cargo.toml` but are **not yet wired** (the Candle forward-pass
//! step in T14 is incomplete). Until T14 ships the real BGE-small loader:
//!
//! - `gbrain embed` stores hash vectors, not semantic vectors.
//! - `gbrain query` ranks by hash-cosine distance, not meaning.
//! - `gbrain search` (FTS5) is unaffected and fully keyword-accurate.
//!
//! The public API (`embed`, `search_vec`, `ensure_model`, `embedding_to_blob`)
//! is stable. Replacing this shim with the Candle model requires no caller changes.

use std::sync::OnceLock;

use rusqlite::types::ToSql;
use rusqlite::Connection;
use sha2::{Digest, Sha256};

use super::types::{InferenceError, SearchError, SearchResult};

const EMBEDDING_DIMENSIONS: usize = 384;
const HASH_CHUNK_COUNT: usize = EMBEDDING_DIMENSIONS / 32;

static MODEL: OnceLock<EmbeddingModel> = OnceLock::new();

/// SHA-256 hash-based placeholder for BGE-small-en-v1.5.
///
/// Satisfies the embedding API contract (384-dim, L2-normalised `Vec<f32>`)
/// but vectors are **not** semantically meaningful — cosine similarity scores
/// reflect hash distance, not text similarity.
///
/// The API and lifecycle match the intended Candle-backed implementation so the
/// real BGE-small loader can replace this without changing callers.
#[derive(Debug)]
pub struct EmbeddingModel {
    dimensions: usize,
}

impl EmbeddingModel {
    fn new() -> Self {
        Self {
            dimensions: EMBEDDING_DIMENSIONS,
        }
    }

    fn embed(&self, text: &str) -> Result<Vec<f32>, InferenceError> {
        let mut embedding = vec![0.0; self.dimensions];

        for (token_index, token) in text.split_whitespace().enumerate() {
            self.accumulate_token_embedding(token, token_index, &mut embedding);
        }

        if embedding.iter().all(|value| *value == 0.0) {
            self.accumulate_token_embedding(text, 0, &mut embedding);
        }

        normalize(&mut embedding)?;
        Ok(embedding)
    }

    fn accumulate_token_embedding(&self, token: &str, token_index: usize, embedding: &mut [f32]) {
        for chunk_index in 0..HASH_CHUNK_COUNT {
            let mut hasher = Sha256::new();
            hasher.update(token.as_bytes());
            hasher.update((token_index as u64).to_le_bytes());
            hasher.update((chunk_index as u64).to_le_bytes());
            let digest = hasher.finalize();
            let start = chunk_index * 32;

            for (offset, byte) in digest.iter().enumerate() {
                let centered = (*byte as f32 / 127.5) - 1.0;
                embedding[start + offset] += centered;
            }
        }
    }
}

/// Lazily initialises the process-global embedding model.
pub fn ensure_model() -> &'static EmbeddingModel {
    MODEL.get_or_init(EmbeddingModel::new)
}

/// Returns a deterministic, L2-normalized 384-dimensional embedding.
///
/// **PLACEHOLDER:** In the current build this returns a SHA-256 hash
/// projection, **not** a semantic BGE-small-en-v1.5 embedding. See the
/// module-level doc for the full placeholder contract.
pub fn embed(text: &str) -> Result<Vec<f32>, InferenceError> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(InferenceError::EmptyInput);
    }

    ensure_model().embed(trimmed)
}

/// Searches the active vector table and returns page-ranked matches.
pub fn search_vec(
    query: &str,
    k: usize,
    wing_filter: Option<&str>,
    conn: &Connection,
) -> Result<Vec<SearchResult>, SearchError> {
    if query.trim().is_empty() || k == 0 {
        return Ok(Vec::new());
    }

    let (model_name, vec_table) = active_model(conn)?;

    let embedding_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM page_embeddings WHERE model = ?1",
        [&model_name],
        |row| row.get(0),
    )?;
    if embedding_count == 0 {
        return Ok(Vec::new());
    }

    if !is_safe_identifier(&vec_table) {
        return Err(SearchError::Internal {
            message: format!("unsafe vec table name: {vec_table}"),
        });
    }

    let query_embedding = embed(query).map_err(|err| SearchError::Internal {
        message: err.to_string(),
    })?;
    let query_blob = embedding_to_blob(&query_embedding);

    let mut sql = format!(
        "SELECT p.slug, p.title, p.summary, \
                MAX(1.0 - vec_distance_cosine(pev.embedding, ?1)) AS score, \
                p.wing \
         FROM {vec_table} pev \
         JOIN page_embeddings pe ON pev.rowid = pe.vec_rowid \
         JOIN pages p ON p.id = pe.page_id \
         WHERE pe.model = ?2"
    );

    let mut params: Vec<Box<dyn ToSql>> = vec![Box::new(query_blob), Box::new(model_name)];

    if let Some(wing) = wing_filter {
        sql.push_str(" AND p.wing = ?3");
        params.push(Box::new(wing.to_owned()));
    }

    let limit_index = params.len() + 1;
    sql.push_str(" GROUP BY p.id ORDER BY score DESC LIMIT ?");
    sql.push_str(&limit_index.to_string());
    params.push(Box::new(k as i64));

    let param_refs: Vec<&dyn ToSql> = params.iter().map(|param| param.as_ref()).collect();
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

fn active_model(conn: &Connection) -> Result<(String, String), SearchError> {
    conn.query_row(
        "SELECT name, vec_table FROM embedding_models WHERE active = 1 LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )
    .map_err(|err| match err {
        rusqlite::Error::QueryReturnedNoRows => SearchError::Internal {
            message: "no active embedding model configured".to_owned(),
        },
        other => SearchError::from(other),
    })
}

pub(crate) fn embedding_to_blob(embedding: &[f32]) -> Vec<u8> {
    let mut blob = Vec::with_capacity(std::mem::size_of_val(embedding));
    for value in embedding {
        blob.extend_from_slice(&value.to_le_bytes());
    }
    blob
}

fn is_safe_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

fn normalize(values: &mut [f32]) -> Result<(), InferenceError> {
    let norm = values.iter().map(|value| value * value).sum::<f32>().sqrt();
    if norm == 0.0 {
        return Err(InferenceError::Internal {
            message: "embedding norm is zero".to_owned(),
        });
    }

    for value in values {
        *value /= norm;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::db;

    fn open_test_db() -> Connection {
        let dir = tempfile::TempDir::new().expect("create temp dir");
        let db_path = dir.path().join("test_brain.db");
        let conn = db::open(db_path.to_str().expect("utf8 path")).expect("open db");
        std::mem::forget(dir);
        conn
    }

    #[test]
    fn embed_returns_normalized_vector_of_expected_length() {
        let embedding = embed("Alice works at Acme Corp").expect("embed text");
        let norm = embedding
            .iter()
            .map(|value| value * value)
            .sum::<f32>()
            .sqrt();

        assert_eq!(embedding.len(), EMBEDDING_DIMENSIONS);
        assert!((norm - 1.0).abs() < 1e-5, "unexpected norm: {norm}");
    }

    #[test]
    fn embed_returns_error_for_empty_input() {
        let err = embed("   ").expect_err("empty input should fail");
        assert!(matches!(err, InferenceError::EmptyInput));
    }

    #[test]
    fn search_vec_on_empty_db_returns_empty_vec() {
        let conn = open_test_db();
        let results = search_vec("board member tech company", 5, None, &conn)
            .expect("empty db search should succeed");

        assert!(results.is_empty());
    }

    #[test]
    fn search_vec_returns_ranked_results_from_vec_table() {
        let conn = open_test_db();
        conn.execute(
            "INSERT INTO pages (slug, type, title, summary, compiled_truth, timeline, frontmatter, wing, room, version) \
             VALUES (?1, 'person', ?2, ?3, '', '', '{}', ?4, '', 1)",
            rusqlite::params!["people/alice", "Alice", "Founder", "people"],
        )
        .expect("insert page");

        let page_id: i64 = conn
            .query_row(
                "SELECT id FROM pages WHERE slug = 'people/alice'",
                [],
                |row| row.get(0),
            )
            .expect("fetch page id");

        let query_embedding = embed("startup founder").expect("embed query");
        let query_blob = embedding_to_blob(&query_embedding);
        conn.execute(
            "INSERT INTO page_embeddings_vec_384(rowid, embedding) VALUES (?1, ?2)",
            rusqlite::params![1_i64, query_blob],
        )
        .expect("insert vec row");
        conn.execute(
            "INSERT INTO page_embeddings (page_id, model, vec_rowid, chunk_type, chunk_index, chunk_text, content_hash, token_count, heading_path) \
             VALUES (?1, 'bge-small-en-v1.5', 1, 'truth_section', 0, 'startup founder', 'hash', 2, 'State')",
            rusqlite::params![page_id],
        )
        .expect("insert embedding metadata");

        let results = search_vec("startup founder", 5, Some("people"), &conn)
            .expect("vector search should succeed");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].slug, "people/alice");
        assert!(
            results[0].score > 0.99,
            "unexpected score: {}",
            results[0].score
        );
    }
}
