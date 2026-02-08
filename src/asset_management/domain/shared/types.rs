// Shared domain types

use chrono::NaiveDate;
use thiserror::Error;

// ============================================================================
// Error Types
// ============================================================================

/// Asset domain errors
#[derive(Error, Debug)]
pub enum AssetError {
    #[error("資産名は必須です")]
    NameRequired,

    #[error("資産名は255文字以内でなければなりません: {0}")]
    NameTooLong(usize),

    #[error("メーカー名は255文字以内でなければなりません: {0}")]
    ManufacturerTooLong(usize),

    #[error("数量は0以上でなければなりません: {0}")]
    InvalidQuantity(i32),

    #[error("メモは10,000文字以内でなければなりません: {0}")]
    MemoTooLong(usize),

    #[error("資産が見つかりません: {0}")]
    #[allow(dead_code)] // Used in later slices
    NotFound(i64),

    #[error("カテゴリが必要です: 資産は少なくとも1つのカテゴリに属する必要があります")]
    #[allow(dead_code)] // Used in later slices
    CategoryRequired,

    #[error("データベースエラー: {0}")]
    DatabaseError(String),

    #[error("価格エラー: {0}")]
    PriceError(#[from] PriceError),
}

/// Category domain errors
#[derive(Error, Debug)]
#[allow(dead_code)] // Will be used in later slices
pub enum CategoryError {
    #[error("カテゴリ名は必須です")]
    NameRequired,

    #[error("カテゴリ名は100文字以内でなければなりません: {0}")]
    NameTooLong(usize),

    #[error("カテゴリ名が重複しています: {0}")]
    DuplicateName(String),

    #[error("カテゴリが見つかりません: {0}")]
    NotFound(i64),

    #[error("循環参照エラー: カテゴリ {0} は自分自身または子孫を親にできません")]
    CircularReference(i64),

    #[error("削除制約違反: カテゴリ {0} には子カテゴリが存在します")]
    HasChildren(i64),

    #[error("削除制約違反: カテゴリ {0} には資産が紐付いています")]
    HasAssets(i64),

    #[error("データベースエラー: {0}")]
    DatabaseError(String),
}

/// Price value object errors
#[derive(Error, Debug)]
pub enum PriceError {
    #[error("価格は0以上でなければなりません: {0}")]
    #[allow(dead_code)] // Used in later slices
    Negative(i32),
}

// ============================================================================
// Value Objects
// ============================================================================

/// Asset unique identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetId(i64);

impl AssetId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

/// Category unique identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CategoryId(i64);

impl CategoryId {
    #[allow(dead_code)] // Used in later slices
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    #[allow(dead_code)] // Used in later slices
    pub fn value(&self) -> i64 {
        self.0
    }
}

/// Price in Japanese Yen (円)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Used in later slices
pub struct Price(i32);

#[allow(dead_code)] // Used in later slices
impl Price {
    /// Create a new price (must be >= 0)
    pub fn new(amount: i32) -> Result<Self, PriceError> {
        if amount < 0 {
            return Err(PriceError::Negative(amount));
        }
        Ok(Self(amount))
    }

    /// Get the price value
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Check if price is positive (> 0)
    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }
}

/// Duration in days
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)] // Used in later slices
pub struct Duration(i64);

#[allow(dead_code)] // Used in later slices
impl Duration {
    /// Calculate duration between two dates
    pub fn between(start: NaiveDate, end: NaiveDate) -> Self {
        let days = (end - start).num_days();
        Self(days)
    }

    /// Calculate duration from a date to now
    pub fn from_now(start: NaiveDate) -> Self {
        let now = chrono::Local::now().date_naive();
        Self::between(start, now)
    }

    /// Get the number of days
    pub fn days(&self) -> i64 {
        self.0
    }
}
