# Data Model: Asset Management

**Date**: 2026-01-24  
**Phase**: 1 - Design Artifacts

## Overview

Asset Management Bounded Contextのデータモデルを定義します。Domain-Driven Designに基づき、Entities, Value Objects, Aggregates, Repositoriesを整理します。

---

## Entities

### 1. Asset（資産）

**Role**: Aggregate Root

**Attributes**:

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | AssetId (i64) | PRIMARY KEY, NOT NULL | 一意識別子 |
| `name` | String | NOT NULL, max 255 chars | 資産名（例: "AirPods Pro"） |
| `manufacturer` | Option<String> | max 255 chars | メーカー（例: "Apple"） |
| `quantity` | i32 | DEFAULT 1, >= 0 | 現在の数量 |
| `memo` | Option<String> | max 10,000 chars | 自由記述メモ |
| `created_at` | NaiveDateTime | NOT NULL | 作成日時 |
| `updated_at` | NaiveDateTime | NOT NULL | 更新日時 |

**Invariants**:
- `quantity >= 0` （負の数量は不正）
- 少なくとも1つのCategoryに属する必要あり（AssetCategoryで強制）

**Domain Methods**:
```rust
impl Asset {
    pub fn new(name: String, manufacturer: Option<String>) -> Result<Self, AssetError>;
    pub fn update_name(&mut self, name: String) -> Result<(), AssetError>;
    pub fn update_quantity(&mut self, quantity: i32) -> Result<(), AssetError>;
    pub fn add_category(&mut self, category_id: CategoryId) -> Result<(), AssetError>;
    pub fn remove_category(&mut self, category_id: CategoryId) -> Result<(), AssetError>;
}
```

---

### 2. Category（カテゴリ）

**Role**: Entity (独立したAggregate Root)

**Attributes**:

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | CategoryId (i64) | PRIMARY KEY, NOT NULL | 一意識別子 |
| `name` | String | NOT NULL, UNIQUE, max 100 chars | カテゴリ名（例: "イヤホン"） |
| `parent_id` | Option<CategoryId> | FOREIGN KEY → categories(id) | 親カテゴリID（NULLの場合はルート） |
| `display_order` | i32 | DEFAULT 0 | 同階層内の表示順序 |
| `created_at` | NaiveDateTime | NOT NULL | 作成日時 |
| `updated_at` | NaiveDateTime | NOT NULL | 更新日時 |

**Invariants**:
- 循環参照の禁止（`parent_id` が自分自身または子孫を指してはならない）
- 削除制約: 子カテゴリが存在する場合、または資産が紐付いている場合は削除不可

**Domain Methods**:
```rust
impl Category {
    pub fn new(name: String, parent_id: Option<CategoryId>) -> Result<Self, CategoryError>;
    pub fn set_parent(&mut self, parent_id: Option<CategoryId>) -> Result<(), CategoryError>;
    pub fn ancestors(&self, repo: &dyn CategoryRepository) -> Result<Vec<Category>, CategoryError>;
    pub fn descendants(&self, repo: &dyn CategoryRepository) -> Result<Vec<Category>, CategoryError>;
    pub fn is_ancestor_of(&self, other: &Category, repo: &dyn CategoryRepository) -> Result<bool, CategoryError>;
}
```

---

### 3. AssetCategory（資産カテゴリ関連）

**Role**: Association Entity（多対多の中間テーブル）

**Attributes**:

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | i64 | PRIMARY KEY, NOT NULL | 一意識別子 |
| `asset_id` | AssetId | FOREIGN KEY → assets(id), NOT NULL | 資産ID |
| `category_id` | CategoryId | FOREIGN KEY → categories(id), NOT NULL | カテゴリID |
| `linked_at` | NaiveDateTime | NOT NULL | 関連付け日時 |

**Constraints**:
- `UNIQUE(asset_id, category_id)` （同じ組み合わせの重複を防止）

**Domain Methods**:
```rust
impl AssetCategory {
    pub fn new(asset_id: AssetId, category_id: CategoryId) -> Self;
}
```

---

### 4. AssetHistory（資産履歴）

**Role**: Entity（Assetの一部、Aggregate内）

