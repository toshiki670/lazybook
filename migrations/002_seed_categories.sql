-- Migration: 002_seed_categories.sql
-- Description: Seed initial category hierarchy
-- Date: 2026-01-24

-- Root categories
INSERT INTO categories (name, parent_id, display_order, created_at, updated_at)
VALUES 
    ('家電', NULL, 0, datetime('now'), datetime('now')),
    ('衣類', NULL, 1, datetime('now'), datetime('now')),
    ('書籍', NULL, 2, datetime('now'), datetime('now')),
    ('その他', NULL, 3, datetime('now'), datetime('now'));

-- Record migration
INSERT INTO schema_version (version, applied_at) 
VALUES (2, datetime('now'));
