// SQLite asset repository

use crate::asset_management::domain::asset::{Asset, AssetRepository};
use crate::asset_management::domain::shared::{AssetError, AssetId, CategoryId};
use chrono::NaiveDateTime;
use rusqlite::{params, Connection, Result as SqliteResult, Row};
use std::sync::{Arc, Mutex};

/// SQLite implementation of AssetRepository
pub struct SqliteAssetRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteAssetRepository {
    /// Create a new repository with a database connection
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    /// Map database row to Asset entity
    fn row_to_asset(row: &Row) -> SqliteResult<Asset> {
        let id: i64 = row.get(0)?;
        let name: String = row.get(1)?;
        let manufacturer: Option<String> = row.get(2)?;
        let quantity: i32 = row.get(3)?;
        let memo: Option<String> = row.get(4)?;
        let created_at: String = row.get(5)?;
        let updated_at: String = row.get(6)?;

        Ok(Asset {
            id: Some(AssetId::new(id)),
            name,
            manufacturer,
            quantity,
            memo,
            created_at: NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| chrono::Local::now().naive_local()),
            updated_at: NaiveDateTime::parse_from_str(&updated_at, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| chrono::Local::now().naive_local()),
        })
    }
}

impl AssetRepository for SqliteAssetRepository {
    fn find(&self, id: AssetId) -> Result<Asset, AssetError> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn
            .prepare("SELECT id, name, manufacturer, quantity, memo, created_at, updated_at FROM assets WHERE id = ?")
            .map_err(|e| AssetError::DatabaseError(e.to_string()))?;

        let asset = stmt
            .query_row(params![id.value()], Self::row_to_asset)
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => AssetError::NotFound(id.value()),
                _ => AssetError::DatabaseError(e.to_string()),
            })?;

        Ok(asset)
    }

    fn find_all(&self) -> Result<Vec<Asset>, AssetError> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn
            .prepare("SELECT id, name, manufacturer, quantity, memo, created_at, updated_at FROM assets ORDER BY id")
            .map_err(|e| AssetError::DatabaseError(e.to_string()))?;

        let assets = stmt
            .query_map([], Self::row_to_asset)
            .map_err(|e| AssetError::DatabaseError(e.to_string()))?
            .collect::<SqliteResult<Vec<Asset>>>()
            .map_err(|e| AssetError::DatabaseError(e.to_string()))?;

        Ok(assets)
    }

    fn find_by_category(
        &self,
        _category_id: CategoryId,
        _include_descendants: bool,
    ) -> Result<Vec<Asset>, AssetError> {
        // To be implemented in later slices
        Ok(Vec::new())
    }

    fn find_by_manufacturer(&self, manufacturer: &str) -> Result<Vec<Asset>, AssetError> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn
            .prepare("SELECT id, name, manufacturer, quantity, memo, created_at, updated_at FROM assets WHERE manufacturer = ? ORDER BY id")
            .map_err(|e| AssetError::DatabaseError(e.to_string()))?;

        let assets = stmt
            .query_map(params![manufacturer], Self::row_to_asset)
            .map_err(|e| AssetError::DatabaseError(e.to_string()))?
            .collect::<SqliteResult<Vec<Asset>>>()
            .map_err(|e| AssetError::DatabaseError(e.to_string()))?;

        Ok(assets)
    }

    fn save(&self, asset: &Asset) -> Result<(), AssetError> {
        let conn = self.conn.lock().unwrap();

        let created_at_str = asset.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
        let updated_at_str = asset.updated_at.format("%Y-%m-%d %H:%M:%S").to_string();

        if let Some(id) = asset.id {
            // Update existing asset
            conn.execute(
                "UPDATE assets SET name = ?, manufacturer = ?, quantity = ?, memo = ?, updated_at = ? WHERE id = ?",
                params![
                    &asset.name,
                    &asset.manufacturer,
                    asset.quantity,
                    &asset.memo,
                    updated_at_str,
                    id.value()
                ],
            )
            .map_err(|e| AssetError::DatabaseError(e.to_string()))?;
        } else {
            // Insert new asset
            conn.execute(
                "INSERT INTO assets (name, manufacturer, quantity, memo, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    &asset.name,
                    &asset.manufacturer,
                    asset.quantity,
                    &asset.memo,
                    created_at_str,
                    updated_at_str
                ],
            )
            .map_err(|e| AssetError::DatabaseError(e.to_string()))?;
        }

        Ok(())
    }

    fn delete(&self, id: AssetId) -> Result<(), AssetError> {
        let conn = self.conn.lock().unwrap();

        conn.execute("DELETE FROM assets WHERE id = ?", params![id.value()])
            .map_err(|e| AssetError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
