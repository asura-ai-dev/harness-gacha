# Architecture - 全体アーキテクチャ

## 概要

harness-gacha は Rust + ratatui による TUI アプリケーションである。ローカル JSON ファイルをデータストアとし、画面遷移ベースの状態機械でユーザー操作を処理する。外部依存は Stripe Payment Links（ブラウザ遷移）のみ。

## 仕様からの対応

- spec 全体: 技術選定（Rust + ratatui + crossterm）
- F-1 〜 F-8: 各画面と機能の基盤
- 非機能要件: パフォーマンス、保守性

## モジュール構成

```
src/
  main.rs              # エントリポイント、App 初期化、イベントループ起動
  app.rs               # App 構造体（状態管理の中心）
  event.rs             # crossterm イベント読み取り、tick タイマー
  ui/
    mod.rs             # ui モジュール公開
    render.rs          # 画面描画のディスパッチ（現在の Screen に応じて委譲）
    catalog.rs         # ホーム / カタログ画面の描画
    discovery.rs       # Discovery 画面（待機・演出・結果）の描画
    pack_detail.rs     # Pack 詳細画面の描画
    safety_detail.rs   # 安全性詳細画面の描画
    purchase.rs        # 購入案内画面の描画
    library.rs         # 購入済みライブラリ画面の描画
    install_detail.rs  # インストール詳細画面の描画
    help.rs            # ヘルプ / 法務情報画面の描画
    theme.rs           # Cherry Cartridge カラーパレット定義
    widgets/
      mod.rs           # カスタム Widget 公開
      capsule_machine.rs  # ガチャガチャマシンの ASCII アート Widget
      pack_card.rs     # Pack カード表示 Widget
      permission_badge.rs # 権限バッジ Widget（danger 色対応）
      qr_code.rs       # テキスト QR コード Widget
  data/
    mod.rs             # data モジュール公開
    catalog.rs         # CatalogEntry の読み込み・フィルタリング
    entitlement.rs     # Entitlement の読み書き
    accounting.rs      # 売上記録の読み書き、月次分配計算
    manifest.rs        # manifest.json パーサー・バリデーター
  models/
    mod.rs             # models モジュール公開
    catalog_entry.rs   # CatalogEntry 構造体
    entitlement.rs     # Entitlement 構造体
    accounting.rs      # Transaction, CreatorShare 構造体
    manifest.rs        # Manifest 構造体（manifest.json の型）
    permissions.rs     # Permissions 構造体
  screen.rs            # Screen enum（画面遷移の状態定義）
  action.rs            # Action enum（ユーザー操作の抽象化）
  discovery.rs         # Discovery ロジック（ランダム選択、演出状態管理）
  clipboard.rs         # URL コピー機能
  browser.rs           # ブラウザ起動（open crate）
data/
  catalog.json         # カタログデータ（サンプル）
  entitlements.json    # Entitlement データ
  accounting.json      # 売上記録データ
```

## ディレクトリ構成（プロジェクトルート）

```
harness-gacha/
  Cargo.toml
  Cargo.lock
  src/                 # Rust ソースコード
  data/                # ローカル JSON データファイル
  docs/                # 製品ドキュメント（既存）
  agent-docs/          # 設計ドキュメント
  tasks/               # タスクチケット
  agent-output/        # ハンドオフ記録
```

## 依存クレート

| クレート   | 用途                             | バージョン目安 |
| ---------- | -------------------------------- | -------------- |
| ratatui    | TUI フレームワーク               | 0.29+          |
| crossterm  | ターミナルバックエンド           | 0.28+          |
| serde      | JSON シリアライズ/デシリアライズ | 1.x            |
| serde_json | JSON パーサー                    | 1.x            |
| chrono     | 日時処理（ISO 8601）             | 0.4.x          |
| rand       | ランダム選択（Discovery）        | 0.8+           |
| open       | ブラウザ起動                     | 5.x            |
| qrcode     | QR コード生成                    | 0.14+          |
| arboard    | クリップボード操作               | 3.x            |
| thiserror  | エラー型定義                     | 2.x            |

## アプリケーションアーキテクチャ

### イベントループ

```
loop {
  1. crossterm からイベントを poll（250ms timeout で tick も発生）
  2. イベントを Action に変換
  3. App::update(action) で状態を更新
  4. Terminal::draw(|f| ui::render(f, &app)) で描画
  5. App::should_quit なら break
}
```

### 状態管理

`App` 構造体が全状態を保持する:

- `current_screen: Screen` - 現在の画面
- `catalog: Vec<CatalogEntry>` - カタログデータ
- `entitlements: EntitlementStore` - Entitlement データ
- `accounting: AccountingData` - 売上記録
- `selected_pack: Option<String>` - 選択中の pack id
- `discovery_state: DiscoveryState` - Discovery の状態（Idle/Animating/Result）
- `catalog_state: CatalogState` - カタログの表示状態（タブ、フィルタ、選択位置）
- `library_state: LibraryState` - ライブラリの選択位置
- `search_query: String` - 検索クエリ
- `tick_count: u64` - アニメーション用 tick カウンター

### 画面遷移（Screen enum）

```rust
enum Screen {
    Catalog,
    Discovery,
    PackDetail,
    SafetyDetail,
    Purchase,
    Library,
    InstallDetail,
    Help,
}
```

遷移は `App::update()` 内で `Action` に応じて `current_screen` を変更することで実現する。戻る操作のために `screen_stack: Vec<Screen>` を保持する。

### Action enum

```rust
enum Action {
    Quit,
    Back,
    Enter,
    Up,
    Down,
    Left,
    Right,
    Tab,
    Search,
    ToggleDiscovery,
    OpenLibrary,
    OpenHelp,
    OpenCheckout,
    CopyUrl,
    Tick,
}
```

## 制約・注意事項

- ratatui は即時モード（immediate mode）の描画。毎フレーム全画面を再描画する
- crossterm の raw mode は main で enter/leave する。panic 時のクリーンアップに注意
- JSON データファイルは起動時に全量読み込み。MVP のデータ量では問題ない
- Discovery アニメーションは tick イベント（250ms 間隔）を利用してフレームを進める
- ブラウザ起動とクリップボード操作は OS 依存。macOS / Linux を対象
