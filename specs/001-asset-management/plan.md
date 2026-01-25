# Implementation Plan: Asset Management

**Branch**: `001-asset-management` | **Date**: 2026-01-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-asset-management/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

個人の所有物（資産）を管理するTUIアプリケーション。資産のCRUD操作、階層的カテゴリ管理、履歴イベント追跡（取得・処分・破損等）、変更監査ログを提供する。SQLiteをマスタデータストアとし、Markdownエクスポート機能を持つ。Domain-Driven Designに基づき、Asset Management Bounded Contextとして実装する。

## Technical Context

**Language/Version**: Rust (stable toolchain - 1.75 or later)  
**Primary Dependencies**: 
- TUI framework: ratatui@0.30 (実装済み、research.mdで選定完了)
- SQLite access: rusqlite@0.38 (bundled feature, 実装済み、research.mdで選定完了)
- CLI parsing: clap@4.4 (derive feature, 実装済み、research.mdで選定完了)
- Date/time handling: chrono@0.4 (実装済み)
- Error handling: thiserror@2.0 (domain layer, 実装済み) + anyhow@1.0 (application/infrastructure layer, 実装済み)

**Storage**: SQLite (local file, no encryption at application level)  
**Transaction Management**: 
- マイグレーション: 各マイグレーションをトランザクション内で実行（rusqliteの`transaction()`を使用）
- Repository操作: 複数テーブルへの操作（Asset + AssetCategory + ChangeHistory等）を同一トランザクション内で実行
- エラー時は自動ロールバック（TransactionのDrop実装）
- ロールバック機能自体は安定バージョンリリース後に検討（現時点では自動ロールバックで対応）
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

**Option A: Bounded Context を最上位に配置**

```text
src/
├── main.rs              # Entry point, CLI setup, context routing
├── lib.rs               # Library root, shared utilities
│
├── asset_management/    # Asset Management Bounded Context
│   ├── mod.rs           # Context public interface
│   │
│   ├── domain/          # Domain layer (pure business logic)
│   │   ├── mod.rs
│   │   ├── asset/       # Asset aggregate
│   │   │   ├── mod.rs
│   │   │   ├── entity.rs    # Asset entity
│   │   │   ├── value_objects.rs  # Price, Duration, etc.
│   │   │   └── repository.rs     # Asset repository trait
│   │   ├── category/    # Category aggregate
│   │   │   ├── mod.rs
│   │   │   ├── entity.rs    # Category entity, hierarchy logic
│   │   │   └── repository.rs
│   │   ├── history/     # History aggregates
│   │   │   ├── mod.rs
│   │   │   ├── asset_history.rs    # AssetHistory entity
│   │   │   ├── change_history.rs   # ChangeHistory entity
│   │   │   └── repository.rs
│   │   └── shared/      # Shared domain concepts (within context)
│   │       ├── mod.rs
│   │       └── types.rs # Common value objects
│   │
│   ├── application/     # Application services (use cases)
│   │   ├── mod.rs
│   │   ├── asset_service.rs      # Asset CRUD, lifecycle
│   │   ├── category_service.rs   # Category management
│   │   ├── history_service.rs    # History query, export
│   │   └── dto.rs                # Data transfer objects
│   │
│   ├── infrastructure/  # Infrastructure layer
│   │   ├── mod.rs
│   │   ├── persistence/ # SQLite implementation
│   │   │   ├── mod.rs
│   │   │   ├── schema.rs    # Table definitions
│   │   │   ├── asset_repository.rs
│   │   │   ├── category_repository.rs
│   │   │   ├── history_repository.rs
│   │   │   └── migrations.rs
│   │   └── export/      # Markdown exporter
│   │       ├── mod.rs
│   │       └── markdown.rs
│   │
│   └── presentation/    # TUI layer
│       ├── mod.rs
│       ├── app.rs       # App state, event loop
│       ├── events.rs    # Event abstraction (keyboard + future mouse)
│       ├── screens/     # Screen components
│       │   ├── mod.rs
│       │   ├── asset_list.rs
│       │   ├── asset_detail.rs
│       │   ├── asset_form.rs
│       │   ├── category_tree.rs
│       │   ├── history_view.rs
│       │   └── help.rs
│       ├── components/  # Reusable TUI widgets
│       │   ├── mod.rs
│       │   ├── table.rs
│       │   ├── form.rs
│       │   ├── tree.rs
│       │   └── dialog.rs
│       └── navigation.rs # Navigation state machine
│
├── shared/              # Cross-Context shared kernel (if needed)
│   ├── mod.rs
│   ├── logging/         # Shared logging infrastructure
│   │   ├── mod.rs
│   │   └── logger.rs
│   └── types.rs         # Common types across all contexts
│
└── future_contexts/     # Placeholder for future bounded contexts
    ├── finance_management/  # Example: Future context
    └── knowledge_management/ # Example: Future context

tests/
├── asset_management/    # Asset Management context tests
│   ├── unit/            # Unit tests (per module)
│   │   ├── domain/
│   │   ├── application/
│   │   └── infrastructure/
│   ├── integration/     # Cross-layer integration tests
│   │   ├── asset_lifecycle_test.rs
│   │   ├── category_hierarchy_test.rs
│   │   └── history_tracking_test.rs
│   └── contract/        # TUI component contract tests
│       ├── asset_list_test.rs
│       └── category_tree_test.rs
│
└── integration_cross_context/  # Future: Cross-context integration tests
```

