# Quickstart Guide: Asset Management

**Date**: 2026-01-24  
**Phase**: 1 - Design Artifacts

## Overview

Asset Management機能の開発を開始するためのクイックスタートガイドです。環境セットアップから最初のテスト実行まで、段階的に説明します。

---

## Prerequisites

### Required Tools

以下のツールを mise で管理します。`.mise.toml` に記載された通りのバージョンを使用してください。

| Tool | Purpose | Installation Command |
|------|---------|---------------------|
| **mise** | Tool version manager | [mise.jdx.dev](https://mise.jdx.dev/) からインストール |
| **Rust** | Programming language | `mise install rust@stable` |
| **cargo** | Rust package manager | Rust toolchainに含まれる |

### System Requirements

- **OS**: macOS 10.15+ or Linux (kernel 4.x+)
- **Terminal**: UTF-8対応ターミナル（iTerm2, Alacritty, GNOME Terminal等）
- **Screen Size**: 最小80カラム × 24行

---

## Initial Setup

### 1. Clone Repository

```bash
git clone https://github.com/toshiki670/lazybook.git
cd lazybook
```

### 2. Install Tools via mise

```bash
# miseをインストール（まだの場合）
curl https://mise.run | sh

# プロジェクト固有のツールをインストール
mise install

# インストール確認
mise list
```

### 3. Create Feature Branch

（既にブランチ `001-asset-management` が存在する場合はスキップ）

```bash
git checkout -b 001-asset-management
```

### 4. Initialize Rust Project Structure

```bash
# Cargo.tomlの作成（まだ存在しない場合）
cargo init --name lazybook

# 依存関係の追加
cargo add ratatui@0.25
cargo add crossterm@0.27
cargo add rusqlite --features bundled
cargo add clap --features derive
cargo add thiserror@1.0
cargo add anyhow@1.0
cargo add chrono@0.4
cargo add log@0.4
cargo add env_logger@0.11
cargo add serde --features derive
cargo add serde_json

# 開発依存関係の追加
cargo add --dev proptest  # Optional: property-based testing
```

---

## Project Structure Setup

### 1. Create Directory Structure

```bash
# Domain層
mkdir -p src/domain/asset
mkdir -p src/domain/category
mkdir -p src/domain/history
mkdir -p src/domain/shared

# Application層
mkdir -p src/application

# Infrastructure層
mkdir -p src/infrastructure/persistence
mkdir -p src/infrastructure/export
mkdir -p src/infrastructure/logging

# Presentation層
mkdir -p src/presentation/screens
mkdir -p src/presentation/components

# Tests
mkdir -p tests/unit/domain
mkdir -p tests/unit/application
mkdir -p tests/unit/infrastructure
mkdir -p tests/integration
mkdir -p tests/contract

# Migrations
mkdir -p migrations
```

### 2. Create Initial Module Files

```bash
# Domain modules
touch src/domain/mod.rs
touch src/domain/asset/mod.rs
touch src/domain/asset/entity.rs
touch src/domain/asset/value_objects.rs
touch src/domain/asset/repository.rs
touch src/domain/category/mod.rs
touch src/domain/category/entity.rs
touch src/domain/category/repository.rs
touch src/domain/history/mod.rs
touch src/domain/history/asset_history.rs
touch src/domain/history/change_history.rs
touch src/domain/history/repository.rs
touch src/domain/shared/mod.rs
touch src/domain/shared/types.rs

# Application modules
touch src/application/mod.rs
touch src/application/asset_service.rs
touch src/application/category_service.rs
touch src/application/history_service.rs
touch src/application/dto.rs

# Infrastructure modules
touch src/infrastructure/mod.rs
touch src/infrastructure/persistence/mod.rs
touch src/infrastructure/persistence/schema.rs
touch src/infrastructure/persistence/migrations.rs
touch src/infrastructure/persistence/asset_repository.rs
touch src/infrastructure/persistence/category_repository.rs
touch src/infrastructure/persistence/history_repository.rs
touch src/infrastructure/export/mod.rs
touch src/infrastructure/export/markdown.rs
touch src/infrastructure/logging/mod.rs
touch src/infrastructure/logging/logger.rs

# Presentation modules
touch src/presentation/mod.rs
touch src/presentation/app.rs
touch src/presentation/navigation.rs
touch src/presentation/screens/mod.rs
touch src/presentation/screens/asset_list.rs
touch src/presentation/screens/asset_detail.rs
touch src/presentation/screens/asset_form.rs
touch src/presentation/screens/category_tree.rs
touch src/presentation/screens/history_view.rs
touch src/presentation/screens/help.rs
touch src/presentation/components/mod.rs
touch src/presentation/components/table.rs
touch src/presentation/components/form.rs
touch src/presentation/components/tree.rs
touch src/presentation/components/dialog.rs
```

### 3. Update src/lib.rs

```rust
// src/lib.rs
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
```

### 4. Update src/main.rs

```rust
// src/main.rs
use clap::Parser;
use lazybook::presentation::app::App;

#[derive(Parser)]
#[command(name = "lazybook")]
#[command(about = "個人情報管理ツール", long_about = None)]
struct Cli {}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let _cli = Cli::parse();
    
    println!("lazybook Asset Management TUI");
    println!("準備中...");
    
    // TUIアプリ起動
    // 実装は Phase 3 Slice 1.1 (T022) で行う
    // let mut app = asset_management::presentation::App::new()?;
    // app.run()?;
    // app.run()?;
    
    Ok(())
}
```

---

## Database Setup

### 1. Create Initial Migration

```sql
-- migrations/001_initial_schema.sql

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

-- Schema Version table
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);

INSERT INTO schema_version (version, applied_at) VALUES (1, datetime('now'));
```

### 2. Seed Initial Categories

```sql
-- migrations/002_seed_categories.sql

-- ルートカテゴリ
INSERT INTO categories (name, parent_id, display_order, created_at, updated_at)
VALUES
('家電', NULL, 1, datetime('now'), datetime('now')),
('衣類', NULL, 2, datetime('now'), datetime('now')),
('書籍', NULL, 3, datetime('now'), datetime('now')),
('その他', NULL, 99, datetime('now'), datetime('now'));

-- 子カテゴリ（家電）
INSERT INTO categories (name, parent_id, display_order, created_at, updated_at)
VALUES
('オーディオ機器', 1, 1, datetime('now'), datetime('now')),
('キッチン家電', 1, 2, datetime('now'), datetime('now')),
('生活家電', 1, 3, datetime('now'), datetime('now'));

-- 孫カテゴリ（オーディオ機器）
INSERT INTO categories (name, parent_id, display_order, created_at, updated_at)
VALUES
('イヤホン', 5, 1, datetime('now'), datetime('now')),
('スピーカー', 5, 2, datetime('now'), datetime('now'));

UPDATE schema_version SET version = 2, applied_at = datetime('now');
```

---

## Build & Run

### 1. Check Compilation

```bash
cargo check
```

### 2. Run Tests

```bash
# 全テスト実行
cargo test

# 特定のテストのみ実行
cargo test --test integration
cargo test domain::asset
```

### 3. Run Application

```bash
# デバッグビルド + 実行
cargo run

# リリースビルド + 実行
cargo run --release

# ログレベル指定
RUST_LOG=debug cargo run
```

---

## Development Workflow

### TDD Cycle (Red-Green-Refactor)

1. **Red**: テストを先に書く（失敗する）
   ```bash
   cargo test domain::asset::test_create_asset
   # Expected: test failed
   ```

2. **Green**: 最小限の実装でテストをパスさせる
   ```rust
   // src/domain/asset/entity.rs
   impl Asset {
       pub fn new(name: String, ...) -> Result<Self, AssetError> {
           // 実装
       }
   }
   ```
   ```bash
   cargo test domain::asset::test_create_asset
   # Expected: test passed
   ```

3. **Refactor**: コードを整理・最適化
   ```bash
   cargo clippy -- -D warnings
   cargo fmt
   ```

### Git Commit Strategy

```bash
# 各ユーザーストーリー単位でコミット
git add src/domain/asset/
git commit -m "feat: implement Asset entity (User Story 1)"

# テスト追加
git add tests/unit/domain/
git commit -m "test: add Asset entity unit tests"

# リファクタリング
git commit -m "refactor: extract validation logic to value objects"
```

---

## Debugging Tips

### 1. Enable Logging

```bash
# すべてのログを表示
RUST_LOG=trace cargo run

# 特定モジュールのみ
RUST_LOG=lazybook::domain=debug cargo run
```

### 2. SQLite Database Inspection

```bash
# SQLiteファイルの場所（実行時に生成）
sqlite3 ./lazybook.db

# テーブル一覧
.tables

# スキーマ確認
.schema assets

# データ確認
SELECT * FROM assets;

# 終了
.quit
```

### 3. TUI Debugging

TUIアプリのデバッグは通常のprintデバッグが使えないため、ログファイルに出力します。

```rust
use log::debug;

debug!("Current screen: {:?}", self.current_screen);
```

```bash
# ログファイルに出力
RUST_LOG=debug cargo run 2> debug.log

# 別ターミナルでログ監視
tail -f debug.log
```

---

## Testing Strategy

### Unit Tests

```bash
# Domain層のテスト
cargo test --lib domain::

# Application層のテスト
cargo test --lib application::
```

### Integration Tests

```bash
# すべての統合テスト実行
cargo test --test '*'

# 特定の統合テスト実行
cargo test --test asset_lifecycle_test
```

### Contract Tests

```bash
# TUIコンポーネントのコントラクトテスト
cargo test --test contract
```

### Coverage (Optional)

```bash
# tarpaulinをインストール
cargo install cargo-tarpaulin

# カバレッジ計測
cargo tarpaulin --out Html --output-dir coverage
```

---

## Performance Profiling

### Benchmarks (Optional)

```bash
# criterionをインストール
cargo add --dev criterion

# ベンチマーク実行
cargo bench

# 特定のベンチマークのみ
cargo bench asset_query
```

---

## CI/CD Setup (Future)

### GitHub Actions Example

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: jdx/mise-action@v2
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check
```

---

## Troubleshooting

### Problem: `cargo build` fails with "linker not found"

**Solution**: Install build tools
```bash
# macOS
xcode-select --install

# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora
sudo dnf install gcc
```

### Problem: SQLite errors on startup

**Solution**: Check file permissions and database location
```bash
ls -la lazybook.db
chmod 644 lazybook.db
```

### Problem: TUI rendering is broken

**Solution**: Check terminal UTF-8 support
```bash
echo $LANG  # Should show UTF-8
locale -a   # Should list UTF-8 locales
```

---

## Next Steps

1. **User Story 1 (P1)**: 資産の新規登録機能を実装
   - Domain: Asset entity, AssetRepository trait
   - Application: AssetService::create_asset()
   - Infrastructure: SqliteAssetRepository
   - Presentation: AssetFormScreen (New mode)
   - Tests: Unit + Integration tests

2. **User Story 2 (P2)**: 資産一覧表示機能を実装
   - Domain: Query methods in AssetRepository
   - Application: AssetService::list_assets()
   - Presentation: AssetListScreen
   - Tests: Rendering tests

3. **Continue with remaining User Stories** (P3-P10)

---

## Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **ratatui Documentation**: https://ratatui.rs/
- **rusqlite Documentation**: https://docs.rs/rusqlite/
- **DDD in Rust**: https://github.com/valyagolev/rust-ddd-example
- **Project Specification**: `/specs/001-asset-management/spec.md`
- **Data Model**: `/specs/001-asset-management/data-model.md`
- **TUI Contracts**: `/specs/001-asset-management/contracts/tui-api.md`

---

## Summary

環境セットアップが完了しました。次のコマンドで開発を開始できます：

```bash
cd lazybook
mise install
cargo test
cargo run
```

TDD サイクル（Red-Green-Refactor）を守りながら、User Story 1（資産の新規登録）から実装を開始してください。
