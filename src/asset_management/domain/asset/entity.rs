// Asset entity

use crate::asset_management::domain::shared::{AssetError, AssetId};
use chrono::NaiveDateTime;

/// Asset aggregate root
#[derive(Debug, Clone)]
pub struct Asset {
    pub id: Option<AssetId>,
    pub name: String,
    pub manufacturer: Option<String>,
    pub quantity: i32,
    pub memo: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Asset {
    /// Create a new Asset with name only (minimal version for Walking Skeleton)
    pub fn new(name: String) -> Result<Self, AssetError> {
        Self::validate_name(&name)?;

        let now = chrono::Local::now().naive_local();

        Ok(Self {
            id: None,
            name,
            manufacturer: None,
            quantity: 1,
            memo: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Create Asset with full fields (will be used later)
    pub fn with_details(
        name: String,
        manufacturer: Option<String>,
        quantity: i32,
        memo: Option<String>,
    ) -> Result<Self, AssetError> {
        Self::validate_name(&name)?;

        if let Some(ref m) = manufacturer {
            Self::validate_manufacturer(m)?;
        }

        Self::validate_quantity(quantity)?;

        if let Some(ref m) = memo {
            Self::validate_memo(m)?;
        }

        let now = chrono::Local::now().naive_local();

        Ok(Self {
            id: None,
            name,
            manufacturer,
            quantity,
            memo,
            created_at: now,
            updated_at: now,
        })
    }

    /// Validate asset name
    fn validate_name(name: &str) -> Result<(), AssetError> {
        if name.trim().is_empty() {
            return Err(AssetError::NameRequired);
        }
        if name.len() > 255 {
            return Err(AssetError::NameTooLong(name.len()));
        }
        Ok(())
    }

    /// Validate manufacturer name
    fn validate_manufacturer(manufacturer: &str) -> Result<(), AssetError> {
        if manufacturer.len() > 255 {
            return Err(AssetError::ManufacturerTooLong(manufacturer.len()));
        }
        Ok(())
    }

    /// Validate quantity
    fn validate_quantity(quantity: i32) -> Result<(), AssetError> {
        if quantity < 0 {
            return Err(AssetError::InvalidQuantity(quantity));
        }
        Ok(())
    }

    /// Validate memo
    fn validate_memo(memo: &str) -> Result<(), AssetError> {
        if memo.len() > 10000 {
            return Err(AssetError::MemoTooLong(memo.len()));
        }
        Ok(())
    }

    /// Update name
    #[allow(dead_code)] // Used in later slices
    pub fn update_name(&mut self, name: String) -> Result<(), AssetError> {
        Self::validate_name(&name)?;
        self.name = name;
        self.updated_at = chrono::Local::now().naive_local();
        Ok(())
    }

    /// Update quantity
    #[allow(dead_code)] // Used in later slices
    pub fn update_quantity(&mut self, quantity: i32) -> Result<(), AssetError> {
        Self::validate_quantity(quantity)?;
        self.quantity = quantity;
        self.updated_at = chrono::Local::now().naive_local();
        Ok(())
    }
}