**Structure Decision**: Option A（Bounded Context最上位）を採用。Constitution Principle VIII（Bounded Context Architecture）に基づき、各Bounded Contextを最上位モジュールとして独立させる。

**理由**:
- ✅ **Context境界の明確化**: `src/asset_management/` で物理的に分離
- ✅ **スケーラビリティ**: 将来の `finance_management`, `knowledge_management` を並列追加可能
- ✅ **独立開発**: 各Contextが独自のdomain/application/infrastructure/presentationを持つ
- ✅ **依存関係の可視化**: Context間の依存は `mod.rs` のpub use で明示
- ✅ **テスト分離**: `tests/asset_management/` でContext単位のテスト管理

**Context間の通信**（将来）:
- Shared Kernel: `src/shared/` で最小限の共通機能（logging, 共通型）
- Context間統合: イベント駆動またはApplication Service経由
- 明示的依存: `src/asset_management/mod.rs` で他Contextへの依存を宣言

**main.rs の役割**:
```rust
// Context routing
mod asset_management;
mod shared;

fn main() -> Result<()> {
    // Initialize shared infrastructure
    shared::logging::init()?;
    
    // Launch Asset Management context
    asset_management::presentation::run()?;
    
    // Future: Context selection menu
    // match select_context() {
    //     Context::AssetManagement => asset_management::run()?,
    //     Context::FinanceManagement => finance_management::run()?,
    // }
    
    Ok(())
}
```

## Complexity Tracking

> **No violations detected.** All constitution requirements are met or explicitly documented.

## Implementation Considerations

### Transaction Management

**現状**: トランザクション管理は未実装。rusqliteは完全にサポートしていることを確認済み。

**検討事項**:

1. **マイグレーションのトランザクション管理** (Phase 2 T012):
   - 各マイグレーションをトランザクション内で実行
   - `schema_version`への記録も同一トランザクション内で実行
   - エラー時は自動ロールバック（TransactionのDrop実装）
   - **実装方針**: `rusqlite::Connection::transaction()`を使用

2. **Repository操作のトランザクション管理** (Phase 3以降):
   - Asset作成時: Asset + AssetCategory + ChangeHistory を同一トランザクション内で実行
   - Asset更新時: Asset + AssetCategory + ChangeHistory を同一トランザクション内で実行
   - Asset削除時: Asset + 関連履歴 + ChangeHistory を同一トランザクション内で実行
   - **実装方針**: Repository層でトランザクション管理、Application層からは意識しない設計

3. **ロールバック機能**:
   - 現時点では自動ロールバック（TransactionのDrop実装）で対応
   - 明示的なロールバック機能は安定バージョンリリース後に検討

**参考**: data-model.md の "Transaction Boundary" セクション参照
