# TUI API Contract: Asset Management

**Date**: 2026-01-24  
**Phase**: 1 - Design Artifacts

## Overview

Asset Management TUIのコンポーネントインターフェース契約を定義します。各画面とコンポーネントの責務、入力/出力、キーバインディングを明確化します。

---

## Screen Contracts

### 1. AssetListScreen

**Responsibility**: 資産一覧表示、フィルタリング、並び替え

**State**:
```rust
pub struct AssetListState {
    assets: Vec<AssetDto>,          // 表示中の資産リスト
    selected_index: usize,          // 選択中の行インデックス
    filter: FilterState,            // フィルタ条件
    sort_by: SortOption,            // 並び替え基準
    scroll_offset: usize,           // スクロール位置
}

pub struct FilterState {
    categories: Vec<CategoryId>,    // 選択中のカテゴリ
    manufacturer: Option<String>,   // 選択中のメーカー
    include_disposed: bool,         // 処分済み資産を含むか
}

pub enum SortOption {
    NameAsc,
    NameDesc,
    AcquisitionDateAsc,
    AcquisitionDateDesc,
    OwnershipDurationAsc,
    OwnershipDurationDesc,
}
```

**Rendered Elements**:
- ヘッダー: タイトル "資産一覧"、件数表示
- テーブル: カラム（名前、カテゴリ、メーカー、購入日、購入価格、数量、所有期間）
- フッター: キーバインディングヒント、ステータスメッセージ

**Key Bindings**:
| Key | Action | Transition |
|-----|--------|------------|
| `↑` / `k` | 前の行を選択 | - |
| `↓` / `j` | 次の行を選択 | - |
| `Enter` | 詳細画面へ遷移 | → AssetDetailScreen |
| `n` | 新規作成フォームへ遷移 | → AssetFormScreen(New) |
| `e` | 編集フォームへ遷移 | → AssetFormScreen(Edit) |
| `d` | 削除確認ダイアログ表示 | → DeleteConfirmDialog |
| `f` | フィルタ設定ダイアログ表示 | → FilterDialog |
| `s` | 並び替えオプション表示 | → SortDialog |
| `q` | アプリ終了 | Exit |
| `h` / `?` | ヘルプ画面表示 | → HelpScreen |

**Contract Tests**:
```rust
#[test]
fn test_asset_list_screen_renders_assets() {
    let state = AssetListState::with_assets(vec![...]);
    let screen = AssetListScreen::new(state);
    let buffer = render_to_buffer(&screen);
    assert!(buffer.contains("AirPods Pro"));
}

#[test]
fn test_asset_list_screen_handles_enter_key() {
    let mut state = AssetListState::with_assets(vec![...]);
    let event = KeyEvent::from(KeyCode::Enter);
    let result = handle_key(&mut state, event);
    assert_eq!(result, Transition::ToDetail(asset_id));
}
```

---

### 2. AssetDetailScreen

**Responsibility**: 資産の詳細情報表示、履歴表示

**State**:
```rust
pub struct AssetDetailState {
    asset: AssetDto,
    categories: Vec<CategoryDto>,
    histories: Vec<AssetHistoryDto>,   // 最新N件
    changes: Vec<ChangeHistoryDto>,    // 最新N件
    active_tab: DetailTab,
}

pub enum DetailTab {
    Overview,       // 基本情報
    Histories,      // AssetHistory
    Changes,        // ChangeHistory
}
```

**Rendered Elements**:
- ヘッダー: タイトル "資産詳細"、資産名
- タブ: Overview / Histories / Changes
- Overview: 名前、カテゴリ（パンくずリスト）、メーカー、数量、購入日、購入価格、所有期間、メモ
- Histories: AssetHistoryテーブル（日付、種類、数量変化、価格、場所、URL、メモ）
- Changes: ChangeHistoryテーブル（日時、操作種類、変更内容）
- フッター: キーバインディングヒント

**Key Bindings**:
| Key | Action | Transition |
|-----|--------|------------|
| `Tab` | 次のタブへ移動 | - |
| `Shift+Tab` | 前のタブへ移動 | - |
| `e` | 編集フォームへ遷移 | → AssetFormScreen(Edit) |
| `d` | 削除確認ダイアログ表示 | → DeleteConfirmDialog |
| `a` | 履歴イベント追加フォーム表示 | → HistoryFormScreen |
| `q` / `Esc` | 一覧画面へ戻る | → AssetListScreen |
| `h` / `?` | ヘルプ画面表示 | → HelpScreen |

---

### 3. AssetFormScreen

**Responsibility**: 資産の新規作成・編集フォーム

**State**:
```rust
pub struct AssetFormState {
    mode: FormMode,
    name: String,
    manufacturer: Option<String>,
    quantity: String,              // 入力中はString、バリデーション後にi32へ変換
    memo: Option<String>,
    selected_categories: Vec<CategoryId>,
    focused_field: FormField,
    validation_errors: HashMap<FormField, String>,
}

pub enum FormMode {
    New,
    Edit(AssetId),
}

pub enum FormField {
    Name,
    Manufacturer,
    Quantity,
    Memo,
    Categories,
}
```

