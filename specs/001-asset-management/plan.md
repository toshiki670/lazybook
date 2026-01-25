# Implementation Plan: Asset Management

**Branch**: `001-asset-management` | **Date**: 2026-01-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-asset-management/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

個人の所有物（資産）を管理するTUIアプリケーション。資産のCRUD操作、階層的カテゴリ管理、履歴イベント追跡（取得・処分・破損等）、変更監査ログを提供する。SQLiteをマスタデータストアとし、Markdownエクスポート機能を持つ。Domain-Driven Designに基づき、Asset Management Bounded Contextとして実装する。

## Technical Context

**Language/Version**: Rust (stable toolchain - 1.75 or later)  
**Primary Dependencies**: 
- TUI framework: NEEDS CLARIFICATION (ratatui vs cursive vs tui-rs)
- SQLite access: NEEDS CLARIFICATION (rusqlite vs diesel vs sqlx)
- CLI parsing: NEEDS CLARIFICATION (clap vs structopt)
- Date/time handling: chrono
- Error handling: anyhow or thiserror

**Storage**: SQLite (local file, no encryption at application level)  
**Testing**: cargo test (unit tests), integration tests for domain logic, contract tests for TUI components  
**Target Platform**: macOS, Linux (terminal environments with UTF-8 support)  
**Project Type**: Single Rust project with TUI binary  
**Performance Goals**: 
- Asset list rendering: <1s for 100 assets (SC-002)
- Database query: <3s for 10,000 history events (SC-008)
- App startup: <3s with 1,000 assets (SC-006)
- Terminal resize response: <0.5s (SC-013)

**Constraints**: 
- Keyboard-only navigation (no mouse required)
- Minimum terminal size: 80 columns × 24 rows
- Single-user, local-only (no network)
- Japanese-only UI

**Scale/Scope**: 
- Expected data: 100-300 assets initially, up to 1,000 assets over 5 years
- 10 user stories, 34 functional requirements
- 5 core entities (Asset, Category, AssetCategory, AssetHistory, ChangeHistory)
- TUI screens: 6-8 screens (list, detail, edit, history, category management)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Initial Check (Before Phase 0)

#### Principle I: Specification-First Development
✅ **PASS**: Feature specification完備 (spec.md 380行、10 user stories、34 FRs、14 SCs、明確化質問14個回答済み)

#### Principle II: Modular Planning
⏳ **PENDING**: Phase 0 (research.md), Phase 1 (data-model.md, contracts/, quickstart.md) を本プランで生成予定

#### Principle III: Independent User Stories
✅ **PASS**: 10個のユーザーストーリー、すべて独立してテスト可能、優先度付き (P1-P10)

#### Principle IV: Test-Driven Development
⏳ **PENDING**: テスト戦略は Phase 1 で定義、実装フェーズで TDD サイクルを適用

#### Principle V: Documentation as Code
✅ **PASS**: 全成果物を specs/001-asset-management/ に格納、バージョン管理下

#### Principle VI: Domain-Driven Design Architecture
✅ **PASS**: 
- 5つのエンティティ定義済み (Asset, Category, AssetCategory, AssetHistory, ChangeHistory)
- Bounded Context: Asset Management
- Value Objects候補: 価格、所有期間（計算値）
- Aggregates候補: Asset（root）+ AssetHistory + ChangeHistory

#### Principle VII: TUI-First Interface Design
✅ **PASS**: 
- TUI専用設計、キーボードナビゲーション必須
- 動的レイアウト対応 (FR-017, FR-018)
- 最小画面サイズ定義 (80x24)

#### Principle VIII: Bounded Context Architecture
✅ **PASS**: 
- Asset Management を独立した Bounded Context として実装
- 将来の Finance Management, Knowledge Management 等とは明確に分離
- Cross-context 統合は Out of Scope (v0.1.0)

#### Technology Stack Compliance
✅ **PASS**: 
- Language: Rust (stable)
- Interface: TUI
- Architecture: Domain-Driven Design
- Tool Management: mise で管理予定

#### Privacy and Security
⚠️ **DEVIATION DOCUMENTED**: 
- Constitution 要件「データを暗号化して保存」→ 明確化により「暗号化不要」に変更
- 根拠: 個人用ローカルツール、OS/ファイルシステムレベルのセキュリティに依存
- 記録: spec.md Clarifications, Assumptions で明記済み

**GATE STATUS (Initial)**: ✅ **PASS** (セキュリティ要件の緩和は明示的に文書化済み)

---

### Post-Phase 1 Re-check

#### Principle II: Modular Planning
✅ **PASS**: 
- Phase 0 完了: research.md 生成（技術選定、ベストプラクティス調査）
- Phase 1 完了: data-model.md, contracts/tui-api.md, quickstart.md 生成
- Phase 2: tasks.md は /speckit.tasks コマンドで生成予定

