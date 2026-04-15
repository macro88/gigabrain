use anyhow::{bail, Result};
use rusqlite::Connection;

use crate::core::graph::{self, GraphError, TemporalFilter};

/// Run the `gbrain graph` command.
///
/// Calls `neighborhood_graph` and prints the result as either human-readable
/// text or JSON, depending on the `json` flag.
pub fn run(db: &Connection, slug: &str, depth: u32, temporal: &str, json: bool) -> Result<()> {
    let filter = match temporal.to_lowercase().as_str() {
        "all" | "history" => TemporalFilter::All,
        _ => TemporalFilter::Active,
    };

    let result = match graph::neighborhood_graph(slug, depth, filter, db) {
        Ok(r) => r,
        Err(GraphError::PageNotFound { slug }) => {
            bail!("page not found: {slug}");
        }
        Err(GraphError::Sqlite(e)) => {
            return Err(e.into());
        }
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{slug}");
        for edge in &result.edges {
            println!("  → {} ({})", edge.to, edge.relationship);
        }
    }

    Ok(())
}
