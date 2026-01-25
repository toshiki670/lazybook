# Tasks: Asset Management

**Input**: Design documents from `/specs/001-asset-management/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Test tasks are included (TDD approach per Constitution Principle IV)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single Rust project**: `src/`, `tests/` at repository root
- Paths follow plan.md structure (Domain → Application → Infrastructure → Presentation layers)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Initialize Rust project with Cargo.toml using `cargo init --name lazybook`
- [ ] T002 [P] Add dependencies to Cargo.toml: ratatui@0.25, crossterm@0.27, rusqlite (bundled), clap (derive), thiserror, anyhow, chrono, log, env_logger, serde, serde_json
- [ ] T003 [P] Configure clippy and rustfmt settings in rustfmt.toml and clippy.toml
- [ ] T004 [P] Setup mise configuration in .mise.toml for Rust stable toolchain
- [ ] T005 Create directory structure: src/{domain,application,infrastructure,presentation}, tests/{unit,integration,contract}, migrations/
- [ ] T006 [P] Create module files: src/lib.rs, src/main.rs, and all mod.rs files per plan.md structure
- [ ] T007 [P] Create initial migration 001_initial_schema.sql in migrations/ with all 5 tables (assets, categories, asset_categories, asset_histories, change_histories, schema_version)
- [ ] T008 [P] Create seed migration 002_seed_categories.sql with initial category hierarchy (家電, 衣類, 書籍, その他)

**Checkpoint**: Project structure complete, ready for foundational implementation

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T009 Implement Error types in src/domain/shared/types.rs: AssetError, CategoryError, PriceError using thiserror
- [ ] T010 [P] Implement Value Objects in src/domain/shared/types.rs: AssetId, CategoryId, Price, Duration with validation
- [ ] T011 [P] Setup logging infrastructure in src/infrastructure/logging/logger.rs using log + env_logger
- [ ] T012 [P] Implement database migration system in src/infrastructure/persistence/migrations.rs: apply_migrations(), check_schema_version()
- [ ] T013 Implement database connection manager in src/infrastructure/persistence/mod.rs: establish_connection(), initialize_database()
- [ ] T014 [P] Create Repository traits in src/domain/asset/repository.rs, src/domain/category/repository.rs, src/domain/history/repository.rs (interfaces only, no implementation)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Add New Asset (Priority: P1) 🎯 MVP

**Goal**: ユーザーが新しい資産を登録できる（名前、カテゴリ、メーカー）。取得イベントとして購入情報を記録できる。

**Independent Test**: 資産を1件登録し、登録した情報が正しく保存されていることを確認。

**FRs**: FR-001, FR-002, FR-010, FR-012, FR-014, FR-022
**SCs**: SC-001 (30秒以内に登録)

### Tests for User Story 1 (TDD - Write FIRST, ensure FAIL)

- [ ] T015 [P] [US1] Unit test: Asset entity creation in tests/unit/domain/asset_entity_test.rs - test_create_asset_with_valid_data(), test_asset_requires_name(), test_quantity_must_be_non_negative()
- [ ] T016 [P] [US1] Unit test: AssetService::create_asset() in tests/unit/application/asset_service_test.rs - test_create_asset_records_change_history()
- [ ] T017 [P] [US1] Integration test: Asset persistence in tests/integration/asset_lifecycle_test.rs - test_asset_creation_and_retrieval()

### Implementation for User Story 1

- [ ] T018 [P] [US1] Implement Asset entity in src/domain/asset/entity.rs: Asset struct, new(), update_name(), update_quantity(), validate()
- [ ] T019 [P] [US1] Implement Category entity in src/domain/category/entity.rs: Category struct, new(), set_parent(), validate_no_cycle()
- [ ] T020 [P] [US1] Implement AssetCategory entity in src/domain/asset/entity.rs (same file): AssetCategory struct, new()
- [ ] T021 [P] [US1] Implement ChangeHistory entity in src/domain/history/change_history.rs: ChangeHistory struct, record_create(), record_update(), to_json()
- [ ] T022 [US1] Implement SqliteAssetRepository in src/infrastructure/persistence/asset_repository.rs: save(), find(), find_all() (depends on T018)
- [ ] T023 [P] [US1] Implement SqliteCategoryRepository in src/infrastructure/persistence/category_repository.rs: save(), find(), find_all(), find_roots()
- [ ] T024 [US1] Implement SqliteChangeHistoryRepository in src/infrastructure/persistence/history_repository.rs: save(), find_by_asset() (depends on T021)
- [ ] T025 [US1] Implement AssetService in src/application/asset_service.rs: create_asset() with ChangeHistory recording (depends on T022, T024)
- [ ] T026 [US1] Implement AssetDto in src/application/dto.rs: AssetDto, CategoryDto conversion from domain entities
- [ ] T027 [US1] Implement AssetFormScreen (New mode) in src/presentation/screens/asset_form.rs: render(), handle_input(), validate_form(), save_asset()
- [ ] T028 [US1] Implement FormComponent in src/presentation/components/form.rs: reusable form widget with field validation
- [ ] T029 [US1] Implement App state machine in src/presentation/app.rs: Screen enum, transition_to(), handle_key_event()
- [ ] T030 [US1] Update main.rs to initialize database, apply migrations, and launch TUI app

**Checkpoint**: At this point, User Story 1 should be fully functional - user can create assets via TUI

---

## Phase 4: User Story 2 - View Asset List (Priority: P2)

**Goal**: 登録済み資産の一覧表示、フィルタリング、並び替え機能。

**Independent Test**: 事前に複数の資産を登録し、一覧表示・フィルタ・ソートが正しく動作することを確認。

**FRs**: FR-003, FR-004, FR-005, FR-006, FR-015
**SCs**: SC-002 (100件を1秒以内), SC-003 (5回以内のキー操作)

### Tests for User Story 2 (TDD)

- [ ] T031 [P] [US2] Unit test: Asset filtering logic in tests/unit/application/asset_service_test.rs - test_filter_by_category(), test_filter_by_manufacturer()
- [ ] T032 [P] [US2] Integration test: Asset list query performance in tests/integration/asset_query_test.rs - test_list_100_assets_under_1_second()
- [ ] T033 [P] [US2] Contract test: AssetListScreen rendering in tests/contract/asset_list_screen_test.rs - test_renders_asset_table(), test_highlights_selected_row()

### Implementation for User Story 2

- [ ] T034 [P] [US2] Add query methods to AssetRepository trait in src/domain/asset/repository.rs: find_by_category(), find_by_manufacturer(), count()
- [ ] T035 [US2] Implement query methods in SqliteAssetRepository in src/infrastructure/persistence/asset_repository.rs (depends on T034)
- [ ] T036 [P] [US2] Implement AssetService query methods in src/application/asset_service.rs: list_assets(), filter_by_category(), sort_by_field()
- [ ] T037 [P] [US2] Implement OwnershipDurationService in src/domain/shared/services.rs: calculate_duration() using AssetHistoryRepository
- [ ] T038 [US2] Implement AssetListScreen in src/presentation/screens/asset_list.rs: render(), handle_key_events(), apply_filter(), apply_sort() (depends on T036)
- [ ] T039 [P] [US2] Implement TableComponent in src/presentation/components/table.rs: reusable table widget with column定義, scroll, highlight
- [ ] T040 [US2] Add AssetListScreen to App state machine in src/presentation/app.rs (depends on T038)

**Checkpoint**: User can now view asset list with filtering/sorting (US1 + US2 both functional)

---

## Phase 5: User Story 3 - View Asset Details (Priority: P3)

**Goal**: 資産の詳細情報表示（すべての属性、履歴イベント、変更履歴）。

**Independent Test**: 特定の資産を選択し、詳細画面に遷移して全情報が表示されることを確認。

**FRs**: FR-007, FR-016
**SCs**: SC-003 (5回以内のキー操作)

### Tests for User Story 3 (TDD)

- [ ] T041 [P] [US3] Contract test: AssetDetailScreen rendering in tests/contract/asset_detail_screen_test.rs - test_displays_all_asset_info(), test_displays_history_events()

### Implementation for User Story 3

- [ ] T042 [P] [US3] Add AssetHistoryDto, ChangeHistoryDto to src/application/dto.rs
- [ ] T043 [US3] Extend AssetService in src/application/asset_service.rs: get_asset_detail() with histories (depends on T042)
- [ ] T044 [US3] Implement AssetDetailScreen in src/presentation/screens/asset_detail.rs: render(), handle_tab_switch(), display_histories()
- [ ] T045 [US3] Add AssetDetailScreen to App state machine transitions in src/presentation/app.rs

**Checkpoint**: User can now view detailed asset information (US1-3 all functional)

---

## Phase 6: User Story 4 - Edit Asset (Priority: P4)

**Goal**: 既存資産の編集機能（名前、メーカー、カテゴリ、メモ）。

**Independent Test**: 既存の資産を編集し、変更が正しく保存されることを確認。

**FRs**: FR-008, FR-012, FR-022
**SCs**: なし

### Tests for User Story 4 (TDD)

- [ ] T046 [P] [US4] Unit test: Asset entity update methods in tests/unit/domain/asset_entity_test.rs - test_update_name(), test_update_manufacturer()
- [ ] T047 [P] [US4] Unit test: AssetService::update_asset() in tests/unit/application/asset_service_test.rs - test_update_records_change_history()

### Implementation for User Story 4

- [ ] T048 [P] [US4] Add update methods to Asset entity in src/domain/asset/entity.rs: update_manufacturer(), update_memo()
- [ ] T049 [P] [US4] Add update methods to AssetRepository trait in src/domain/asset/repository.rs: update()
- [ ] T050 [US4] Implement update() in SqliteAssetRepository in src/infrastructure/persistence/asset_repository.rs (depends on T049)
- [ ] T051 [US4] Implement AssetService::update_asset() in src/application/asset_service.rs with ChangeHistory recording (depends on T048, T050)
- [ ] T052 [US4] Implement AssetFormScreen (Edit mode) in src/presentation/screens/asset_form.rs: load_existing_asset(), update_asset()
- [ ] T053 [US4] Add edit transition from AssetDetailScreen in src/presentation/screens/asset_detail.rs

**Checkpoint**: User can now edit existing assets (US1-4 all functional)

---

## Phase 7: User Story 5 - Delete Asset (Priority: P5)

**Goal**: 資産の削除機能（確認ダイアログ付き）。

**Independent Test**: 資産を削除し、一覧から消えることを確認。

**FRs**: FR-009
**SCs**: なし

### Tests for User Story 5 (TDD)

- [ ] T054 [P] [US5] Unit test: AssetService::delete_asset() in tests/unit/application/asset_service_test.rs - test_delete_records_change_history(), test_delete_removes_from_repository()

### Implementation for User Story 5

- [ ] T055 [P] [US5] Add delete() to AssetRepository trait in src/domain/asset/repository.rs
- [ ] T056 [US5] Implement delete() in SqliteAssetRepository in src/infrastructure/persistence/asset_repository.rs with CASCADE handling (depends on T055)
- [ ] T057 [US5] Implement AssetService::delete_asset() in src/application/asset_service.rs with ChangeHistory recording (depends on T056)
- [ ] T058 [P] [US5] Implement DialogComponent in src/presentation/components/dialog.rs: confirmation dialog widget
- [ ] T059 [US5] Add delete action to AssetDetailScreen/AssetListScreen in src/presentation/screens/ with confirmation dialog (depends on T058)

**Checkpoint**: User can now delete assets with confirmation (US1-5 all functional)

---

## Phase 8: User Story 6 - Record Asset History Events (Priority: P6)

**Goal**: 資産の履歴イベント記録（取得、処分、破損、紛失、数量調整）と価格・場所・URL。

**Independent Test**: 資産に各種イベントを記録し、履歴として確認できることを検証。

**FRs**: FR-021, FR-024, FR-025
**SCs**: SC-007 (20秒以内), SC-008 (10,000件を3秒以内)

### Tests for User Story 6 (TDD)

- [ ] T060 [P] [US6] Unit test: AssetHistory entity in tests/unit/domain/asset_history_test.rs - test_record_acquisition(), test_record_disposal_with_price()
- [ ] T061 [P] [US6] Integration test: History persistence in tests/integration/asset_history_test.rs - test_multiple_acquisitions_tracked_separately()

### Implementation for User Story 6

- [ ] T062 [P] [US6] Implement AssetHistory entity in src/domain/history/asset_history.rs: AssetHistory struct, HistoryType enum, record_acquisition(), record_disposal(), record_damage()
- [ ] T063 [P] [US6] Implement AssetHistoryRepository trait in src/domain/history/repository.rs: save(), find_by_asset(), find_first_acquisition(), find_last_disposal()
- [ ] T064 [US6] Implement SqliteAssetHistoryRepository in src/infrastructure/persistence/history_repository.rs (depends on T063)
- [ ] T065 [US6] Implement HistoryService in src/application/history_service.rs: record_acquisition(), record_disposal(), get_asset_histories() (depends on T064)
- [ ] T066 [US6] Implement HistoryFormScreen in src/presentation/screens/history_form.rs: render(), handle_input(), save_history()
- [ ] T067 [US6] Add history recording action to AssetDetailScreen in src/presentation/screens/asset_detail.rs

**Checkpoint**: User can now record and view asset history events (US1-6 all functional)

---

## Phase 9: User Story 7 - View Change History (Priority: P7)

**Goal**: 資産の変更履歴表示（誰が、いつ、何を変更したか）。

**Independent Test**: 資産を作成・編集・削除し、すべての操作が履歴として表示されることを確認。

**FRs**: FR-022, FR-023
**SCs**: なし

### Tests for User Story 7 (TDD)

- [ ] T068 [P] [US7] Unit test: ChangeHistory filtering in tests/unit/application/history_service_test.rs - test_filter_by_date_range(), test_filter_by_operation_type()

### Implementation for User Story 7

- [ ] T069 [P] [US7] Add filtering methods to ChangeHistoryRepository trait in src/domain/history/repository.rs: find_by_date_range(), find_by_operation_type()
- [ ] T070 [US7] Implement filtering in SqliteChangeHistoryRepository in src/infrastructure/persistence/history_repository.rs (depends on T069)
- [ ] T071 [US7] Extend HistoryService in src/application/history_service.rs: get_change_histories(), filter_changes()
- [ ] T072 [US7] Add ChangeHistory display tab to AssetDetailScreen in src/presentation/screens/asset_detail.rs
- [ ] T073 [P] [US7] Implement HistoryViewScreen (optional separate screen) in src/presentation/screens/history_view.rs if needed

**Checkpoint**: User can now view complete change history (US1-7 all functional)

---

## Phase 10: User Story 8 - View Ownership Duration (Priority: P8)

**Goal**: 資産の所有期間計算・表示（最初の取得から現在まで、または最終処分まで）。

**Independent Test**: 取得イベントが記録された資産の所有期間が正しく計算されることを確認。

**FRs**: FR-025, FR-026
**SCs**: SC-009 (1秒以内に計算)

### Tests for User Story 8 (TDD)

- [ ] T074 [P] [US8] Unit test: Duration calculation in tests/unit/domain/services_test.rs - test_duration_for_active_asset(), test_duration_for_disposed_asset()

### Implementation for User Story 8

- [ ] T075 [US8] Implement Duration value object in src/domain/shared/types.rs: Duration struct, between(), from_now(), days() (may already exist from T010)
- [ ] T076 [US8] Enhance OwnershipDurationService in src/domain/shared/services.rs: calculate_duration() using first acquisition and last disposal (depends on T037, T075)
- [ ] T077 [US8] Display ownership duration in AssetListScreen in src/presentation/screens/asset_list.rs (add column)
- [ ] T078 [US8] Display ownership duration in AssetDetailScreen in src/presentation/screens/asset_detail.rs

**Checkpoint**: Ownership duration is now calculated and displayed (US1-8 all functional)

---

## Phase 11: User Story 9 - Manage Category Hierarchy (Priority: P9)

**Goal**: カテゴリの階層構造管理（ルート、親子、ツリー表示、削除制約）。

**Independent Test**: ルートカテゴリと子カテゴリを作成し、階層構造が正しく保存・表示されることを確認。

**FRs**: FR-027, FR-028, FR-029
**SCs**: SC-010 (15秒以内), SC-011 (100階層を2秒以内)

### Tests for User Story 9 (TDD)

- [ ] T079 [P] [US9] Unit test: Category hierarchy methods in tests/unit/domain/category_entity_test.rs - test_ancestors(), test_descendants(), test_is_ancestor_of()
- [ ] T080 [P] [US9] Unit test: Cycle detection in tests/unit/domain/category_entity_test.rs - test_validate_no_cycle()
- [ ] T081 [P] [US9] Integration test: Hierarchy queries in tests/integration/category_hierarchy_test.rs - test_recursive_cte_ancestors(), test_recursive_cte_descendants()

### Implementation for User Story 9

- [ ] T082 [P] [US9] Implement hierarchy query methods in CategoryRepository trait in src/domain/category/repository.rs: find_ancestors(), find_descendants(), find_children(), has_children(), has_assets()
- [ ] T083 [US9] Implement hierarchy queries in SqliteCategoryRepository in src/infrastructure/persistence/category_repository.rs using recursive CTEs (depends on T082)
- [ ] T084 [P] [US9] Implement CategoryHierarchyService in src/domain/shared/services.rs: validate_no_cycle(), get_breadcrumb()
- [ ] T085 [US9] Implement CategoryService in src/application/category_service.rs: create_category(), update_category(), delete_category() with validation (depends on T084)
- [ ] T086 [US9] Implement CategoryTreeScreen in src/presentation/screens/category_tree.rs: render_tree(), expand_node(), collapse_node(), handle_navigation()
- [ ] T087 [P] [US9] Implement TreeComponent in src/presentation/components/tree.rs: reusable tree widget with indentation
- [ ] T088 [US9] Add CategoryTreeScreen to App state machine in src/presentation/app.rs

**Checkpoint**: User can now manage category hierarchy (US1-9 all functional)

---

## Phase 12: User Story 10 - Filter Assets by Category Hierarchy (Priority: P10)

**Goal**: カテゴリ階層を利用したフィルタリング（親カテゴリ選択で子孫も表示）。

**Independent Test**: 階層構造を持つカテゴリに資産を登録し、親カテゴリで検索した際に子孫カテゴリの資産も含まれることを確認。

**FRs**: FR-030, FR-031, FR-032
**SCs**: SC-012 (1秒以内)

### Tests for User Story 10 (TDD)

- [ ] T089 [P] [US10] Integration test: Hierarchical filtering in tests/integration/asset_query_test.rs - test_filter_by_parent_includes_descendants()

### Implementation for User Story 10

- [ ] T090 [US10] Add hierarchical filtering to AssetRepository trait in src/domain/asset/repository.rs: find_by_category_with_descendants()
- [ ] T091 [US10] Implement hierarchical query in SqliteAssetRepository in src/infrastructure/persistence/asset_repository.rs (depends on T090)
- [ ] T092 [US10] Extend AssetService in src/application/asset_service.rs: filter_by_category_hierarchy()
- [ ] T093 [US10] Add "include descendants" option to AssetListScreen filter dialog in src/presentation/screens/asset_list.rs
- [ ] T094 [US10] Display breadcrumb navigation in AssetListScreen and AssetDetailScreen in src/presentation/screens/

**Checkpoint**: All 10 user stories are now implemented and functional!

---

## Phase 13: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T095 [P] Implement Markdown export service in src/infrastructure/export/markdown.rs: export_all_assets(), generate_markdown_table()
- [ ] T096 [P] Add export command to main.rs: `lazybook export --output assets.md`
- [ ] T097 [P] Implement HelpScreen in src/presentation/screens/help.rs with keybinding reference table
- [ ] T098 [P] Add terminal size validation in src/presentation/app.rs: check minimum 80x24, display warning
- [ ] T099 [P] Implement dynamic layout adjustment for terminal resize in src/presentation/app.rs
- [ ] T100 [P] Add comprehensive error handling for all database operations (wrap with anyhow context)
- [ ] T101 [P] Implement graceful shutdown on Ctrl+C in main.rs
- [ ] T102 Code review and refactoring: extract common patterns, reduce duplication
- [ ] T103 [P] Performance optimization: add database indices, query optimization for SC validation
- [ ] T104 [P] Security audit: verify no sensitive data logging (Constitution compliance)
- [ ] T105 [P] Documentation: update README.md with installation, usage, screenshots
- [ ] T106 Run quickstart.md validation: verify all setup steps work end-to-end
- [ ] T107 [P] Run all tests: `cargo test --all-features` and ensure 100% pass
- [ ] T108 Run clippy and fix all warnings: `cargo clippy -- -D warnings`
- [ ] T109 Format all code: `cargo fmt --all`
- [ ] T110 Verify Success Criteria: manually test SC-001 through SC-014 from spec.md

**Checkpoint**: Project is polished, tested, and ready for release!

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-12)**: All depend on Foundational phase completion
  - User stories CAN proceed in parallel (if staffed)
  - OR sequentially in priority order: US1 → US2 → US3 → ... → US10
- **Polish (Phase 13)**: Depends on desired user stories being complete (minimum US1-3 for MVP)

### User Story Dependencies

- **US1 (P1)**: Can start after Foundational - No dependencies on other stories ✅ MVP CORE
- **US2 (P2)**: Can start after Foundational - Integrates with US1 (displays assets from US1) but independently testable
- **US3 (P3)**: Can start after Foundational - Depends on US2 (selects from list) but independently testable ✅ MVP COMPLETE (US1-3)
- **US4 (P4)**: Depends on US1, US3 (edits existing assets, transitions from detail screen)
- **US5 (P5)**: Depends on US1, US2 or US3 (deletes existing assets)
- **US6 (P6)**: Depends on US1, US3 (records history for existing assets, accessed from detail screen)
- **US7 (P7)**: Depends on US1, US3 (displays change history from detail screen)
- **US8 (P8)**: Depends on US2, US3, US6 (displays duration calculated from histories)
- **US9 (P9)**: Can start after Foundational - Mostly independent (category management)
- **US10 (P10)**: Depends on US2, US9 (hierarchical filtering requires category tree)

### Within Each User Story

- Tests MUST be written FIRST and FAIL before implementation (TDD)
- Domain entities before repositories
- Repositories before services
- Services before presentation screens
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- **Phase 1 (Setup)**: T002, T003, T004, T006, T007, T008 can all run in parallel
- **Phase 2 (Foundational)**: T010, T011, T012, T014 can run in parallel after T009
- **Within each User Story**: Tests can run in parallel, domain entities can run in parallel
- **Cross-Story Parallelism**: After Phase 2, multiple user stories can be worked on by different developers simultaneously

---

## Parallel Example: User Story 1

```bash
# Write all tests first (TDD):
Parallel: T015 (asset entity test), T016 (service test), T017 (integration test)

