-- Migration: 001_initial_schema.sql
-- Description: Create initial database schema for Asset Management
-- Date: 2026-01-24

-- Schema version tracking table
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);

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