**Attributes**:

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | i64 | PRIMARY KEY, NOT NULL | 一意識別子 |
| `asset_id` | AssetId | FOREIGN KEY → assets(id), NOT NULL | 資産ID |
| `history_type` | HistoryType (enum) | NOT NULL | 履歴種類（取得/処分/破損/紛失/数量調整） |
| `history_date` | NaiveDate | NOT NULL | イベント発生日 |
| `quantity_change` | i32 | NOT NULL | 数量変化（+N, -N, 0） |
| `price` | Option<Price> | >= 0 | 価格（購入価格/売却価格、円単位） |
| `reason` | Option<String> | max 255 chars | 理由（購入、売却、破損等） |
| `location` | Option<String> | max 255 chars | 場所（購入場所/売却場所） |
| `url` | Option<String> | max 2048 chars | URL（購入URL/売却URL） |
| `memo` | Option<String> | max 10,000 chars | 自由記述メモ |
| `created_at` | NaiveDateTime | NOT NULL | 記録日時 |

**HistoryType Enum**:
```rust
pub enum HistoryType {
    Acquisition,  // 取得（購入、贈答）
    Disposal,     // 処分（廃棄、譲渡、売却）
    Damage,       // 破損
    Loss,         // 紛失
    Adjustment,   // 数量調整
}
```

**Domain Methods**:
```rust
impl AssetHistory {
    pub fn record_acquisition(
        asset_id: AssetId,
        date: NaiveDate,
        quantity: i32,
        price: Option<Price>,
        location: Option<String>,
        url: Option<String>,
    ) -> Result<Self, AssetError>;

    pub fn record_disposal(
        asset_id: AssetId,
        date: NaiveDate,
        quantity: i32,
        price: Option<Price>, // 売却価格
        location: Option<String>,
        url: Option<String>,
    ) -> Result<Self, AssetError>;

    pub fn record_damage(
        asset_id: AssetId,
        date: NaiveDate,
        memo: Option<String>,
    ) -> Result<Self, AssetError>;
}
```

---

### 5. ChangeHistory（変更履歴）

**Role**: Entity（監査ログ、Assetの一部、Aggregate内）

**Attributes**:

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | i64 | PRIMARY KEY, NOT NULL | 一意識別子 |
| `asset_id` | AssetId | FOREIGN KEY → assets(id), NOT NULL | 資産ID |
| `operation_type` | OperationType (enum) | NOT NULL | 操作種類 |
| `operation_at` | NaiveDateTime | NOT NULL | 操作日時 |
| `changes` | String (JSON) | max 100,000 chars | 変更内容（変更前後の値） |

**OperationType Enum**:
```rust
pub enum OperationType {
    Create,         // 作成
    Update,         // 編集
    Delete,         // 削除
    AddCategory,    // カテゴリ追加
    RemoveCategory, // カテゴリ削除
}
```

**JSON Format Example**:
```json
{
  "before": {
    "name": "AirPods",
    "manufacturer": null
  },
  "after": {
    "name": "AirPods Pro",
    "manufacturer": "Apple"
  }
}
```

**Domain Methods**:
```rust
impl ChangeHistory {
    pub fn record_create(asset_id: AssetId) -> Self;
    pub fn record_update(asset_id: AssetId, before: &Asset, after: &Asset) -> Self;
    pub fn record_delete(asset_id: AssetId) -> Self;
}
```

---

## Value Objects

### 1. AssetId

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetId(i64);

impl AssetId {
    pub fn new(id: i64) -> Self;
    pub fn value(&self) -> i64;
}
```

### 2. CategoryId

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CategoryId(i64);

impl CategoryId {
    pub fn new(id: i64) -> Self;
    pub fn value(&self) -> i64;
}
```

### 3. Price

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(i32); // 円単位

impl Price {
    pub fn new(amount: i32) -> Result<Self, PriceError>;
    pub fn value(&self) -> i32;
    pub fn is_positive(&self) -> bool;
}

#[derive(Error, Debug)]
pub enum PriceError {
    #[error("価格は0以上でなければなりません: {0}")]
    Negative(i32),
}
```

### 4. Duration

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duration(i64); // 日数

impl Duration {
    pub fn between(start: NaiveDate, end: NaiveDate) -> Self;
    pub fn from_now(start: NaiveDate) -> Self;
    pub fn days(&self) -> i64;
}
```

---

## Aggregates

### Asset Aggregate

**Root**: Asset  
**Members**: AssetHistory (collection), ChangeHistory (collection), AssetCategory (collection)

**Invariants**:
1. `Asset.quantity` は常に `>= 0`
2. `Asset` は少なくとも1つの `Category` に属する
3. `AssetHistory` の `quantity_change` の合計は `Asset.quantity` と整合する（整合性チェックは推奨、強制ではない）

