# Research: Asset Management

**Date**: 2026-01-24  
**Phase**: 0 - Technology Selection & Best Practices

## Overview

このドキュメントは、Asset Management機能の実装に必要な技術選定と設計パターンの調査結果をまとめます。Technical Contextで "NEEDS CLARIFICATION" とマークされた項目を解決し、実装方針を確定させます。

---

## 1. TUI Framework Selection

### Decision: **ratatui**

### Rationale

**ratatui** (Rust Asynchronous TUI, formerly tui-rs) を採用します。

**理由**:
- ✅ **アクティブなメンテナンス**: tui-rsの後継として活発に開発中
- ✅ **豊富なウィジェット**: Table, List, Tree, Block, Paragraph, Gauge等、必要な全コンポーネントが揃っている
- ✅ **カスタマイズ性**: レイアウト、スタイリング、イベントハンドリングが柔軟
- ✅ **非同期対応**: async/awaitと統合可能（将来の拡張性）
- ✅ **crossterm統合**: クロスプラットフォームのターミナル操作
- ✅ **実績**: cargo-watch, gitui, bottom等の主要TUIツールで採用
- ✅ **日本語対応**: UTF-8フル対応、CJK文字の幅計算も正確

**Alternatives Considered**:

| Framework | Pros | Cons | Rejected Because |
|-----------|------|------|------------------|
| **cursive** | 高レベルAPI、簡単な構築 | レイアウトの柔軟性が低い、カスタマイズが困難 | 複雑な階層表示（カテゴリツリー、履歴テーブル）に対応しづらい |
| **tui-rs** | 安定版、ドキュメント豊富 | メンテナンス停止（ratatuiに移行済み） | 後継のratatuiを使うべき |
| **termion** | 軽量、低レベル制御 | ウィジェットなし、全て自作が必要 | 開発コストが高すぎる |

### Implementation Notes

- **依存関係**: `ratatui = "0.25"`, `crossterm = "0.27"`
- **レイアウト戦略**: 
  - 一覧画面: `Table` widget + `Block` for borders
  - 詳細画面: `Paragraph` + `List` for history
  - カテゴリツリー: カスタム `Tree` component (ratatuiのListベース)
  - フォーム: カスタム `Form` component (入力フィールド、バリデーション表示)
- **イベントループ**: crossterm の event polling (60 FPS target)
- **画面遷移**: ステートマシンパターン（`Screen` enum + `App` struct）

---

## 2. SQLite Access Library Selection

### Decision: **rusqlite**

### Rationale

**rusqlite** を採用します（dieselやsqlxではなく）。

**理由**:
- ✅ **シンプル**: 低レベルAPIで直接SQL制御、DDD repositoryパターンと相性良好
- ✅ **軽量**: 依存関係が少ない、ビルドが速い
- ✅ **柔軟**: 複雑なクエリ（カテゴリ階層の再帰CTE、履歴イベントの集計）を素直に書ける
- ✅ **トランザクション制御**: 明示的なトランザクション境界（Aggregate保護に適している）
- ✅ **SQLite最適化**: SQLiteのFTS5、JSON拡張等を直接利用可能
- ✅ **実績**: ripgrep, alacritty等で採用

**Alternatives Considered**:

| Library | Pros | Cons | Rejected Because |
|---------|------|------|------------------|
| **diesel** | 型安全なクエリビルダー、マイグレーション管理 | 重い、複雑なクエリ（再帰CTE）が書きにくい、学習コスト高 | カテゴリ階層の祖先/子孫クエリがクエリビルダーで表現困難 |
| **sqlx** | 非同期、コンパイル時SQLチェック | 本プロジェクトは同期処理で十分、オーバーキル | TUIアプリでは非同期の恩恵が少ない |
| **sea-orm** | 高レベルORM、リレーション自動解決 | 複雑、DDD repositoryパターンとミスマッチ | Domain層の純粋性を保ちにくい |

### Implementation Notes

- **依存関係**: `rusqlite = { version = "0.30", features = ["bundled"] }`
  - `bundled` feature: SQLiteを静的リンク（システムライブラリ不要）