**Rendered Elements**:
- ヘッダー: タイトル "新規資産" or "資産編集"
- フォーム:
  - 名前: テキスト入力（必須）
  - メーカー: テキスト入力（任意）
  - 数量: 数値入力（任意、デフォルト1）
  - カテゴリ: 複数選択（最低1つ必須）
  - メモ: 複数行テキスト入力（任意）
- バリデーションエラー表示
- フッター: キーバインディングヒント

**Key Bindings**:
| Key | Action | Transition |
|-----|--------|------------|
| `Tab` | 次のフィールドへ移動 | - |
| `Shift+Tab` | 前のフィールドへ移動 | - |
| `Enter` | 保存して一覧へ戻る | → AssetListScreen (on success) |
| `Ctrl+S` | 保存 | - |
| `Esc` / `Ctrl+C` | キャンセルして戻る | → AssetListScreen (discard changes) |

**Validation Rules**:
- 名前: 必須、1-255文字
- 数量: 0以上の整数
- カテゴリ: 最低1つ選択必須

---

### 4. HistoryFormScreen

**Responsibility**: AssetHistory イベント追加フォーム

**State**:
```rust
pub struct HistoryFormState {
    asset_id: AssetId,
    history_type: HistoryType,
    history_date: String,          // YYYY-MM-DD形式、入力中はString
    quantity_change: String,       // 入力中はString、バリデーション後にi32へ変換
    price: Option<String>,
    reason: Option<String>,
    location: Option<String>,
    url: Option<String>,
    memo: Option<String>,
    focused_field: HistoryFormField,
    validation_errors: HashMap<HistoryFormField, String>,
}

pub enum HistoryFormField {
    HistoryType,
    HistoryDate,
    QuantityChange,
    Price,
    Reason,
    Location,
    Url,
    Memo,
}
```

**Rendered Elements**:
- ヘッダー: タイトル "履歴イベント追加"
- フォーム:
  - 種類: ラジオボタン（取得/処分/破損/紛失/数量調整）
  - 日付: 日付入力（YYYY-MM-DD）
  - 数量変化: 数値入力（+N, -N, 0）
  - 価格: 数値入力（任意、取得/処分時のみ表示）
  - 理由: テキスト入力（任意）
  - 場所: テキスト入力（任意、取得/処分時のみ表示）
  - URL: テキスト入力（任意、取得/処分時のみ表示）
  - メモ: 複数行テキスト入力（任意）
- バリデーションエラー表示
- フッター: キーバインディングヒント

**Key Bindings**: （AssetFormScreenと同様）

**Validation Rules**:
- 種類: 必須
- 日付: 必須、YYYY-MM-DD形式、未来の日付禁止
- 数量変化: 必須、整数
- 価格: 0以上の整数（指定時）

---

### 5. CategoryTreeScreen

**Responsibility**: カテゴリ階層管理

**State**:
```rust
pub struct CategoryTreeState {
    root_categories: Vec<CategoryDto>,
    expanded_nodes: HashSet<CategoryId>,   // 展開中のノード
    selected_node: Option<CategoryId>,     // 選択中のノード
}
```

**Rendered Elements**:
- ヘッダー: タイトル "カテゴリ管理"
- ツリー: インデント表示、展開/折りたたみアイコン
- フッター: キーバインディングヒント

**Key Bindings**:
| Key | Action | Transition |
|-----|--------|------------|
| `↑` / `k` | 前のノードを選択 | - |
| `↓` / `j` | 次のノードを選択 | - |
| `→` / `l` | ノードを展開 | - |
| `←` / `h` | ノードを折りたたみ | - |
| `n` | 新規カテゴリ作成フォーム表示 | → CategoryFormScreen(New) |
| `e` | カテゴリ編集フォーム表示 | → CategoryFormScreen(Edit) |
| `d` | カテゴリ削除確認ダイアログ表示 | → DeleteConfirmDialog |
| `q` / `Esc` | 一覧画面へ戻る | → AssetListScreen |

---

### 6. HelpScreen

**Responsibility**: キーバインディングヘルプ表示

**Rendered Elements**:
- ヘッダー: タイトル "ヘルプ"
- テーブル: 画面ごとのキーバインディング一覧
- フッター: "qで戻る"

**Key Bindings**:
| Key | Action | Transition |
|-----|--------|------------|
| `q` / `Esc` | 前の画面へ戻る | → (previous screen) |

---

## Component Contracts

### TableComponent

**Responsibility**: データテーブル表示、選択、スクロール

**Props**:
```rust
pub struct TableProps<T> {
    items: Vec<T>,
    columns: Vec<ColumnDef>,
    selected_index: usize,
    scroll_offset: usize,
    visible_rows: usize,
}

pub struct ColumnDef {
    title: String,
    width: ColumnWidth,
    align: Alignment,
    render: fn(&T) -> String,
}

pub enum ColumnWidth {
    Fixed(u16),
    Percentage(u16),
    Flex(u16),
}

pub enum Alignment {
    Left,
    Center,
    Right,
}
```