#### Principle IV: Test-Driven Development
✅ **PASS**: 
- テスト戦略定義済み（quickstart.md, research.md）
- Contract tests 定義済み（contracts/tui-api.md）
- TDD サイクル手順明記（quickstart.md）

#### Principle VI: Domain-Driven Design Architecture
✅ **PASS** (強化): 
- 完全なデータモデル定義（data-model.md）
- 5 Entities, 4 Value Objects, 2 Aggregates, 4 Repositories, 2 Domain Services
- SQLiteスキーマ定義、マイグレーション戦略
- Repository pattern（trait定義 + 実装分離）

#### Principle VII: TUI-First Interface Design
✅ **PASS** (強化):
- 6 Screens 定義済み（contracts/tui-api.md）
- 4 Reusable Components 定義済み
- キーバインディング一覧、画面遷移フロー明記
- DTO 設計完了

**FINAL GATE STATUS**: ✅ **PASS** - すべての Constitution 要件を満たしています

## Project Structure

### Documentation (this feature)

```text
specs/001-asset-management/
├── spec.md              # Feature specification (completed)
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (to be generated)
├── data-model.md        # Phase 1 output (to be generated)
├── quickstart.md        # Phase 1 output (to be generated)
├── contracts/           # Phase 1 output (to be generated)
│   └── tui-api.md       # TUI component contracts
├── checklists/          # Quality checklists
│   └── requirements.md  # Specification quality checklist
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── main.rs              # Entry point, CLI setup
├── lib.rs               # Library root
│
├── domain/              # Domain layer (pure business logic)
│   ├── mod.rs
│   ├── asset/           # Asset aggregate
│   │   ├── mod.rs
│   │   ├── entity.rs    # Asset entity
│   │   ├── value_objects.rs  # Price, Duration, etc.
│   │   └── repository.rs     # Asset repository trait
│   ├── category/        # Category aggregate
│   │   ├── mod.rs
│   │   ├── entity.rs    # Category entity, hierarchy logic
│   │   └── repository.rs
│   ├── history/         # History aggregates
│   │   ├── mod.rs
│   │   ├── asset_history.rs    # AssetHistory entity
│   │   ├── change_history.rs   # ChangeHistory entity
│   │   └── repository.rs
│   └── shared/          # Shared domain concepts
│       ├── mod.rs
│       └── types.rs     # Common value objects
│
├── application/         # Application services (use cases)
│   ├── mod.rs
│   ├── asset_service.rs      # Asset CRUD, lifecycle
│   ├── category_service.rs   # Category management
│   ├── history_service.rs    # History query, export
│   └── dto.rs                # Data transfer objects
│
├── infrastructure/      # Infrastructure layer
│   ├── mod.rs
│   ├── persistence/     # SQLite implementation
│   │   ├── mod.rs
│   │   ├── schema.rs    # Table definitions
│   │   ├── asset_repository.rs
│   │   ├── category_repository.rs
│   │   ├── history_repository.rs
│   │   └── migrations.rs
│   ├── export/          # Markdown exporter
│   │   ├── mod.rs
│   │   └── markdown.rs
│   └── logging/         # Operation logging
│       ├── mod.rs
│       └── logger.rs
│
└── presentation/        # TUI layer
    ├── mod.rs
    ├── app.rs           # App state, event loop
    ├── screens/         # Screen components
    │   ├── mod.rs
    │   ├── asset_list.rs
    │   ├── asset_detail.rs
    │   ├── asset_form.rs
    │   ├── category_tree.rs
    │   ├── history_view.rs
    │   └── help.rs
    ├── components/      # Reusable TUI widgets
    │   ├── mod.rs
    │   ├── table.rs
    │   ├── form.rs
    │   ├── tree.rs
    │   └── dialog.rs
    └── navigation.rs    # Navigation state machine

tests/
├── unit/                # Unit tests (per module)
│   ├── domain/
│   ├── application/
│   └── infrastructure/
├── integration/         # Cross-layer integration tests
│   ├── asset_lifecycle_test.rs
│   ├── category_hierarchy_test.rs
│   └── history_tracking_test.rs
└── contract/            # TUI component contract tests
    ├── asset_list_test.rs
    └── category_tree_test.rs
```

**Structure Decision**: Single Rust project (Option 1) selected. This is a standalone TUI application without separate frontend/backend concerns. Domain-Driven Design layering (domain → application → infrastructure → presentation) is enforced through module structure. The Asset Management Bounded Context is implemented within the `src/` directory structure. Future Bounded Contexts (e.g., Finance Management) will be added as peer modules under `src/` or as separate crates if isolation requirements increase.

## Complexity Tracking

> **No violations detected.** All constitution requirements are met or explicitly documented.