- **マイグレーション**: 手動SQL管理（`migrations/` ディレクトリ、起動時適用）
- **Repository実装**: trait定義はdomain層、rusqlite実装はinfrastructure層
- **接続管理**: `Connection` を `Arc<Mutex<Connection>>` でラップ（複数スレッド対応は将来拡張）
- **クエリ例**:
  ```sql
  -- カテゴリの祖先取得（再帰CTE）
  WITH RECURSIVE ancestors(id, name, parent_id, level) AS (
    SELECT id, name, parent_id, 0 FROM categories WHERE id = ?
    UNION ALL
    SELECT c.id, c.name, c.parent_id, a.level + 1
    FROM categories c JOIN ancestors a ON c.id = a.parent_id
  )
  SELECT * FROM ancestors ORDER BY level DESC;
  ```

---

## 3. CLI Parsing Library Selection

### Decision: **clap (v4)**

### Rationale

**clap v4** (Command Line Argument Parser) を採用します。

**理由**:
- ✅ **derive マクロ**: 構造体ベースの宣言的定義、型安全
- ✅ **サブコマンド**: 将来的に `lazybook asset list`, `lazybook export` 等の拡張が容易
- ✅ **ヘルプ自動生成**: `--help` の見やすい出力
- ✅ **バリデーション**: 引数の型チェック、範囲チェック
- ✅ **実績**: cargo, ripgrep, fd等で採用
- ✅ **日本語エラーメッセージ**: カスタマイズ可能

**Alternatives Considered**:

| Library | Pros | Cons | Rejected Because |
|---------|------|------|------------------|
| **structopt** | derive マクロ、シンプル | clap v3ベース、v4に統合済み | clap v4 が後継 |
| **argh** | 軽量、高速 | 機能が少ない、サブコマンドサポートが弱い | 将来の拡張性に不安 |

### Implementation Notes

- **依存関係**: `clap = { version = "4.4", features = ["derive"] }`
- **基本構造**:
  ```rust
  use clap::{Parser, Subcommand};

  #[derive(Parser)]
  #[command(name = "lazybook")]
  #[command(about = "個人情報管理ツール", long_about = None)]
  struct Cli {
      #[command(subcommand)]
      command: Option<Commands>,
  }

  #[derive(Subcommand)]
  enum Commands {
      /// 資産管理TUIを起動
      Asset,
      /// Markdownエクスポート
      Export {
          #[arg(short, long)]
          output: String,
      },
  }
  ```
- **初期バージョン**: サブコマンドなし、引数なしで直接TUI起動
- **将来拡張**: `lazybook asset`, `lazybook finance`, `lazybook export` 等

---

## 4. Error Handling Strategy

### Decision: **thiserror + anyhow**

### Rationale

**thiserror** (ドメインエラー型定義) + **anyhow** (アプリケーション層エラー伝搬) のハイブリッドアプローチを採用します。

**理由**:
- ✅ **thiserror**: ドメイン層の明示的エラー型（例: `AssetNotFound`, `InvalidQuantity`）
- ✅ **anyhow**: アプリケーション・インフラ層での簡潔なエラー伝搬（`Result<T>` のラッピング）
- ✅ **コンテキスト**: anyhowの `.context()` でエラーチェインを構築
- ✅ **実績**: Rustコミュニティのベストプラクティス

### Implementation Notes