# Implement domain entities in parallel:
Parallel: T018 (Asset), T019 (Category), T020 (AssetCategory), T021 (ChangeHistory)

# Implement repositories in parallel:
Parallel: T022 (AssetRepository), T023 (CategoryRepository), T024 (ChangeHistoryRepository)
```

---

## Implementation Strategy

### MVP First (User Stories 1-3)

1. Complete Phase 1: Setup → **Project initialized**
2. Complete Phase 2: Foundational → **Infrastructure ready**
3. Complete Phase 3: User Story 1 → **Can create assets** ✅ MVP Alpha
4. Complete Phase 4: User Story 2 → **Can view asset list** ✅ MVP Beta
5. Complete Phase 5: User Story 3 → **Can view asset details** ✅ MVP Release
6. **STOP and VALIDATE**: Test MVP independently, demo to stakeholders
7. Deploy/release MVP (v0.1.0-alpha)

### Incremental Delivery

1. MVP (US1-3) → Deploy v0.1.0-alpha
2. Add US4-5 (Edit/Delete) → Deploy v0.1.0-beta
3. Add US6-8 (History/Duration) → Deploy v0.1.0-rc1
4. Add US9-10 (Category Management) → Deploy v0.1.0-rc2
5. Complete Phase 13 (Polish) → Deploy v0.1.0 (stable)

### Parallel Team Strategy

With 3 developers:

1. All: Complete Setup + Foundational together → **Foundation ready**
2. Parallel work (after Phase 2):
   - **Developer A**: US1 (Asset creation) - Critical path
   - **Developer B**: US9 (Category management) - Independent
   - **Developer C**: Setup tests, infrastructure improvements
3. After US1 complete:
   - **Developer A**: US2 (Asset list)
   - **Developer B**: US6 (History events)
   - **Developer C**: US3 (Asset details)
4. Continue in priority order with load balancing

---

## TDD Workflow Reminder

For EVERY implementation task:

1. **RED**: Write test first (ensure it FAILS)
   ```bash
   cargo test domain::asset::test_create_asset
   # Expected: test failed (function not implemented)
   ```

2. **GREEN**: Implement minimum code to pass
   ```rust
   impl Asset {
       pub fn new(name: String) -> Result<Self, AssetError> {
           // Minimal implementation
       }
   }
   ```
   ```bash
   cargo test domain::asset::test_create_asset
   # Expected: test passed
   ```

3. **REFACTOR**: Clean up, optimize, extract patterns
   ```bash
   cargo clippy -- -D warnings
   cargo fmt
   cargo test
   ```

---

## Success Criteria Validation

After completing all user stories, manually verify each Success Criteria from spec.md:

- **SC-001**: Time user creating new asset (target: <30s)
- **SC-002**: Benchmark list rendering with 100 assets (target: <1s)
- **SC-003**: Count key presses to reach asset detail (target: ≤5)
- **SC-004**: Test validation error messages for clarity
- **SC-005**: Verify data persistence reliability (100% success rate)
- **SC-006**: Measure app startup time with 1,000 assets (target: <3s)
- **SC-007**: Time history event recording (target: <20s)
- **SC-008**: Benchmark history query with 10,000 events (target: <3s)
- **SC-009**: Verify ownership duration calculation speed (target: <1s)
- **SC-010**: Time category creation (target: <15s)
- **SC-011**: Test category tree display with 100 levels (target: <2s)
- **SC-012**: Benchmark hierarchical filtering (target: <1s)
- **SC-013**: Test terminal resize response time (target: <0.5s)
- **SC-014**: Verify minimum size warning displays immediately

---

## Task Summary

**Total Tasks**: 110
- **Phase 1 (Setup)**: 8 tasks
- **Phase 2 (Foundational)**: 6 tasks (BLOCKS all user stories)
- **Phase 3 (US1 - MVP Core)**: 16 tasks (3 tests + 13 implementation)
- **Phase 4 (US2)**: 10 tasks (3 tests + 7 implementation)
- **Phase 5 (US3)**: 5 tasks (1 test + 4 implementation)
- **Phase 6 (US4)**: 8 tasks (2 tests + 6 implementation)
- **Phase 7 (US5)**: 5 tasks (1 test + 4 implementation)
- **Phase 8 (US6)**: 8 tasks (2 tests + 6 implementation)
- **Phase 9 (US7)**: 6 tasks (1 test + 5 implementation)
- **Phase 10 (US8)**: 5 tasks (1 test + 4 implementation)
- **Phase 11 (US9)**: 10 tasks (3 tests + 7 implementation)
- **Phase 12 (US10)**: 6 tasks (1 test + 5 implementation)
- **Phase 13 (Polish)**: 16 tasks

**Parallel Opportunities**: 45 tasks marked [P] can run in parallel within their phase

**Test Tasks**: 18 (following TDD approach as required by Constitution)

**MVP Scope (US1-3)**: 31 tasks (Setup + Foundational + US1 + US2 + US3)

---

## Notes

- All tasks follow strict format: `- [ ] [ID] [P?] [Story?] Description with file path`
- [P] tasks = different files, no dependencies, can run in parallel
- [Story] label maps task to specific user story for traceability (required for US tasks)
- Each user story is independently completable and testable (Constitution Principle III)
- Tests written FIRST, must FAIL before implementation (Constitution Principle IV - TDD)
- Commit after each task or logical group for clean git history
- Stop at any checkpoint to validate story independently
- MVP (US1-3) provides core value: create assets, view list, view details
