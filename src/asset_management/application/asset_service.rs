// Asset service - application use cases

use crate::asset_management::domain::asset::{Asset, AssetRepository};
use crate::asset_management::domain::shared::{AssetError, AssetId, CategoryId};

/// Asset service - orchestrates asset-related use cases
pub struct AssetService<R: AssetRepository> {
    repository: R,
}

impl<R: AssetRepository> AssetService<R> {
    /// Create a new AssetService
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Create a new asset (minimal version for Walking Skeleton)
    pub fn create_asset(
        &self,
        name: String,
        manufacturer: Option<String>,
        quantity: Option<i32>,
        memo: Option<String>,
        _category_ids: Vec<CategoryId>, // To be used in later slices
    ) -> Result<AssetId, AssetError> {
        // Create asset entity
        let mut asset = if manufacturer.is_some() || quantity.is_some() || memo.is_some() {
            Asset::with_details(name, manufacturer, quantity.unwrap_or(1), memo)?
        } else {
            Asset::new(name)?
        };

        // Save to repository
        self.repository.save(&asset)?;

        // Get the ID of the saved asset
        // Since SQLite auto-increments, we need to query for the last inserted row
        // For now, we'll retrieve by name (this is a simplification for Walking Skeleton)
        let assets = self.repository.find_all()?;
        if let Some(saved_asset) = assets.last()
            && let Some(id) = saved_asset.id
        {
            asset.id = Some(id);
            return Ok(id);
        }

        Err(AssetError::DatabaseError(
            "Failed to retrieve asset ID after save".to_string(),
        ))
    }

    /// Get asset by ID
    #[allow(dead_code)] // Used in later slices
    pub fn get_asset(&self, id: AssetId) -> Result<Asset, AssetError> {
        self.repository.find(id)
    }

    /// List all assets
    pub fn list_assets(&self) -> Result<Vec<Asset>, AssetError> {
        self.repository.find_all()
    }

    /// Update asset
    #[allow(dead_code)] // Used in later slices
    pub fn update_asset(&self, asset: &Asset) -> Result<(), AssetError> {
        // Verify asset exists
        if let Some(id) = asset.id {
            let _ = self.repository.find(id)?;
        } else {
            return Err(AssetError::DatabaseError(
                "Cannot update asset without ID".to_string(),
            ));
        }

        self.repository.save(asset)
    }

    /// Delete asset
    #[allow(dead_code)] // Used in later slices
    pub fn delete_asset(&self, id: AssetId) -> Result<(), AssetError> {
        // Verify asset exists
        let _ = self.repository.find(id)?;

        self.repository.delete(id)
    }
}
