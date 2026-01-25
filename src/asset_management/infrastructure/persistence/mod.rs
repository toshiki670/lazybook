// Persistence infrastructure

mod asset_repository;
mod category_repository;
pub mod migrations;

pub use asset_repository::SqliteAssetRepository;
// Note: SqliteCategoryRepository not used yet - will be needed in later slices
// pub use category_repository::SqliteCategoryRepository;

use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

/// Establish database connection
pub fn establish_connection(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    Ok(conn)
}

/// Initialize database with migrations
pub fn initialize_database(conn: &Connection) -> Result<()> {
    migrations::apply_migrations(conn)?;
    Ok(())
}
