// Database migrations

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::fs;
use std::path::Path;

const MIGRATIONS_DIR: &str = "migrations";

/// Apply all pending migrations
pub fn apply_migrations(conn: &Connection) -> Result<()> {
    // Create schema_version table if it doesn't exist
    create_schema_version_table(conn)?;

    // Get current schema version
    let current_version = get_current_version(conn)?;
    log::info!("Current schema version: {}", current_version);

    // Find all migration files
    let migrations = find_migration_files()?;
    log::info!("Found {} migration files", migrations.len());

    // Apply pending migrations
    let mut applied_count = 0;
    for (version, filepath) in migrations {
        if version > current_version {
            log::info!("Applying migration {}: {:?}", version, filepath);
            apply_migration(conn, version, &filepath)?;
            applied_count += 1;
        }
    }

    if applied_count > 0 {
        log::info!("Applied {} migrations", applied_count);
    } else {
        log::info!("No pending migrations");
    }

    Ok(())
}

/// Create schema_version table if it doesn't exist
fn create_schema_version_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL
        )",
        [],
    )
    .context("Failed to create schema_version table")?;
    Ok(())
}

/// Get current schema version
fn get_current_version(conn: &Connection) -> Result<i32> {
    let version: Result<Option<i32>, rusqlite::Error> =
        conn.query_row("SELECT MAX(version) FROM schema_version", [], |row| {
            row.get(0)
        });

    match version {
        Ok(Some(v)) => Ok(v),
        Ok(None) => Ok(0), // No migrations applied yet
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
        Err(e) => Err(e).context("Failed to get current schema version"),
    }
}

/// Find all migration SQL files in migrations/ directory
fn find_migration_files() -> Result<Vec<(i32, std::path::PathBuf)>> {
    let migrations_path = Path::new(MIGRATIONS_DIR);

    if !migrations_path.exists() {
        log::warn!("Migrations directory not found: {}", MIGRATIONS_DIR);
        return Ok(Vec::new());
    }

    let mut migrations = Vec::new();

    for entry in fs::read_dir(migrations_path)
        .context(format!("Failed to read directory: {}", MIGRATIONS_DIR))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("sql") {
            // Extract version number from filename (e.g., "001_initial_schema.sql" -> 1)
            if let Some(filename) = path.file_name().and_then(|s| s.to_str())
                && let Some(version_str) = filename.split('_').next()
                && let Ok(version) = version_str.parse::<i32>()
            {
                migrations.push((version, path));
            }
        }
    }

    // Sort by version number
    migrations.sort_by_key(|(version, _)| *version);

    Ok(migrations)
}

/// Apply a single migration file
fn apply_migration(conn: &Connection, version: i32, filepath: &Path) -> Result<()> {
    // Read SQL file
    let sql = fs::read_to_string(filepath)
        .context(format!("Failed to read migration file: {:?}", filepath))?;

    // Execute SQL (this handles multiple statements)
    conn.execute_batch(&sql)
        .context(format!("Failed to execute migration {}", version))?;

    log::info!("Migration {} applied successfully", version);

    Ok(())
}

/// Check current schema version (utility function for debugging)
#[allow(dead_code)] // Utility function for debugging
pub fn check_schema_version(conn: &Connection) -> Result<i32> {
    get_current_version(conn)
}