**Transaction Boundary**: 
- Asset作成時: Asset + AssetCategory + ChangeHistory (Create)
- 履歴記録時: AssetHistory + Asset.quantity更新 + ChangeHistory (Update)
- Asset削除時: Asset + 関連するすべての履歴 + ChangeHistory (Delete)

**実装方針**:
- Repository層でトランザクション管理を実装（rusqliteの`transaction()`を使用）
- 複数テーブルへの操作は同一トランザクション内で実行
- エラー時は自動ロールバック（TransactionのDrop実装）
- Application層（Service）からはトランザクションを意識しない設計
- 将来的な拡張: ロールバック機能、ネストされたトランザクション（savepoint）は安定バージョンリリース後に検討

---

### Category Aggregate

**Root**: Category  
**Members**: なし（単独エンティティ）

**Invariants**:
1. 循環参照の禁止
2. 削除制約: 子カテゴリまたは資産が存在する場合は削除不可

---

## Repositories

### AssetRepository

```rust
pub trait AssetRepository {
    fn find(&self, id: AssetId) -> Result<Asset, AssetError>;
    fn find_all(&self) -> Result<Vec<Asset>, AssetError>;
    fn find_by_category(&self, category_id: CategoryId, include_descendants: bool) -> Result<Vec<Asset>, AssetError>;
    fn find_by_manufacturer(&self, manufacturer: &str) -> Result<Vec<Asset>, AssetError>;
    fn save(&self, asset: &Asset) -> Result<(), AssetError>;
    fn delete(&self, id: AssetId) -> Result<(), AssetError>;
}
```

### CategoryRepository

```rust
pub trait CategoryRepository {
    fn find(&self, id: CategoryId) -> Result<Category, CategoryError>;
    fn find_all(&self) -> Result<Vec<Category>, CategoryError>;
    fn find_roots(&self) -> Result<Vec<Category>, CategoryError>;
    fn find_children(&self, parent_id: CategoryId) -> Result<Vec<Category>, CategoryError>;
    fn find_ancestors(&self, id: CategoryId) -> Result<Vec<Category>, CategoryError>;
    fn find_descendants(&self, id: CategoryId) -> Result<Vec<Category>, CategoryError>;
    fn save(&self, category: &Category) -> Result<(), CategoryError>;
    fn delete(&self, id: CategoryId) -> Result<(), CategoryError>;
    fn has_children(&self, id: CategoryId) -> Result<bool, CategoryError>;
    fn has_assets(&self, id: CategoryId) -> Result<bool, CategoryError>;
}
```

### AssetHistoryRepository

```rust
pub trait AssetHistoryRepository {
    fn find_by_asset(&self, asset_id: AssetId) -> Result<Vec<AssetHistory>, AssetError>;
    fn find_first_acquisition(&self, asset_id: AssetId) -> Result<Option<AssetHistory>, AssetError>;
    fn find_last_disposal(&self, asset_id: AssetId) -> Result<Option<AssetHistory>, AssetError>;
    fn save(&self, history: &AssetHistory) -> Result<(), AssetError>;
}
```

### ChangeHistoryRepository

```rust
pub trait ChangeHistoryRepository {
    fn find_by_asset(&self, asset_id: AssetId, limit: usize) -> Result<Vec<ChangeHistory>, AssetError>;
    fn find_by_date_range(&self, start: NaiveDate, end: NaiveDate) -> Result<Vec<ChangeHistory>, AssetError>;
    fn save(&self, history: &ChangeHistory) -> Result<(), AssetError>;
}
```

---

## Database Schema

### SQLite Table Definitions

