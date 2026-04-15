use anyhow::Result;
use rusqlite::Connection;
use serde::Serialize;

use crate::core::gaps::list_gaps;

pub fn run(db: &Connection, limit: u32, resolved: bool, json: bool) -> Result<()> {
    let gaps = list_gaps(resolved, limit as usize, db)?;

    if json {
        #[derive(Serialize)]
        struct GapEntry {
            id: i64,
            query_hash: String,
            context: String,
            confidence_score: Option<f64>,
            sensitivity: String,
            resolved_at: Option<String>,
            detected_at: String,
        }

        let entries: Vec<GapEntry> = gaps
            .into_iter()
            .map(|g| GapEntry {
                id: g.id,
                query_hash: g.query_hash,
                context: g.context,
                confidence_score: g.confidence_score,
                sensitivity: g.sensitivity,
                resolved_at: g.resolved_at,
                detected_at: g.detected_at,
            })
            .collect();

        println!("{}", serde_json::to_string_pretty(&entries)?);
    } else if gaps.is_empty() {
        println!("No knowledge gaps found.");
    } else {
        for gap in &gaps {
            let status = if gap.resolved_at.is_some() {
                "resolved"
            } else {
                "unresolved"
            };
            println!(
                "[{}] {} (confidence: {}, {})",
                gap.id,
                gap.query_hash,
                gap.confidence_score
                    .map(|s| format!("{s:.2}"))
                    .unwrap_or_else(|| "n/a".to_string()),
                status
            );
        }
        println!("{} gap(s) found.", gaps.len());
    }

    Ok(())
}
