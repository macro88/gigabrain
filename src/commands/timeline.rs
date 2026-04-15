use anyhow::Result;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct TimelineOutput {
    slug: String,
    entries: Vec<String>,
}

/// Show timeline entries for a page from the `timeline_entries` table,
/// with legacy fallback to the page's `timeline` markdown field.
pub fn run(db: &Connection, slug: &str, limit: u32, json: bool) -> Result<()> {
    // Verify page exists
    let page = crate::commands::get::get_page(db, slug)?;

    let page_id: i64 = db
        .query_row("SELECT id FROM pages WHERE slug = ?1", [slug], |row| {
            row.get(0)
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => anyhow::anyhow!("page not found: {slug}"),
            other => other.into(),
        })?;

    // Query structured timeline_entries table
    let mut stmt = db.prepare(
        "SELECT date, summary, source, detail FROM timeline_entries \
         WHERE page_id = ?1 ORDER BY date DESC LIMIT ?2",
    )?;
    let rows = stmt.query_map(rusqlite::params![page_id, limit], |row| {
        let date: String = row.get(0)?;
        let summary: String = row.get(1)?;
        let source: String = row.get(2)?;
        let detail: String = row.get(3)?;
        let mut entry = format!("{date}: {summary}");
        if !source.is_empty() {
            entry.push_str(&format!(" [source: {source}]"));
        }
        if !detail.is_empty() {
            entry.push_str(&format!("\n{detail}"));
        }
        Ok(entry)
    })?;

    let mut entries: Vec<String> = Vec::new();
    for row in rows {
        entries.push(row?);
    }

    // Fall back to legacy timeline markdown field if no structured entries exist
    if entries.is_empty() {
        let timeline = page.timeline.trim();
        if !timeline.is_empty() {
            entries = split_timeline(timeline)
                .into_iter()
                .take(limit as usize)
                .collect();
        }
    }

    if entries.is_empty() {
        if json {
            let output = TimelineOutput {
                slug: slug.to_string(),
                entries: Vec::new(),
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            println!("No timeline entries for {slug}");
        }
        return Ok(());
    }

    if json {
        let output = TimelineOutput {
            slug: slug.to_string(),
            entries,
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        for entry in &entries {
            println!("{entry}");
            println!();
        }
    }

    Ok(())
}

pub fn add(
    db: &Connection,
    slug: &str,
    date: &str,
    summary: &str,
    source: Option<String>,
    detail: Option<String>,
) -> Result<()> {
    let page_id: i64 = db
        .query_row("SELECT id FROM pages WHERE slug = ?1", [slug], |row| {
            row.get(0)
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => anyhow::anyhow!("page not found: {slug}"),
            other => other.into(),
        })?;

    let summary_hash = {
        use sha2::{Digest, Sha256};
        let digest = Sha256::digest(summary.as_bytes());
        format!("{digest:x}")
    };

    db.execute(
        "INSERT INTO timeline_entries (page_id, date, source, summary, summary_hash, detail) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            page_id,
            date,
            source.as_deref().unwrap_or(""),
            summary,
            summary_hash,
            detail.as_deref().unwrap_or(""),
        ],
    )?;

    println!("Added timeline entry for {slug}");
    Ok(())
}

/// Split timeline markdown into individual entries separated by `---` lines.
fn split_timeline(timeline: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let mut current = Vec::new();

    for line in timeline.lines() {
        if line.trim() == "---" {
            if !current.is_empty() {
                entries.push(current.join("\n"));
                current.clear();
            }
        } else {
            current.push(line.to_string());
        }
    }

    if !current.is_empty() {
        entries.push(current.join("\n"));
    }

    entries
        .into_iter()
        .filter(|e| !e.trim().is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_timeline_separates_on_bare_boundary() {
        let entries = split_timeline("entry one\n---\nentry two");
        assert_eq!(entries, vec!["entry one", "entry two"]);
    }

    #[test]
    fn split_timeline_single_entry_no_boundary() {
        let entries = split_timeline("just one entry");
        assert_eq!(entries, vec!["just one entry"]);
    }

    #[test]
    fn split_timeline_empty_returns_empty() {
        let entries = split_timeline("");
        assert!(entries.is_empty());
    }
}
