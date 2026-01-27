# Tasks: Asset Management (Vertical Slice Approach)

**Input**: Design documents from `/specs/001-asset-management/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Test tasks are included (TDD approach per Constitution Principle IV)

**Organization**: Tasks are organized by **Vertical Slices** - each slice delivers a working, end-to-end feature through all layers (Domain → Infrastructure → Application → Presentation).

---

## ⚡ Vertical Slice Philosophy

**Key Principle**: Each slice must produce a **working, demoable feature** that touches all architectural layers.

**Benefits**:
- ✅ Early feedback (working software after each slice)
- ✅ Risk reduction (integration issues discovered incrementally)
- ✅ Continuous validation (demo after each slice)
- ✅ Motivation (see progress immediately)

**Anti-pattern to avoid**: Implementing all entities first, then all repositories, then all services (Horizontal Layering).

---

## Format: `[ID] [S#] [Story] Description`

- **[S#]**: Slice number within the user story (e.g., S1, S2, S3)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- **[P]**: Can run in parallel (different files, no dependencies) - only within same slice
- Include exact file paths in descriptions

## Path Conventions

- **Bounded Context最上位**: `src/asset_management/` (Asset Management context)
- **DDD 4層構造**: `{domain,application,infrastructure,presentation}` under each context
- **Shared kernel**: `src/shared/` (cross-context shared utilities)
- **Tests**: `tests/asset_management/{unit,integration,contract}`
- Future contexts: `src/finance_management/`, `src/knowledge_management/`, etc.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Initialize Rust project with Cargo.toml using `cargo init --name lazybook`
- [X] T002 [P] Add dependencies to Cargo.toml: ratatui@0.25, crossterm@0.27, rusqlite (bundled), clap (derive), thiserror, anyhow, chrono, log, env_logger, serde, serde_json
- [X] T003 [P] Configure clippy and rustfmt settings in rustfmt.toml and clippy.toml
- [X] T004 [P] Setup mise configuration in .mise.toml for Rust stable toolchain
- [X] T005 Create directory structure: src/{asset_management/{domain,application,infrastructure,presentation},shared}, tests/asset_management/{unit,integration,contract}, migrations/
- [X] T006 [P] Create module files: src/lib.rs, src/main.rs, src/asset_management/mod.rs, src/shared/mod.rs, and all nested mod.rs files per plan.md structure
- [X] T007 [P] Create initial migration 001_initial_schema.sql in migrations/ with all 5 tables (assets, categories, asset_categories, asset_histories, change_histories, schema_version)
- [X] T008 [P] Create seed migration 002_seed_categories.sql with initial category hierarchy (家電, 衣類, 書籍, その他)
- [X] T008a [P] Run cargo fmt: `cargo fmt --all` to format all code
- [X] T008b [P] Run cargo clippy: `cargo clippy -- -D warnings` to check code quality

**Checkpoint**: Project structure complete, ready for foundational implementation

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

**FRs**: FR-010, FR-011, FR-013 (基盤要件: データ永続化、起動時読み込み、キーボード操作)

- [X] T009 Implement Error types in src/asset_management/domain/shared/types.rs: AssetError, CategoryError, PriceError using thiserror
- [X] T010 [P] Implement Value Objects in src/asset_management/domain/shared/types.rs: AssetId, CategoryId, Price, Duration with validation
- [X] T011 [P] Setup logging infrastructure in src/shared/logging/logger.rs using log + env_logger
- [X] T012 [P] Implement database migration system in src/asset_management/infrastructure/persistence/migrations.rs: apply_migrations(), check_schema_version()
- [ ] T012a [P] Add transaction management to apply_migration() in src/asset_management/infrastructure/persistence/migrations.rs: wrap SQL execution and schema_version record in rusqlite::Connection::transaction(). Reference: data-model.md "Transaction Boundary" section (L456-472)
- [X] T013 Implement database connection manager in src/asset_management/infrastructure/persistence/mod.rs: establish_connection(), initialize_database() (FR-010, FR-011: データ永続化と起動時読み込み)
- [X] T014 [P] Create Repository traits in src/asset_management/domain/asset/repository.rs, src/asset_management/domain/category/repository.rs (interfaces only, no implementation yet)
- [X] T014a [P] Run cargo fmt: `cargo fmt --all` to format all code
- [X] T014b [P] Run cargo clippy: `cargo clippy -- -D warnings` to check code quality

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Add New Asset (Priority: P1) 🎯 MVP

**Goal**: ユーザーが新しい資産を登録できる（名前、カテゴリ、メーカー、購入情報）

**Independent Test**: 資産を1件登録し、登録した情報が正しく保存されていることを確認

**FRs**: FR-001, FR-002, FR-010, FR-011, FR-012, FR-013, FR-014, FR-022
**SCs**: SC-001 (30秒以内に登録)

---

### 🔹 Slice 1.1: Walking Skeleton - 最小限の資産登録（名前のみ）

**Goal**: 資産名だけを入力して登録し、一覧で確認できる**最小限の動くもの**を完成させる

**Why this first**: End-to-end integration (UI → Service → Repository → Database) を早期に検証。すべてのレイヤーが正しく接続されているか確認。

**Tests (Write FIRST, ensure FAIL)**:
- [X] T015 [S1] [US1] Integration test: End-to-end minimal asset creation in tests/asset_management/integration/asset_minimal_test.rs - test_create_asset_with_name_only(), test_list_created_assets()

**Implementation (All layers in sequence)**:

**Domain Layer**:
- [X] T016 [S1] [US1] Implement minimal Asset entity in src/asset_management/domain/asset/entity.rs: Asset struct with id+name only, new(name), validate_name()

**Infrastructure Layer**:
- [X] T017 [S1] [US1] Implement minimal SqliteAssetRepository in src/asset_management/infrastructure/persistence/asset_repository.rs: save(asset), find_all() - INSERT/SELECT for assets table

**Application Layer**:
- [X] T018 [S1] [US1] Implement minimal AssetService in src/asset_management/application/asset_service.rs: create_asset(name) -> Result<Asset>

**Presentation Layer**:
- [X] T019 [S1] [US1] Implement minimal AssetFormScreen in src/asset_management/presentation/screens/asset_form.rs: render name input field, handle Enter key, save action (FR-013: キーボードのみ操作)
- [X] T020 [S1] [US1] Implement minimal AssetListScreen in src/asset_management/presentation/screens/asset_list.rs: render table with name column, display all assets (FR-013: キーボードのみ操作)
- [X] T021 [S1] [US1] Implement minimal App state machine in src/asset_management/presentation/app.rs: Screen enum (List, Form), transition logic, event loop (FR-013: キーボードのみ操作)

**Integration**:
- [X] T022 [S1] [US1] Wire up main.rs: initialize database, apply migrations, launch TUI app (FR-010, FR-011: データ永続化と起動時読み込み)
- [X] T022a [S1] [US1] Run cargo fmt: `cargo fmt --all` to format all code
- [X] T022b [S1] [US1] Run cargo clippy: `cargo clippy -- -D warnings` to check code quality

**Checkpoint**: ✅ **DEMO** - `cargo run` → TUI起動 → 名前入力 → 保存 → 一覧画面に表示される（10タスク完了）

---

### 🔹 Slice 1.2: カテゴリ機能追加

**Goal**: 資産に1つ以上のカテゴリを選択できるようにする

**Why this second**: カテゴリは必須要件（FR-002）。Walking Skeletonが動いている状態で、カテゴリ機能を追加。

**Tests**:
- [ ] T023 [S2] [US1] Unit test: Asset with categories in tests/asset_management/unit/domain/asset_entity_test.rs - test_asset_requires_category(), test_multiple_categories()
- [ ] T024 [S2] [US1] Integration test: Asset with categories persistence in tests/asset_management/integration/asset_category_test.rs - test_save_and_retrieve_asset_with_categories()

**Implementation**:

**Domain Layer**:
- [ ] T025 [S2] [US1] Implement Category entity in src/asset_management/domain/category/entity.rs: Category struct, new(name, parent_id), validate()
- [ ] T026 [S2] [US1] Implement AssetCategory association in src/asset_management/domain/asset/entity.rs: AssetCategory struct, Asset::add_category(), Asset::categories()
- [ ] T027 [S2] [US1] Extend Asset entity validation in src/asset_management/domain/asset/entity.rs: validate_has_categories() - at least one category required

**Infrastructure Layer**:
- [ ] T028 [S2] [US1] Implement SqliteCategoryRepository in src/asset_management/infrastructure/persistence/category_repository.rs: find_all(), find_by_id(), find_roots()
- [ ] T029 [S2] [US1] Extend SqliteAssetRepository in src/asset_management/infrastructure/persistence/asset_repository.rs: save with asset_categories junction table, find_all with JOIN
- [ ] T029a [S2] [US1] Add transaction management to SqliteAssetRepository::save() in src/asset_management/infrastructure/persistence/asset_repository.rs: wrap Asset INSERT and AssetCategory INSERT operations in rusqlite::Connection::transaction(). Reference: data-model.md "Transaction Boundary" section (L288-291) - Asset作成時: Asset + AssetCategory

**Application Layer**:
- [ ] T030 [S2] [US1] Extend AssetService in src/asset_management/application/asset_service.rs: create_asset(name, category_ids), list_categories()
- [ ] T031 [S2] [US1] Add AssetDto, CategoryDto in src/asset_management/application/dto.rs: conversion from domain entities

**Presentation Layer**:
- [ ] T032 [S2] [US1] Extend AssetFormScreen in src/asset_management/presentation/screens/asset_form.rs: add category multi-select widget, validation for at least one category
- [ ] T033 [S2] [US1] Extend AssetListScreen in src/asset_management/presentation/screens/asset_list.rs: add categories column, display all categories for each asset
- [ ] T033a [S2] [US1] Run cargo fmt: `cargo fmt --all` to format all code
- [ ] T033b [S2] [US1] Run cargo clippy: `cargo clippy -- -D warnings` to check code quality

**Checkpoint**: ✅ **DEMO** - カテゴリ選択 → 保存 → 一覧にカテゴリ表示（13タスク累計21）

---

### 🔹 Slice 1.3: メーカー・数量・メモ追加

**Goal**: 任意項目（メーカー、数量、メモ）を入力できるようにする

**Why this third**: 基本機能が動いている状態で、追加フィールドを拡張。

**Tests**:
- [ ] T034 [S3] [US1] Unit test: Asset with all fields in tests/asset_management/unit/domain/asset_entity_test.rs - test_asset_with_manufacturer(), test_quantity_validation(), test_memo()

**Implementation**:

**Domain Layer**:
- [ ] T035 [S3] [US1] Extend Asset entity in src/asset_management/domain/asset/entity.rs: add manufacturer (Option<String>), quantity (i32, default 1), memo (Option<String>), validate_quantity()

**Infrastructure Layer**:
- [ ] T036 [S3] [US1] Extend SqliteAssetRepository in src/asset_management/infrastructure/persistence/asset_repository.rs: UPDATE/INSERT to include manufacturer, quantity, memo columns

**Application Layer**:
- [ ] T037 [S3] [US1] Extend AssetService in src/asset_management/application/asset_service.rs: create_asset with all parameters (name, category_ids, manufacturer, quantity, memo)
- [ ] T038 [S3] [US1] Extend AssetDto in src/asset_management/application/dto.rs: add all fields

**Presentation Layer**:
- [ ] T039 [S3] [US1] Implement FormComponent in src/asset_management/presentation/components/form.rs: reusable form widget (text input, number input, textarea)
- [ ] T040 [S3] [US1] Extend AssetFormScreen in src/asset_management/presentation/screens/asset_form.rs: add manufacturer input, quantity input, memo textarea using FormComponent
- [ ] T041 [S3] [US1] Extend AssetListScreen in src/asset_management/presentation/screens/asset_list.rs: add manufacturer column, quantity column
- [ ] T041a [S3] [US1] Run cargo fmt: `cargo fmt --all` to format all code
- [ ] T041b [S3] [US1] Run cargo clippy: `cargo clippy -- -D warnings` to check code quality

**Checkpoint**: ✅ **DEMO** - 全項目入力 → 保存 → 一覧に全情報表示（10タスク累計29）

---

### 🔹 Slice 1.4: 変更履歴記録

**Goal**: 資産の作成・編集操作を変更履歴として記録する

**Why this fourth**: 基本的な資産登録が完成した後、監査ログ機能を追加。

**Tests**:
- [ ] T042 [S4] [US1] Unit test: ChangeHistory recording in tests/asset_management/unit/application/asset_service_test.rs - test_create_asset_records_change_history()

**Implementation**:

**Domain Layer**:
- [ ] T043 [S4] [US1] Implement ChangeHistory entity in src/asset_management/domain/history/change_history.rs: ChangeHistory struct, record_create(), record_update(), to_json()
- [ ] T044 [S4] [US1] Create ChangeHistoryRepository trait in src/asset_management/domain/history/repository.rs: save(), find_by_asset()

**Infrastructure Layer**:
- [ ] T045 [S4] [US1] Implement SqliteChangeHistoryRepository in src/asset_management/infrastructure/persistence/history_repository.rs: save(), find_by_asset() - INSERT/SELECT for change_histories table

**Application Layer**:
- [ ] T046 [S4] [US1] Extend AssetService in src/asset_management/application/asset_service.rs: record ChangeHistory after create_asset() operation
- [ ] T046a [S4] [US1] Extend transaction management in SqliteAssetRepository::save() to include ChangeHistory: wrap Asset + AssetCategory + ChangeHistory operations in rusqlite::Connection::transaction(). Reference: data-model.md "Transaction Boundary" section (L288-291) - Asset作成時: Asset + AssetCategory + ChangeHistory (Create). Note: Repository層でトランザクション管理を実装し、Application層（Service）からはトランザクションを意識しない設計
- [ ] T046b [S4] [US1] Run cargo fmt: `cargo fmt --all` to format all code
- [ ] T046c [S4] [US1] Run cargo clippy: `cargo clippy -- -D warnings` to check code quality

**Checkpoint**: ✅ **DEMO** - 資産作成 → change_histories テーブルにレコード保存確認（6タスク累計33）

**Phase 3 Complete**: User Story 1 fully implemented - user can create assets with all fields and change history tracking

---

## Phase 4: User Story 2 - View Asset List (Priority: P2)

**Goal**: 登録済み資産の一覧表示、フィルタリング、並び替え機能

**Independent Test**: 事前に複数の資産を登録し、一覧表示・フィルタ・ソートが正しく動作することを確認

**FRs**: FR-003, FR-004, FR-005, FR-006, FR-015
**SCs**: SC-002 (100件を1秒以内), SC-003 (5回以内のキー操作)

**Note**: AssetListScreen の基本実装は Slice 1.1で完了済み。このフェーズでは機能拡張を行う。

---

### 🔹 Slice 2.1: フィルタリング機能

**Goal**: カテゴリ、メーカーで資産をフィルタリングできる

**Tests**:
- [ ] T047 [S1] [US2] Unit test: Asset filtering in tests/asset_management/unit/application/asset_service_test.rs - test_filter_by_category(), test_filter_by_manufacturer()
- [ ] T048 [S1] [US2] Integration test: Filter performance in tests/asset_management/integration/asset_query_test.rs - test_filter_100_assets_under_1_second()

**Implementation**:

**Domain Layer**:
- [ ] T049 [S1] [US2] Extend AssetRepository trait in src/asset_management/domain/asset/repository.rs: find_by_category(), find_by_manufacturer()

**Infrastructure Layer**:
- [ ] T050 [S1] [US2] Implement filtering queries in SqliteAssetRepository in src/asset_management/infrastructure/persistence/asset_repository.rs: WHERE clauses, JOIN for categories

**Application Layer**:
- [ ] T051 [S1] [US2] Extend AssetService in src/asset_management/application/asset_service.rs: filter_by_category(category_id), filter_by_manufacturer(name)

**Presentation Layer**:
- [ ] T052 [S1] [US2] Add filter dialog to AssetListScreen in src/asset_management/presentation/screens/asset_list.rs: filter menu (F key), category selector, manufacturer input
- [ ] T053 [S1] [US2] Implement DialogComponent in src/asset_management/presentation/components/dialog.rs: reusable dialog widget for filter input

**Checkpoint**: ✅ **DEMO** - 一覧画面でFキー → フィルタ選択 → 絞り込み表示（7タスク）

---

### 🔹 Slice 2.2: ソート機能

**Goal**: 名前、購入日、メーカー、所有期間で並び替えできる

**Tests**:
- [ ] T054 [S2] [US2] Unit test: Asset sorting in tests/asset_management/unit/application/asset_service_test.rs - test_sort_by_name(), test_sort_by_date()

**Implementation**:

**Domain Layer**:
- [ ] T055 [S2] [US2] Extend AssetRepository trait in src/asset_management/domain/asset/repository.rs: find_all_sorted(field, direction)

**Infrastructure Layer**:
- [ ] T056 [S2] [US2] Implement sorting queries in SqliteAssetRepository in src/asset_management/infrastructure/persistence/asset_repository.rs: ORDER BY clauses

**Application Layer**:
- [ ] T057 [S2] [US2] Extend AssetService in src/asset_management/application/asset_service.rs: sort_assets(field, direction)

**Presentation Layer**:
- [ ] T058 [S2] [US2] Implement TableComponent in src/asset_management/presentation/components/table.rs: reusable table with column headers, sort indicators, click to sort
- [ ] T059 [S2] [US2] Extend AssetListScreen to use TableComponent in src/asset_management/presentation/screens/asset_list.rs: header row with sort indicators, keyboard shortcuts for column sort

**Checkpoint**: ✅ **DEMO** - カラムヘッダー選択 → ソート（6タスク累計44）

---

### 🔹 Slice 2.3: 購入価格・所有期間表示

**Goal**: 一覧に購入価格と所有期間を追加表示（AssetHistoryから計算）

**Why this slice**: FR-004で要求されているが、AssetHistoryが必要なのでUS6後に実装すべき。ただし、ダミーデータでUI実装は先行可能。

**Tests**:
- [ ] T060 [S3] [US2] Unit test: OwnershipDurationService in tests/asset_management/unit/domain/services_test.rs - test_calculate_duration_placeholder()

**Implementation**:

**Domain Layer**:
- [ ] T061 [S3] [US2] Implement placeholder OwnershipDurationService in src/asset_management/domain/shared/services.rs: calculate_duration() returns dummy value (0 days) - will be implemented properly in US8

**Application Layer**:
- [ ] T062 [S3] [US2] Extend AssetDto in src/asset_management/application/dto.rs: add purchase_price (Option<i32>), ownership_duration (Option<i32>)

**Presentation Layer**:
- [ ] T063 [S3] [US2] Extend AssetListScreen in src/asset_management/presentation/screens/asset_list.rs: add purchase_price column, ownership_duration column (show "-" if None)

**Checkpoint**: ✅ **DEMO** - 一覧に購入価格・所有期間列表示（プレースホルダー）（3タスク累計47）

**Phase 4 Complete**: User Story 2 fully implemented - user can view, filter, and sort asset list

---

## Phase 5: User Story 3 - View Asset Details (Priority: P3)

**Goal**: 資産の詳細情報表示（すべての属性、履歴イベント、変更履歴）

**Independent Test**: 特定の資産を選択し、詳細画面に遷移して全情報が表示されることを確認

**FRs**: FR-007, FR-016
**SCs**: SC-003 (5回以内のキー操作)

---

### 🔹 Slice 3.1: 基本情報詳細表示

**Goal**: 一覧から資産を選択し、詳細画面で全属性を表示

**Tests**:
- [ ] T064 [S1] [US3] Contract test: AssetDetailScreen rendering in tests/asset_management/contract/asset_detail_screen_test.rs - test_displays_all_asset_info()

**Implementation**:

**Application Layer**:
- [ ] T065 [S1] [US3] Extend AssetService in src/asset_management/application/asset_service.rs: get_asset_detail(id) -> Result<AssetDto>

**Presentation Layer**:
- [ ] T066 [S1] [US3] Implement AssetDetailScreen in src/asset_management/presentation/screens/asset_detail.rs: render all fields (name, categories, manufacturer, quantity, memo, created_at, updated_at)
- [ ] T067 [S1] [US3] Extend App state machine in src/asset_management/presentation/app.rs: add Detail screen, transition from List (Enter key on selected row)

**Checkpoint**: ✅ **DEMO** - 一覧で資産選択 → Enter → 詳細画面表示（3タスク）

---

### 🔹 Slice 3.2: 変更履歴タブ表示

**Goal**: 詳細画面で変更履歴を表示（タブ切り替え）

**Tests**:
- [ ] T068 [S2] [US3] Contract test: ChangeHistory display in tests/asset_management/contract/asset_detail_screen_test.rs - test_displays_change_history()

**Implementation**:

**Application Layer**:
- [ ] T069 [S2] [US3] Add ChangeHistoryDto in src/asset_management/application/dto.rs
- [ ] T070 [S2] [US3] Extend AssetService in src/asset_management/application/asset_service.rs: get_change_history(asset_id) -> Result<Vec<ChangeHistoryDto>>

**Presentation Layer**:
- [ ] T071 [S2] [US3] Extend AssetDetailScreen with tab navigation in src/asset_management/presentation/screens/asset_detail.rs: Tab 1 (基本情報), Tab 2 (変更履歴), Tab key to switch
- [ ] T072 [S2] [US3] Render ChangeHistory list in AssetDetailScreen: display operation, timestamp, changes (JSON diff)

**Checkpoint**: ✅ **DEMO** - 詳細画面でTabキー → 変更履歴表示（4タスク累計54）

**Phase 5 Complete**: User Story 3 fully implemented - user can view asset details and change history

---

## Phase 6: User Story 4 - Edit Asset (Priority: P4)

**Goal**: 既存資産の編集機能（名前、メーカー、カテゴリ、メモ）

**Independent Test**: 既存の資産を編集し、変更が正しく保存されることを確認

**FRs**: FR-008, FR-012, FR-022, FR-033

---

### 🔹 Slice 4.1: 編集機能実装

**Goal**: 詳細画面から編集モードに入り、項目を変更して保存

**Tests**:
- [ ] T073 [S1] [US4] Unit test: Asset update in tests/asset_management/unit/domain/asset_entity_test.rs - test_update_name(), test_update_manufacturer()
- [ ] T074 [S1] [US4] Unit test: Update records ChangeHistory in tests/asset_management/unit/application/asset_service_test.rs - test_update_records_change_history()

**Implementation**:

**Domain Layer**:
- [ ] T075 [S1] [US4] Add update methods to Asset entity in src/asset_management/domain/asset/entity.rs: update_name(), update_manufacturer(), update_memo(), update_categories() (FR-033: カテゴリ追加・削除機能)

**Infrastructure Layer**:
- [ ] T076 [S1] [US4] Extend AssetRepository trait in src/asset_management/domain/asset/repository.rs: update(asset)
- [ ] T077 [S1] [US4] Implement update in SqliteAssetRepository in src/asset_management/infrastructure/persistence/asset_repository.rs: UPDATE assets SET ..., handle asset_categories junction table update

**Application Layer**:
- [ ] T078 [S1] [US4] Implement AssetService::update_asset in src/asset_management/application/asset_service.rs: update_asset(id, ...) with ChangeHistory recording

**Presentation Layer**:
- [ ] T079 [S1] [US4] Extend AssetFormScreen with Edit mode in src/asset_management/presentation/screens/asset_form.rs: load_existing_asset(), pre-populate fields, update_asset() on save (FR-033: 編集時のカテゴリ追加・削除UI)
- [ ] T080 [S1] [US4] Add edit transition from AssetDetailScreen in src/asset_management/presentation/screens/asset_detail.rs: E key to enter edit mode

**Checkpoint**: ✅ **DEMO** - 詳細画面でEキー → 編集モード → 変更 → 保存（8タスク）

**Phase 6 Complete**: User Story 4 fully implemented - user can edit existing assets

---

## Phase 7: User Story 5 - Delete Asset (Priority: P5)

**Goal**: 資産の削除機能（確認ダイアログ付き）

**Independent Test**: 資産を削除し、一覧から消えることを確認

**FRs**: FR-009

---

### 🔹 Slice 5.1: 削除機能実装

**Goal**: 詳細画面または一覧画面から資産を削除（確認ダイアログ表示）

**Tests**:
- [ ] T081 [S1] [US5] Unit test: Delete records ChangeHistory in tests/asset_management/unit/application/asset_service_test.rs - test_delete_records_change_history()

**Implementation**:

**Infrastructure Layer**:
- [ ] T082 [S1] [US5] Extend AssetRepository trait in src/asset_management/domain/asset/repository.rs: delete(id)
- [ ] T083 [S1] [US5] Implement delete in SqliteAssetRepository in src/asset_management/infrastructure/persistence/asset_repository.rs: DELETE with CASCADE handling

**Application Layer**:
- [ ] T084 [S1] [US5] Implement AssetService::delete_asset in src/asset_management/application/asset_service.rs: delete_asset(id) with ChangeHistory recording

**Presentation Layer**:
- [ ] T085 [S1] [US5] Extend DialogComponent with confirmation dialog in src/asset_management/presentation/components/dialog.rs: yes/no confirmation
- [ ] T086 [S1] [US5] Add delete action to AssetDetailScreen/AssetListScreen in src/asset_management/presentation/screens/: D key → confirmation dialog → delete on Yes

**Checkpoint**: ✅ **DEMO** - Dキー → 確認ダイアログ → Yes → 一覧から削除（5タスク累計67）

**Phase 7 Complete**: User Story 5 fully implemented - user can delete assets with confirmation

---

## Phase 8: User Story 6 - Record Asset History Events (Priority: P6)

**Goal**: 資産の履歴イベント記録（取得、処分、破損、紛失、数量調整）と価格・場所・URL

**Independent Test**: 資産に各種イベントを記録し、履歴として確認できることを検証

**FRs**: FR-021, FR-024, FR-025
**SCs**: SC-007 (20秒以内), SC-008 (10,000件を3秒以内)

---

### 🔹 Slice 6.1: 取得イベント記録

**Goal**: 資産に取得イベント（購入）を記録（日付、数量、価格、場所、URL）

**Tests**:
- [ ] T087 [S1] [US6] Unit test: AssetHistory acquisition in tests/asset_management/unit/domain/asset_history_test.rs - test_record_acquisition()
- [ ] T088 [S1] [US6] Integration test: AssetHistory persistence in tests/asset_management/integration/asset_history_test.rs - test_save_acquisition_event()

**Implementation**:

**Domain Layer**:
- [ ] T089 [S1] [US6] Implement AssetHistory entity in src/asset_management/domain/history/asset_history.rs: AssetHistory struct, HistoryType enum (Acquisition, Disposal, Damage, Loss, Adjustment), record_acquisition()
- [ ] T090 [S1] [US6] Create AssetHistoryRepository trait in src/asset_management/domain/history/repository.rs: save(), find_by_asset(), find_first_acquisition()

**Infrastructure Layer**:
- [ ] T091 [S1] [US6] Implement SqliteAssetHistoryRepository in src/asset_management/infrastructure/persistence/history_repository.rs: save(), find_by_asset() - INSERT/SELECT for asset_histories table

**Application Layer**:
- [ ] T092 [S1] [US6] Implement HistoryService in src/asset_management/application/history_service.rs: record_acquisition(asset_id, date, quantity, price, location, url, memo)

**Presentation Layer**:
- [ ] T093 [S1] [US6] Implement HistoryFormScreen in src/asset_management/presentation/screens/history_form.rs: input fields (date, quantity, price, location, url, memo), save action
- [ ] T094 [S1] [US6] Add "Add History Event" action to AssetDetailScreen in src/asset_management/presentation/screens/asset_detail.rs: H key → HistoryFormScreen

**Checkpoint**: ✅ **DEMO** - 詳細画面でHキー → 取得イベント入力 → 保存（8タスク）

---

### 🔹 Slice 6.2: 処分・破損・紛失イベント追加

**Goal**: 取得以外のイベントタイプを記録できるようにする

**Implementation**:

**Domain Layer**:
- [ ] T095 [S2] [US6] Extend AssetHistory entity in src/asset_management/domain/history/asset_history.rs: record_disposal(), record_damage(), record_loss(), record_adjustment()

**Application Layer**:
- [ ] T096 [S2] [US6] Extend HistoryService in src/asset_management/application/history_service.rs: record_disposal(), record_damage(), record_loss(), record_adjustment()

**Presentation Layer**:
- [ ] T097 [S2] [US6] Extend HistoryFormScreen with event type selector in src/asset_management/presentation/screens/history_form.rs: dropdown for event type, conditional fields (price for disposal)

**Checkpoint**: ✅ **DEMO** - すべてのイベントタイプを記録可能（3タスク累計78）

---

### 🔹 Slice 6.3: 履歴イベント表示（詳細画面）

**Goal**: 詳細画面で資産の履歴イベントを時系列表示

**Implementation**:

**Application Layer**:
- [ ] T098 [S3] [US6] Add AssetHistoryDto in src/asset_management/application/dto.rs
- [ ] T099 [S3] [US6] Extend AssetService in src/asset_management/application/asset_service.rs: get_asset_histories(asset_id) -> Result<Vec<AssetHistoryDto>>

**Presentation Layer**:
- [ ] T100 [S3] [US6] Add AssetHistory tab to AssetDetailScreen in src/asset_management/presentation/screens/asset_detail.rs: Tab 3 (履歴イベント), display list with date, type, quantity_change, price, location

**Checkpoint**: ✅ **DEMO** - 詳細画面のTab 3で履歴イベント表示（3タスク累計81）

**Phase 8 Complete**: User Story 6 fully implemented - user can record and view asset history events

---

## Phase 9: User Story 7 - View Change History (Priority: P7)

**Goal**: 資産の変更履歴表示（誰が、いつ、何を変更したか）

**FRs**: FR-022, FR-023

**Note**: 基本的な ChangeHistory 表示は Slice 3.2で実装済み。このフェーズではフィルタリング機能を追加。

---

### 🔹 Slice 7.1: 変更履歴フィルタリング

**Goal**: 変更履歴を日付範囲、操作種類でフィルタリング

**Tests**:
- [ ] T101 [S1] [US7] Unit test: ChangeHistory filtering in tests/asset_management/unit/application/history_service_test.rs - test_filter_by_date_range(), test_filter_by_operation_type()

**Implementation**:

**Infrastructure Layer**:
- [ ] T102 [S1] [US7] Extend ChangeHistoryRepository in src/asset_management/domain/history/repository.rs: find_by_date_range(), find_by_operation_type()
- [ ] T103 [S1] [US7] Implement filtering in SqliteChangeHistoryRepository in src/asset_management/infrastructure/persistence/history_repository.rs: WHERE clauses for date range and operation

**Application Layer**:
- [ ] T104 [S1] [US7] Implement HistoryService::get_change_histories in src/asset_management/application/history_service.rs: filter_changes(asset_id, date_from, date_to, operation_type)

**Presentation Layer**:
- [ ] T105 [S1] [US7] Add filter controls to ChangeHistory tab in AssetDetailScreen: date range picker, operation type dropdown

**Checkpoint**: ✅ **DEMO** - 変更履歴タブでフィルタリング（5タスク）

**Phase 9 Complete**: User Story 7 fully implemented - user can view and filter change history

---

## Phase 10: User Story 8 - View Ownership Duration (Priority: P8)

**Goal**: 資産の所有期間計算・表示（最初の取得から現在まで、または最終処分まで）

**FRs**: FR-025, FR-026
**SCs**: SC-009 (1秒以内に計算)

---

### 🔹 Slice 8.1: 所有期間計算ロジック実装

**Goal**: 取得イベントから所有期間を正しく計算

**Tests**:
- [ ] T106 [S1] [US8] Unit test: Duration calculation in tests/asset_management/unit/domain/services_test.rs - test_duration_for_active_asset(), test_duration_for_disposed_asset()

**Implementation**:

**Domain Layer**:
- [ ] T107 [S1] [US8] Implement proper OwnershipDurationService in src/asset_management/domain/shared/services.rs: calculate_duration(asset_id) using AssetHistoryRepository (replace placeholder from Slice 2.3)

**Application Layer**:
- [ ] T108 [S1] [US8] Update AssetService to use OwnershipDurationService in src/asset_management/application/asset_service.rs: populate ownership_duration in AssetDto

**Presentation Layer**:
- [ ] T109 [S1] [US8] Update AssetListScreen to display real ownership_duration in src/asset_management/presentation/screens/asset_list.rs: replace placeholder with calculated value
- [ ] T110 [S1] [US8] Display ownership duration in AssetDetailScreen in src/asset_management/presentation/screens/asset_detail.rs: show duration in days, highlight if disposed

**Checkpoint**: ✅ **DEMO** - 一覧・詳細で正しい所有期間表示（4タスク累計90）

**Phase 10 Complete**: User Story 8 fully implemented - ownership duration calculated and displayed

---

## Phase 11: User Story 9 - Manage Category Hierarchy (Priority: P9)

**Goal**: カテゴリの階層構造管理（ルート、親子、ツリー表示、削除制約）

**FRs**: FR-027, FR-028, FR-029
**SCs**: SC-010 (15秒以内), SC-011 (100階層を2秒以内)

---

### 🔹 Slice 9.1: カテゴリツリー表示

**Goal**: 階層構造を持つカテゴリをツリー表示

**Tests**:
- [ ] T111 [S1] [US9] Unit test: Category hierarchy in tests/asset_management/unit/domain/category_entity_test.rs - test_ancestors(), test_descendants()
- [ ] T112 [S1] [US9] Integration test: Hierarchy queries in tests/asset_management/integration/category_hierarchy_test.rs - test_recursive_cte()

**Implementation**:

**Domain Layer**:
- [ ] T113 [S1] [US9] Add hierarchy methods to Category entity in src/asset_management/domain/category/entity.rs: ancestors(), descendants(), is_ancestor_of()

**Infrastructure Layer**:
- [ ] T114 [S1] [US9] Extend CategoryRepository trait in src/asset_management/domain/category/repository.rs: find_ancestors(), find_descendants(), find_children()
- [ ] T115 [S1] [US9] Implement hierarchy queries in SqliteCategoryRepository in src/asset_management/infrastructure/persistence/category_repository.rs: recursive CTEs for ancestors/descendants

**Application Layer**:
- [ ] T116 [S1] [US9] Implement CategoryService in src/asset_management/application/category_service.rs: get_category_tree() -> Result<Vec<CategoryDto>>

**Presentation Layer**:
- [ ] T117 [S1] [US9] Implement TreeComponent in src/asset_management/presentation/components/tree.rs: reusable tree widget with expand/collapse, indentation
- [ ] T118 [S1] [US9] Implement CategoryTreeScreen in src/asset_management/presentation/screens/category_tree.rs: render tree using TreeComponent, navigation (arrow keys, expand/collapse)
- [ ] T119 [S1] [US9] Add CategoryTreeScreen to App state machine in src/asset_management/presentation/app.rs: C key from main menu

**Checkpoint**: ✅ **DEMO** - Cキー → カテゴリツリー表示、階層ナビゲーション（9タスク）

---

### 🔹 Slice 9.2: カテゴリ作成・編集・削除

**Goal**: カテゴリを追加、編集、削除できる（削除制約付き）

**Tests**:
- [ ] T120 [S2] [US9] Unit test: Cycle detection in tests/asset_management/unit/domain/category_entity_test.rs - test_validate_no_cycle()

**Implementation**:

**Domain Layer**:
- [ ] T121 [S2] [US9] Implement CategoryHierarchyService in src/asset_management/domain/shared/services.rs: validate_no_cycle(), get_breadcrumb()

**Infrastructure Layer**:
- [ ] T122 [S2] [US9] Extend CategoryRepository trait in src/asset_management/domain/category/repository.rs: has_children(), has_assets()
- [ ] T123 [S2] [US9] Implement deletion checks in SqliteCategoryRepository: check for children/assets before delete

**Application Layer**:
- [ ] T124 [S2] [US9] Extend CategoryService in src/asset_management/application/category_service.rs: create_category(), update_category(), delete_category() with validation

**Presentation Layer**:
- [ ] T125 [S2] [US9] Add category management actions to CategoryTreeScreen in src/asset_management/presentation/screens/category_tree.rs: N (new), E (edit), D (delete) keys, form dialogs

**Checkpoint**: ✅ **DEMO** - カテゴリツリーでNキー → 新規作成、Dキー → 削除（制約チェック）（5タスク累計104）

**Phase 11 Complete**: User Story 9 fully implemented - category hierarchy management

---

## Phase 12: User Story 10 - Filter Assets by Category Hierarchy (Priority: P10)

**Goal**: カテゴリ階層を利用したフィルタリング（親カテゴリ選択で子孫も表示）

**FRs**: FR-030, FR-031, FR-032
**SCs**: SC-012 (1秒以内)

---

### 🔹 Slice 10.1: 階層フィルタリング実装

**Goal**: 親カテゴリ選択時に子孫カテゴリの資産も含めて表示

**Tests**:
- [ ] T126 [S1] [US10] Integration test: Hierarchical filtering in tests/asset_management/integration/asset_query_test.rs - test_filter_by_parent_includes_descendants()

**Implementation**:

**Infrastructure Layer**:
- [ ] T127 [S1] [US10] Extend AssetRepository trait in src/asset_management/domain/asset/repository.rs: find_by_category_with_descendants(category_id, include_descendants: bool)
- [ ] T128 [S1] [US10] Implement hierarchical query in SqliteAssetRepository in src/asset_management/infrastructure/persistence/asset_repository.rs: JOIN with recursive CTE for descendants

**Application Layer**:
- [ ] T129 [S1] [US10] Extend AssetService in src/asset_management/application/asset_service.rs: filter_by_category_hierarchy(category_id, include_descendants)

**Presentation Layer**:
- [ ] T130 [S1] [US10] Add "include descendants" checkbox to filter dialog in AssetListScreen in src/asset_management/presentation/screens/asset_list.rs: toggle for hierarchical filtering

**Checkpoint**: ✅ **DEMO** - フィルタダイアログで"include descendants"チェック → 親+子孫の資産表示（4タスク）

---

### 🔹 Slice 10.2: パンくずリスト表示

**Goal**: 資産一覧・詳細でカテゴリのパンくずリスト表示

**Implementation**:

**Application Layer**:
- [ ] T131 [S2] [US10] Use CategoryHierarchyService::get_breadcrumb in src/asset_management/application/asset_service.rs: populate breadcrumb in AssetDto

**Presentation Layer**:
- [ ] T132 [S2] [US10] Display breadcrumb in AssetListScreen in src/asset_management/presentation/screens/asset_list.rs: "家電 > オーディオ機器 > イヤホン" format
- [ ] T133 [S2] [US10] Display breadcrumb in AssetDetailScreen in src/asset_management/presentation/screens/asset_detail.rs: show full path for each category

**Checkpoint**: ✅ **DEMO** - 一覧・詳細にパンくずリスト表示（3タスク累計111）

**Phase 12 Complete**: User Story 10 fully implemented - hierarchical filtering and breadcrumb navigation

---

## Phase 13: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T134 [P] Implement Markdown export service in src/asset_management/infrastructure/export/markdown.rs: export_all_assets(), generate_markdown_table()
- [ ] T135 [P] Add export command to main.rs: `lazybook export --output assets.md`
- [ ] T136 [P] Implement HelpScreen in src/asset_management/presentation/screens/help.rs with keybinding reference table
- [ ] T137 [P] Add terminal size validation in src/asset_management/presentation/app.rs: check minimum 80x24, display warning
- [ ] T138 [P] Implement dynamic layout adjustment for terminal resize in src/asset_management/presentation/app.rs
- [ ] T139 [P] Add comprehensive error handling for all database operations (wrap with anyhow context)
- [ ] T140 [P] Implement graceful shutdown on Ctrl+C in main.rs
- [ ] T141 Code review and refactoring: extract common patterns, reduce duplication
- [ ] T142 [P] Performance optimization: add database indices, query optimization for SC validation
- [ ] T143 [P] Security audit: verify no sensitive data logging (Constitution compliance)
- [ ] T144 [P] Documentation: update README.md with installation, usage, screenshots
- [ ] T145 Run quickstart.md validation: verify all setup steps work end-to-end
- [ ] T146 [P] Run all tests: `cargo test --all-features` and ensure 100% pass
- [ ] T147 Run clippy and fix all warnings: `cargo clippy -- -D warnings`
- [ ] T148 Format all code: `cargo fmt --all`
- [ ] T149 Verify Success Criteria: manually test SC-001 through SC-014 from spec.md

**Checkpoint**: Project is polished, tested, and ready for release!

---

## Implementation Strategy

### 🎯 MVP First (Vertical Slice through US1-3)

```
Week 1:
  Day 1: Phase 1 (Setup) + Phase 2 (Foundational)
  Day 2-3: Phase 3 Slice 1.1 (Walking Skeleton)
    → ✅ DEMO: 名前のみ資産登録
  Day 4-5: Phase 3 Slice 1.2 (カテゴリ)
    → ✅ DEMO: カテゴリ付き資産登録

Week 2:
  Day 1: Phase 3 Slice 1.3 (全項目) + 1.4 (変更履歴)
    → ✅ US1 完了
  Day 2-3: Phase 4 (US2 - フィルタ・ソート)
    → ✅ US2 完了
  Day 4: Phase 5 (US3 - 詳細表示)
    → ✅ MVP完了 (US1-3)

Week 3-4: US4-10追加
```

### 🚀 Incremental Delivery

1. **MVP (US1-3)**: 資産登録・一覧表示・詳細確認 → Deploy v0.1.0-alpha
2. **Edit/Delete (US4-5)**: 編集・削除機能 → Deploy v0.1.0-beta
3. **History Tracking (US6-8)**: 履歴管理・所有期間 → Deploy v0.1.0-rc1
4. **Category Management (US9-10)**: カテゴリ階層管理 → Deploy v0.1.0-rc2
5. **Polish (Phase 13)**: 最終調整 → Deploy v0.1.0 (stable)

---

## Vertical Slice Summary

| Phase | User Story | Slices | Total Tasks | First Working Demo |
|-------|-----------|--------|-------------|-------------------|
| 3 | US1: Add New Asset | 4 slices | 39 tasks | After Slice 1.1 (10 tasks) |
| 4 | US2: View Asset List | 3 slices | 17 tasks | Already working from US1 |
| 5 | US3: View Asset Details | 2 slices | 9 tasks | After Slice 3.1 (3 tasks) |
| 6 | US4: Edit Asset | 1 slice | 8 tasks | After slice (8 tasks) |
| 7 | US5: Delete Asset | 1 slice | 5 tasks | After slice (5 tasks) |
| 8 | US6: Record History Events | 3 slices | 14 tasks | After Slice 6.1 (8 tasks) |
| 9 | US7: View Change History | 1 slice | 5 tasks | After slice (5 tasks) |
| 10 | US8: View Ownership Duration | 1 slice | 4 tasks | After slice (4 tasks) |
| 11 | US9: Manage Category Hierarchy | 2 slices | 14 tasks | After Slice 9.1 (9 tasks) |
| 12 | US10: Filter by Hierarchy | 2 slices | 7 tasks | After Slice 10.1 (4 tasks) |
| 13 | Polish | - | 16 tasks | - |
| **Total** | **10 User Stories** | **20 Slices** | **164 tasks** | **9 demo points** |

**Key Achievement**: First working demo after only **10 tasks** (Setup + Foundational + Slice 1.1 + quality checks)

**Note**: Each slice includes `cargo fmt` and `cargo clippy` tasks for code quality assurance.

---

## TDD Workflow Reminder

For EVERY slice:

1. **RED**: Write integration test first (ensure it FAILS)
   ```bash
   cargo test slice_1_1_minimal_asset -- --nocapture
   # Expected: test failed (function not implemented)
   ```

2. **GREEN**: Implement minimum code to pass (all layers)
   ```rust
   // Domain → Infrastructure → Application → Presentation
   // Implement in sequence, test after each layer
   ```

3. **REFACTOR**: Clean up, optimize, extract patterns
   ```bash
   # Code quality checks (required after each slice)
   cargo fmt --all                    # Format all code
   cargo clippy -- -D warnings        # Check code quality (warnings as errors)
   cargo test                         # Run tests
   ```

4. **DEMO**: Run application and verify slice works end-to-end
   ```bash
   cargo run
   # Manually test the new feature
   ```

---

## Notes

- **[S#]** label maps task to specific slice within user story
- Each slice delivers **working, demoable feature** through all layers
- Avoid implementing all domain entities first (Horizontal Layering)
- Prefer small, frequent demos over big-bang integration
- Stop at any checkpoint to validate slice independently
- Commit after each slice completion for clean git history

## Code Quality Workflow

**After each slice implementation**:
1. Run `cargo fmt --all` to format all code
2. Run `cargo clippy -- -D warnings` to check code quality (warnings as errors)
3. Fix any warnings or formatting issues
4. Run `cargo test` to ensure all tests pass
5. Commit changes

**Quality checks are included as tasks** at key milestones:
- Phase 1 completion (T008a, T008b)
- Phase 2 completion (T014a, T014b)
- Each slice completion (T022a/b, T033a/b, T041a/b, T046a/b/c)
- Final polish phase (T147, T148)
