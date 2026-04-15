use anyhow::Result;
use rusqlite::Connection;

use crate::core::fts::search_fts;

pub fn run(
    db: &Connection,
    query: &str,
    wing: Option<String>,
    limit: u32,
    json: bool,
) -> Result<()> {
    let results = search_fts(query, wing.as_deref(), db, limit as usize)?;
    let results: Vec<_> = results.into_iter().take(limit as usize).collect();

    if json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    } else if results.is_empty() {
        println!("No results found.");
    } else {
        for r in &results {
            println!("{}: {}", r.slug, r.summary);
        }
    }

    Ok(())
}