```sql
-- Assets table
CREATE TABLE assets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    manufacturer TEXT,
    quantity INTEGER NOT NULL DEFAULT 1,
    memo TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    CHECK (quantity >= 0)
);

CREATE INDEX idx_assets_name ON assets(name);
CREATE INDEX idx_assets_manufacturer ON assets(manufacturer);

-- Categories table
CREATE TABLE categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    parent_id INTEGER,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES categories(id) ON DELETE RESTRICT
);

CREATE INDEX idx_categories_parent_id ON categories(parent_id);

-- Asset-Category junction table
CREATE TABLE asset_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    asset_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    linked_at TEXT NOT NULL,
    FOREIGN KEY (asset_id) REFERENCES assets(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE RESTRICT,
    UNIQUE(asset_id, category_id)
);

CREATE INDEX idx_asset_categories_asset_id ON asset_categories(asset_id);
CREATE INDEX idx_asset_categories_category_id ON asset_categories(category_id);

-- Asset History table
CREATE TABLE asset_histories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    asset_id INTEGER NOT NULL,
    history_type TEXT NOT NULL,
    history_date TEXT NOT NULL,
    quantity_change INTEGER NOT NULL,
    price INTEGER,
    reason TEXT,
    location TEXT,
    url TEXT,
    memo TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (asset_id) REFERENCES assets(id) ON DELETE CASCADE,
    CHECK (history_type IN ('acquisition', 'disposal', 'damage', 'loss', 'adjustment')),
    CHECK (price IS NULL OR price >= 0)
);

CREATE INDEX idx_asset_histories_asset_id ON asset_histories(asset_id);
CREATE INDEX idx_asset_histories_history_date ON asset_histories(history_date);

-- Change History table
CREATE TABLE change_histories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    asset_id INTEGER NOT NULL,
    operation_type TEXT NOT NULL,
    operation_at TEXT NOT NULL,
    changes TEXT NOT NULL,
    FOREIGN KEY (asset_id) REFERENCES assets(id) ON DELETE CASCADE,
    CHECK (operation_type IN ('create', 'update', 'delete', 'add_category', 'remove_category'))
);

CREATE INDEX idx_change_histories_asset_id ON change_histories(asset_id);
CREATE INDEX idx_change_histories_operation_at ON change_histories(operation_at);
```

### Migration Strategy

1. **Initial Schema**: `migrations/001_initial_schema.sql`
2. **Manual Execution**: アプリ起動時にマイグレーション適用
3. **Version Table**: `schema_version` テーブルで管理
4. **Transaction Management**: 
   - 各マイグレーションはトランザクション内で実行（rusqliteの`transaction()`を使用）
   - 失敗時は自動ロールバック（TransactionのDrop実装により）
   - `schema_version`への記録もトランザクション内で実行
   - 既存データの保護を確保

```sql
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);
```

**実装方針**:
- `rusqlite::Connection::transaction()`を使用してトランザクション開始
- マイグレーションSQL実行と`schema_version`への記録を同一トランザクション内で実行
- エラー時は自動ロールバック（TransactionのDrop実装）
- ロールバック機能自体は安定バージョンリリース後に検討（現時点では自動ロールバックで対応）

---

## Domain Services

### OwnershipDurationService

```rust
pub struct OwnershipDurationService;

impl OwnershipDurationService {
    pub fn calculate_duration(
        asset_id: AssetId,
        history_repo: &dyn AssetHistoryRepository,
    ) -> Result<Option<Duration>, AssetError> {
        let first_acquisition = history_repo.find_first_acquisition(asset_id)?;
        let last_disposal = history_repo.find_last_disposal(asset_id)?;

        match (first_acquisition, last_disposal) {
            (Some(acq), Some(disp)) => {
                Ok(Some(Duration::between(acq.history_date, disp.history_date)))
            }
            (Some(acq), None) => {
                Ok(Some(Duration::from_now(acq.history_date)))
            }
            (None, _) => Ok(None),
        }
    }
}
```

### CategoryHierarchyService

```rust
pub struct CategoryHierarchyService;

impl CategoryHierarchyService {
    pub fn validate_no_cycle(
        category_id: CategoryId,
        new_parent_id: Option<CategoryId>,
        repo: &dyn CategoryRepository,
    ) -> Result<(), CategoryError> {
        // 再帰的にnew_parent_idの祖先をたどり、category_idが含まれていればエラー
    }

    pub fn get_breadcrumb(
        category_id: CategoryId,
        repo: &dyn CategoryRepository,
    ) -> Result<Vec<Category>, CategoryError> {
        // 祖先カテゴリをルートから現在まで並べる
    }
}
```

---

## Summary

Asset Management Bounded Contextのデータモデルが完全に定義されました。

- **Entities**: 5個（Asset, Category, AssetCategory, AssetHistory, ChangeHistory）
- **Value Objects**: 4個（AssetId, CategoryId, Price, Duration）
- **Aggregates**: 2個（Asset Aggregate, Category Aggregate）
- **Repositories**: 4個（Asset, Category, AssetHistory, ChangeHistory）
- **Domain Services**: 2個（OwnershipDuration, CategoryHierarchy）
- **Database Tables**: 5個（SQLiteスキーマ定義済み）

次のステップ: contracts/tui-api.md, quickstart.md を生成
