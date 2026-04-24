use anyhow::Result;
use rusqlite::Connection;

pub async fn run(db: Connection) -> Result<()> {
    let db_path = crate::core::vault_sync::database_path(&db)?;
    // start_serve_runtime is cross-platform; watcher threads are #[cfg(unix)]-gated internally.
    let _runtime = crate::core::vault_sync::start_serve_runtime(db_path)?;
    crate::mcp::server::run(db).await
}
