// Asset repository trait

use crate::asset_management::domain::asset::entity::Asset;
use crate::asset_management::domain::shared::{AssetError, AssetId, CategoryId};

/// Repository interface for Asset aggregate
#[allow(dead_code)] // Some methods used only in later slices
pub trait AssetRepository {
    /// Find asset by ID
    fn find(&self, id: AssetId) -> Result<Asset, AssetError>;

    /// Find all assets
    fn find_all(&self) -> Result<Vec<Asset>, AssetError>;

    /// Find assets by category (optionally including descendants)
    fn find_by_category(
        &self,
        category_id: CategoryId,
        include_descendants: bool,
    ) -> Result<Vec<Asset>, AssetError>;

    /// Find assets by manufacturer
    fn find_by_manufacturer(&self, manufacturer: &str) -> Result<Vec<Asset>, AssetError>;

    /// Save asset (insert or update)
    fn save(&self, asset: &Asset) -> Result<(), AssetError>;

    /// Delete asset by ID
    fn delete(&self, id: AssetId) -> Result<(), AssetError>;
}
