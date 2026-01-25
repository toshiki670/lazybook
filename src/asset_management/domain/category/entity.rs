// Category entity - to be implemented later

use crate::asset_management::domain::shared::CategoryId;
use chrono::NaiveDateTime;

/// Category entity (placeholder)
#[derive(Debug, Clone)]
#[allow(dead_code)] // Will be used in later slices
pub struct Category {
    pub id: Option<CategoryId>,
    pub name: String,
    pub parent_id: Option<CategoryId>,
    pub display_order: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