**Behavior**:
- ヘッダー行のレンダリング
- データ行のレンダリング（選択行はハイライト）
- スクロール処理（表示範囲外の行は非表示）

**Contract Tests**:
```rust
#[test]
fn test_table_component_renders_headers() {
    let props = TableProps { ... };
    let buffer = render_to_buffer(&props);
    assert!(buffer.contains("名前"));
    assert!(buffer.contains("カテゴリ"));
}

#[test]
fn test_table_component_highlights_selected_row() {
    let props = TableProps { selected_index: 1, ... };
    let buffer = render_to_buffer(&props);
    assert!(buffer.row(1).is_highlighted());
}
```

---

### FormComponent

**Responsibility**: フォーム入力、バリデーション表示

**Props**:
```rust
pub struct FormProps {
    fields: Vec<FormFieldDef>,
    focused_index: usize,
    validation_errors: HashMap<String, String>,
}

pub struct FormFieldDef {
    id: String,
    label: String,
    field_type: FieldType,
    value: String,
    required: bool,
}

pub enum FieldType {
    Text,
    Number,
    Date,
    MultilineText,
    Select(Vec<String>),
    MultiSelect(Vec<String>),
}
```

**Behavior**:
- フィールドのレンダリング（フォーカス中は枠線表示）
- バリデーションエラーの表示（フィールド下に赤色メッセージ）
- 入力値の変更ハンドリング

---

### TreeComponent

**Responsibility**: 階層構造表示、展開/折りたたみ

**Props**:
```rust
pub struct TreeProps<T> {
    root_nodes: Vec<TreeNode<T>>,
    expanded_ids: HashSet<String>,
    selected_id: Option<String>,
}

pub struct TreeNode<T> {
    id: String,
    data: T,
    children: Vec<TreeNode<T>>,
}
```

**Behavior**:
- ツリー構造の再帰的レンダリング
- インデント表示（深さに応じて増加）
- 展開/折りたたみアイコン表示（子ノードがある場合）

---

### DialogComponent

**Responsibility**: 確認ダイアログ、モーダル表示

**Props**:
```rust
pub struct DialogProps {
    title: String,
    message: String,
    buttons: Vec<ButtonDef>,
    focused_button: usize,
}

pub struct ButtonDef {
    label: String,
    action: DialogAction,
}

pub enum DialogAction {
    Confirm,
    Cancel,
    Custom(String),
}
```

**Behavior**:
- 中央にモーダルウィンドウ表示
- ボタンの選択（フォーカス移動）
- Enterでボタンアクション実行

---

## Application State Machine

```rust
pub enum Screen {
    AssetList(AssetListState),
    AssetDetail(AssetDetailState),
    AssetForm(AssetFormState),
    HistoryForm(HistoryFormState),
    CategoryTree(CategoryTreeState),
    Help(HelpState),
}

pub struct App {
    current_screen: Screen,
    screen_stack: Vec<Screen>,  // 画面履歴（戻る機能用）
}

impl App {
    pub fn transition_to(&mut self, screen: Screen);
    pub fn go_back(&mut self);
    pub fn handle_key(&mut self, event: KeyEvent) -> Result<(), AppError>;
    pub fn render(&self, frame: &mut Frame);
}
```

---

## Event Handling Contract

```rust
pub trait EventHandler {
    fn handle_key(&mut self, event: KeyEvent) -> EventResult;
}

pub enum EventResult {
    Handled,
    Unhandled,
    Transition(Screen),
    Exit,
}
```

---

## DTO (Data Transfer Objects)

```rust
pub struct AssetDto {
    pub id: AssetId,
    pub name: String,
    pub manufacturer: Option<String>,
    pub quantity: i32,
    pub memo: Option<String>,
    pub categories: Vec<CategoryDto>,
    pub first_acquisition_date: Option<NaiveDate>,
    pub purchase_price: Option<Price>,
    pub ownership_duration: Option<Duration>,
    pub is_disposed: bool,
}

pub struct CategoryDto {
    pub id: CategoryId,
    pub name: String,
    pub parent_id: Option<CategoryId>,
    pub ancestors: Vec<CategoryDto>,  // パンくずリスト用
}

pub struct AssetHistoryDto {
    pub id: i64,
    pub history_type: HistoryType,
    pub history_date: NaiveDate,
    pub quantity_change: i32,
    pub price: Option<Price>,
    pub reason: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub memo: Option<String>,
}

pub struct ChangeHistoryDto {
    pub id: i64,
    pub operation_type: OperationType,
    pub operation_at: NaiveDateTime,
    pub changes_summary: String,  // JSONから生成した人間可読な変更サマリー
}
```

---

## Summary

Asset Management TUIの全コンポーネント契約が定義されました。

- **Screens**: 6個（List, Detail, Form, HistoryForm, CategoryTree, Help）
- **Components**: 4個（Table, Form, Tree, Dialog）
- **Key Bindings**: 各画面で一貫したキーバインディング
- **State Management**: Screen enumベースのステートマシン
- **DTOs**: プレゼンテーション層とアプリケーション層の境界

次のステップ: quickstart.md を生成
