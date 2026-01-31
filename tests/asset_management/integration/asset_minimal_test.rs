// Integration test: Minimal asset creation
// Test FIRST approach - these tests should FAIL initially

use lazybook::asset_management::application::asset_service::AssetService;
use lazybook::asset_management::infrastructure::persistence::{
    SqliteAssetRepository, establish_connection, initialize_database,
};
use std::path::PathBuf;
use tempfile::TempDir;

/// Setup test database
fn setup_test_db() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    (temp_dir, db_path)
}

#[test]
fn test_create_asset_with_name_only() {
    // Setup
    let (_temp_dir, db_path) = setup_test_db();
    let mut conn = establish_connection(&db_path).expect("Failed to establish connection");
    initialize_database(&mut conn).expect("Failed to initialize database");

    let repository = SqliteAssetRepository::new(conn);
    let service = AssetService::new(repository);

    // Act: Create asset with name only
    let asset_id = service
        .create_asset("テストアイテム".to_string(), None, None, None, vec![])
        .expect("Failed to create asset");

    // Assert: Asset should be created with ID
    assert!(asset_id.value() > 0);

    // Verify: Asset can be retrieved
    let asset = service.get_asset(asset_id).expect("Failed to get asset");
    assert_eq!(asset.name, "テストアイテム");
    assert_eq!(asset.manufacturer, None);
    assert_eq!(asset.quantity, 1);
}

#[test]
fn test_list_created_assets() {
    // Setup
    let (_temp_dir, db_path) = setup_test_db();
    let mut conn = establish_connection(&db_path).expect("Failed to establish connection");
    initialize_database(&mut conn).expect("Failed to initialize database");

    let repository = SqliteAssetRepository::new(conn);
    let service = AssetService::new(repository);

    // Act: Create multiple assets
    service
        .create_asset("アイテム1".to_string(), None, None, None, vec![])
        .expect("Failed to create asset 1");
    service
        .create_asset("アイテム2".to_string(), None, None, None, vec![])
        .expect("Failed to create asset 2");

    // Assert: List should return all created assets
    let assets = service.list_assets().expect("Failed to list assets");
    assert_eq!(assets.len(), 2);
    assert_eq!(assets[0].name, "アイテム1");
    assert_eq!(assets[1].name, "アイテム2");
}
