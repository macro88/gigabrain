use rusqlite::{params, Connection, OptionalExtension};
use thiserror::Error;

use crate::core::collections::{self, OpKind, SlugResolution};

#[derive(Debug, Error)]
pub enum SupersedeError {
    #[error("page not found: {slug}")]
    NotFound { slug: String },

    #[error("ambiguous slug: {slug} ({candidates})")]
    Ambiguous { slug: String, candidates: String },

    #[error("SupersedeConflictError: page `{slug}` is already superseded by `{successor_slug}`")]
    NonHeadTarget {
        slug: String,
        successor_slug: String,
    },

    #[error("SupersedeConflictError: page `{slug}` cannot supersede itself")]
    SelfReference { slug: String },

    #[error("SupersedeConflictError: supersede target `{slug}` must stay in the same collection")]
    CrossCollection { slug: String },

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("{0}")]
    Collection(#[from] collections::CollectionError),
}

#[derive(Debug, Clone)]
struct SupersedeTarget {
    id: i64,
    canonical_slug: String,
}

pub fn successor_slug_by_id(
    conn: &Connection,
    successor_id: Option<i64>,
) -> Result<Option<String>, rusqlite::Error> {
    successor_id
        .map(|page_id| canonical_slug_by_page_id(conn, page_id))
        .transpose()
}

pub fn predecessor_slug_by_successor_id(
    conn: &Connection,
    collection_id: i64,
    namespace: &str,
    successor_id: i64,
) -> Result<Option<String>, rusqlite::Error> {
    conn.query_row(
        "SELECT slug
         FROM pages
         WHERE collection_id = ?1 AND namespace = ?2 AND superseded_by = ?3
         LIMIT 1",
        params![collection_id, namespace, successor_id],
        |row| row.get(0),
    )
    .optional()
}

pub fn reconcile_supersede_chain(
    conn: &Connection,
    collection_id: i64,
    namespace: &str,
    page_id: i64,
    page_slug: &str,
    supersedes: Option<&str>,
) -> Result<(), SupersedeError> {
    let desired_target = supersedes
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| resolve_supersede_target(conn, collection_id, namespace, value))
        .transpose()?;

    if let Some(target) = desired_target.as_ref() {
        if target.id == page_id {
            return Err(SupersedeError::SelfReference {
                slug: page_slug.to_owned(),
            });
        }

        let current_successor: Option<i64> = conn
            .query_row(
                "SELECT superseded_by FROM pages WHERE id = ?1",
                [target.id],
                |row| row.get(0),
            )
            .optional()?
            .flatten();
        if let Some(successor_id) = current_successor {
            if successor_id != page_id {
                return Err(SupersedeError::NonHeadTarget {
                    slug: target.canonical_slug.clone(),
                    successor_slug: canonical_slug_by_page_id(conn, successor_id)?,
                });
            }
        }
    }

    let existing_predecessor_id: Option<i64> = conn
        .query_row(
            "SELECT id
             FROM pages
             WHERE collection_id = ?1 AND namespace = ?2 AND superseded_by = ?3
             LIMIT 1",
            params![collection_id, namespace, page_id],
            |row| row.get(0),
        )
        .optional()?;

    let desired_predecessor_id = desired_target.as_ref().map(|target| target.id);
    if existing_predecessor_id == desired_predecessor_id {
        return Ok(());
    }

    if let Some(existing_id) = existing_predecessor_id {
        conn.execute(
            "UPDATE pages
             SET superseded_by = NULL
             WHERE id = ?1 AND superseded_by = ?2",
            params![existing_id, page_id],
        )?;
    }

    if let Some(target) = desired_target {
        let updated = conn.execute(
            "UPDATE pages
             SET superseded_by = ?1
             WHERE id = ?2 AND superseded_by IS NULL",
            params![page_id, target.id],
        )?;
        if updated == 0 {
            let successor_id: Option<i64> = conn
                .query_row(
                    "SELECT superseded_by FROM pages WHERE id = ?1",
                    [target.id],
                    |row| row.get(0),
                )
                .optional()?
                .flatten();
            return Err(SupersedeError::NonHeadTarget {
                slug: target.canonical_slug,
                successor_slug: successor_slug_by_id(conn, successor_id)?
                    .unwrap_or_else(|| page_slug.to_owned()),
            });
        }
    }

    Ok(())
}

fn resolve_supersede_target(
    conn: &Connection,
    collection_id: i64,
    namespace: &str,
    raw_slug: &str,
) -> Result<SupersedeTarget, SupersedeError> {
    let normalized_slug = if raw_slug.contains("::") {
        match collections::parse_slug(conn, raw_slug, OpKind::Read)? {
            SlugResolution::Resolved {
                collection_id: resolved_collection_id,
                slug,
                ..
            } => {
                if resolved_collection_id != collection_id {
                    return Err(SupersedeError::CrossCollection {
                        slug: raw_slug.to_owned(),
                    });
                }
                slug
            }
            SlugResolution::NotFound { slug } => return Err(SupersedeError::NotFound { slug }),
            SlugResolution::Ambiguous { slug, candidates } => {
                return Err(SupersedeError::Ambiguous {
                    slug,
                    candidates: candidates
                        .into_iter()
                        .map(|candidate| candidate.full_address)
                        .collect::<Vec<_>>()
                        .join(", "),
                })
            }
        }
    } else {
        raw_slug.to_owned()
    };

    let target_id: i64 = conn
        .query_row(
            "SELECT id
             FROM pages
             WHERE collection_id = ?1 AND namespace = ?2 AND slug = ?3",
            params![collection_id, namespace, normalized_slug],
            |row| row.get(0),
        )
        .optional()?
        .ok_or_else(|| SupersedeError::NotFound {
            slug: raw_slug.to_owned(),
        })?;

    Ok(SupersedeTarget {
        id: target_id,
        canonical_slug: canonical_slug_by_page_id(conn, target_id)?,
    })
}

fn canonical_slug_by_page_id(conn: &Connection, page_id: i64) -> Result<String, rusqlite::Error> {
    conn.query_row(
        "SELECT c.name || '::' || p.slug
         FROM pages p
         JOIN collections c ON c.id = p.collection_id
         WHERE p.id = ?1",
        [page_id],
        |row| row.get(0),
    )
}
