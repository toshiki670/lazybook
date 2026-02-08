// Category repository trait

use crate::asset_management::domain::category::entity::Category;
use crate::asset_management::domain::shared::types::{CategoryError, CategoryId};

/// Repository interface for Category aggregate
#[allow(dead_code)] // Will be used in later slices
pub trait CategoryRepository {
    /// Find category by ID
    fn find(&self, id: CategoryId) -> Result<Category, CategoryError>;

    /// Find all categories
    fn find_all(&self) -> Result<Vec<Category>, CategoryError>;

    /// Find root categories (parent_id is NULL)
    fn find_roots(&self) -> Result<Vec<Category>, CategoryError>;

    /// Find direct children of a category
    fn find_children(&self, parent_id: CategoryId) -> Result<Vec<Category>, CategoryError>;

    /// Find all ancestors of a category (from root to parent)
    fn find_ancestors(&self, id: CategoryId) -> Result<Vec<Category>, CategoryError>;

    /// Find all descendants of a category (children, grandchildren, etc.)
    fn find_descendants(&self, id: CategoryId) -> Result<Vec<Category>, CategoryError>;

    /// Save category (insert or update)
    fn save(&self, category: &Category) -> Result<(), CategoryError>;

    /// Delete category by ID
    fn delete(&self, id: CategoryId) -> Result<(), CategoryError>;

    /// Check if category has children
    fn has_children(&self, id: CategoryId) -> Result<bool, CategoryError>;

    /// Check if category has assets
    fn has_assets(&self, id: CategoryId) -> Result<bool, CategoryError>;
}
