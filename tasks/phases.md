# フェーズ定義

## 概要

harness-gacha MVP の実装を 5 フェーズに分割する。各フェーズは前フェーズの成果物に依存し、順次実行する。

## Phase 1: 基盤とカタログ表示

### 概要

Rust プロジェクトの初期化、データモデル定義、JSON 読み込み、カラーテーマ、TUI イベントループの骨格、カタログ画面の描画を実装する。全フェーズの土台。

### 成果物

- Cargo.toml と依存クレート設定
- src/models/ 配下の全 struct 定義
- src/data/catalog.rs: catalog.json 読み込み
- src/ui/theme.rs: Cherry Cartridge パレット
- src/app.rs: App 構造体と基本状態管理
- src/screen.rs: Screen enum
- src/action.rs: Action enum
- src/event.rs: crossterm イベントループ
- src/ui/catalog.rs: カタログ画面描画
- src/ui/render.rs: 画面ディスパッチ
- src/main.rs: エントリポイント
- data/catalog.json: サンプルデータ（3 pack 以上）
- src/data/manifest.rs: manifest バリデーター

### 依存

なし

### チケット

task-1001 〜 task-1007

---

## Phase 2: Pack 詳細 + 安全性詳細

### 概要

Pack 詳細画面と安全性詳細画面を実装し、カタログからの画面遷移を接続する。

### 成果物

- src/ui/pack_detail.rs: Pack 詳細画面
- src/ui/safety_detail.rs: 安全性詳細画面
- src/ui/widgets/permission_badge.rs: 権限バッジ
- カタログ -> 詳細 -> 安全性の画面遷移

### 依存

Phase 1

### チケット

task-2001 〜 task-2003

---

## Phase 3: Discovery（ガチャガチャ演出）

### 概要

Discovery 画面の 3 状態（待機・演出・結果）を実装する。ASCII アートによるマシン描画とフレームアニメーションを含む。

### 成果物

- src/ui/discovery.rs: Discovery 画面（3 状態）
- src/ui/widgets/capsule_machine.rs: マシン ASCII アート Widget
- src/ui/widgets/pack_card.rs: Pack カード Widget
- src/discovery.rs: ランダム選択ロジック
- 結果から Pack 詳細への遷移

### 依存

Phase 1（カタログデータ）、Phase 2（Pack 詳細画面）

### チケット

task-3001 〜 task-3003

---

## Phase 4: 購入導線と Entitlement

### 概要

購入案内画面、購入済みライブラリ画面、インストール詳細画面を実装する。ブラウザ起動、URL コピー、QR コード表示、entitlement データの読み書きを含む。

### 成果物

- src/ui/purchase.rs: 購入案内画面
- src/ui/library.rs: 購入済みライブラリ画面
- src/ui/install_detail.rs: インストール詳細画面
- src/ui/widgets/qr_code.rs: QR コード Widget
- src/data/entitlement.rs: entitlement 読み書き
- src/browser.rs: ブラウザ起動
- src/clipboard.rs: URL コピー
- data/entitlements.json: サンプルデータ

### 依存

Phase 2（Pack 詳細画面からの遷移）

### チケット

task-4001 〜 task-4004

---

## Phase 5: 運用基盤と仕上げ

### 概要

売上記録、月次分配計算、法務表示、検索・タグ絞り込み、エラーハンドリング、サンプルデータの充実を実装する。

### 成果物

- src/data/accounting.rs: 売上記録読み書き、月次分配計算
- src/ui/help.rs: ヘルプ / 法務情報画面
- カタログ検索・タグ絞り込み機能
- エラーハンドリング整備
- data/accounting.json: サンプルデータ
- data/catalog.json: サンプル pack 追加（5 pack 以上）

### 依存

Phase 1-4

### チケット

task-5001 〜 task-5005
