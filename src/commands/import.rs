use std::path::Path;

use anyhow::Result;
use rusqlite::Connection;

use crate::core::migrate;

pub fn run(db: &Connection, path: &str, validate_only: bool) -> Result<()> {
    let dir = Path::new(path);
    let stats = migrate::import_dir(db, dir, validate_only)?;

    if validate_only {
        println!("Validation passed: {} file(s) OK", stats.imported);
    } else {
        let total_skipped = stats.total_skipped();
        if total_skipped == 0 {
            println!("Imported {} page(s)", stats.imported);
        } else {
            let mut reasons = Vec::new();
            if stats.skipped_already_ingested > 0 {
                reasons.push(format!(
                    "{} already ingested",
                    stats.skipped_already_ingested
                ));
            }
            if stats.skipped_non_markdown > 0 {
                reasons.push(format!("{} non-markdown", stats.skipped_non_markdown));
            }
            println!(
                "Imported {} page(s) ({} skipped: {})",
                stats.imported,
                total_skipped,
                reasons.join(", ")
            );
        }
    }

    Ok(())
}