- **依存関係**: `thiserror = "1.0"`, `anyhow = "1.0"`
- **Domain層**: thiserrorでエラー定義
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum AssetError {
      #[error("資産が見つかりません: {0}")]
      NotFound(i64),
      #[error("数量が不正です: {0}")]
      InvalidQuantity(i32),
      #[error("カテゴリが未選択です")]
      NoCategorySelected,
  }
  ```
- **Application/Infrastructure層**: anyhow::Result
  ```rust
  use anyhow::{Context, Result};

  pub fn load_asset(id: i64) -> Result<Asset> {
      repository.find(id)
          .context("資産の読み込みに失敗しました")?;
  }
  ```

---

## 5. Date/Time Handling

### Decision: **chrono**

### Rationale

**chrono** は Rustの標準的な日時ライブラリで、必要な全機能（日付演算、フォーマット、パース）を提供します。

**Implementation Notes**:
- **依存関係**: `chrono = "0.4"`
- **使用例**:
  - 所有期間計算: `(Utc::now().naive_utc().date() - acquisition_date).num_days()`
  - 日付フォーマット: `date.format("%Y-%m-%d")`
  - SQLite互換: `NaiveDate` <-> `TEXT` (ISO 8601)

---

## 6. Domain-Driven Design Patterns

### 採用パターン

#### **Repository Pattern**
- **Trait定義**: domain層で `AssetRepository` trait
- **実装**: infrastructure層で `SqliteAssetRepository`
- **依存性逆転**: domain層はinfrastructureに依存しない

#### **Aggregate Pattern**
- **Asset Aggregate**: Asset (root) + AssetHistory + ChangeHistory
  - 不変条件: 数量 ≥ 0、カテゴリ ≥ 1
  - トランザクション境界: Asset + 関連履歴を一括保存
- **Category Aggregate**: Category (単独、階層構造は参照関係)
  - 不変条件: 循環参照の禁止

#### **Value Object Pattern**
- `Price`: 価格（整数、円単位、負数禁止）
- `Duration`: 所有期間（日数、計算値）
- `AssetId`, `CategoryId`: 型安全なID

#### **Domain Event Pattern**
- イベント例: `AssetCreated`, `AssetHistoryRecorded`, `CategoryAdded`
- ChangeHistory への記録にイベントパターンを活用

### Best Practices

- **Pure Domain Logic**: domain層は外部ライブラリ依存を最小化（chrono, thiserror のみ）
- **Ubiquitous Language**: コード内で日本語の業務用語を英訳（Asset=資産、Acquisition=取得）
- **Invariant Protection**: Aggregate rootでバリデーションを集中管理

---

## 7. TUI Best Practices

### 画面設計原則

1. **キーボードショートカット一貫性**
   - `q`: 終了 / 戻る
   - `Enter`: 選択 / 詳細表示
   - `e`: 編集
   - `d`: 削除
   - `n`: 新規作成
   - `h`: ヘルプ
   - `Tab` / `Shift+Tab`: フォーカス移動

2. **視覚的フィードバック**
   - 選択行のハイライト
   - ステータスバーに操作ヒント表示
   - エラーメッセージは赤色、成功メッセージは緑色

3. **レスポンシブレイアウト**
   - ターミナル幅80未満で警告表示
   - 幅に応じてカラム省略（長い名前は "..." で切り詰め）
   - リサイズイベントで即座に再描画

4. **パフォーマンス**
   - 仮想スクロール（表示範囲のみレンダリング）
   - 大量データはページネーション（100件/ページ）

---

## 8. Testing Strategy

### Unit Tests
- **Domain層**: エンティティのバリデーション、Aggregateの不変条件
- **Application層**: ユースケースロジック（モックRepository使用）
- **Infrastructure層**: SQLクエリの正確性（in-memory SQLite使用）

### Integration Tests
- **エンドツーエンドシナリオ**: Asset作成 → 履歴記録 → エクスポート
- **カテゴリ階層**: 親子関係、祖先/子孫クエリ

### Contract Tests
- **TUI Component**: 画面遷移、キーイベントハンドリング
- **Repository Interface**: trait契約の検証

### Test Tools
- `cargo test`: 標準テストフレームワーク
- `proptest` (optional): プロパティベーステスト（例: 数量変化の整合性）
- `criterion` (optional): ベンチマーク（パフォーマンスSC検証）

---

## 9. Logging Strategy

### Decision: **log + env_logger**

### Rationale
- ✅ **log**: 標準的なファサードcrate、`debug!`, `info!`, `warn!`, `error!` マクロ
- ✅ **env_logger**: 環境変数 `RUST_LOG` でログレベル制御
- ✅ **軽量**: オーバーヘッド最小

### Implementation Notes
- **依存関係**: `log = "0.4"`, `env_logger = "0.11"`
- **ログポリシー**（Constitution準拠）:
  - ✅ 記録する: 操作イベント（作成、編集、削除、エクスポート）
  - ❌ 記録しない: 資産名、価格、メモ等の具体的データ
- **例**:
  ```rust
  info!("Asset created: id={}", asset.id); // OK
  info!("Asset created: name={}", asset.name); // NG (プライバシー違反)
  ```

---

## Summary

すべてのNEEDS CLARIFICATIONが解決されました。Phase 1（設計フェーズ）に進む準備が整いました。

### 確定した技術スタック

| コンポーネント | 選定ライブラリ | バージョン |
|---------------|---------------|-----------|
| TUI Framework | ratatui | 0.25 |
| Terminal Backend | crossterm | 0.27 |
| SQLite Access | rusqlite | 0.30 |
| CLI Parsing | clap | 4.4 |
| Error Handling (Domain) | thiserror | 1.0 |
| Error Handling (App) | anyhow | 1.0 |
| Date/Time | chrono | 0.4 |
| Logging | log + env_logger | 0.4 / 0.11 |

### 次のステップ

- **Phase 1**: data-model.md, contracts/tui-api.md, quickstart.md を生成
