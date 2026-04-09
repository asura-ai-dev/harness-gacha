# UI Components - UI コンポーネント設計

## 概要

ratatui ベースの各画面の Widget 構成、レイアウト分割、キーバインド、状態管理を定義する。全画面で Cherry Cartridge パレットを適用する。

## 仕様からの対応

- F-1: カタログ画面 → catalog.rs
- F-2: Discovery 画面 → discovery.rs
- F-3: Pack 詳細画面 → pack_detail.rs
- F-4: 安全性詳細画面 → safety_detail.rs
- F-5: 購入案内画面 → purchase.rs
- F-6: 購入済みライブラリ画面 → library.rs
- F-7: インストール詳細画面 → install_detail.rs
- Cherry Cartridge パレット → theme.rs

## 設計

### テーマ定義 (`src/ui/theme.rs`)

```rust
use ratatui::style::Color;

pub struct Theme {
    pub primary_bg: Color,
    pub panel_bg: Color,
    pub border: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub accent: Color,
    pub accent_alt: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub owned: Color,
}

pub fn cherry_cartridge() -> Theme {
    Theme {
        primary_bg: Color::Rgb(0x15, 0x10, 0x18),
        panel_bg: Color::Rgb(0x24, 0x1A, 0x28),
        border: Color::Rgb(0x5B, 0x3A, 0x4A),
        text_primary: Color::Rgb(0xF7, 0xE8, 0xD8),
        text_secondary: Color::Rgb(0xD9, 0xB8, 0xA7),
        accent: Color::Rgb(0xFF, 0x7A, 0x59),
        accent_alt: Color::Rgb(0xFF, 0xD1, 0x66),
        success: Color::Rgb(0x7B, 0xD3, 0x89),
        warning: Color::Rgb(0xF4, 0xB9, 0x42),
        danger: Color::Rgb(0xE1, 0x56, 0x56),
        owned: Color::Rgb(0x6E, 0xC5, 0xE9),
    }
}
```

### 画面描画ディスパッチ (`src/ui/render.rs`)

```rust
pub fn render(frame: &mut Frame, app: &App) {
    match app.current_screen {
        Screen::Catalog => catalog::render(frame, app),
        Screen::Discovery => discovery::render(frame, app),
        Screen::PackDetail => pack_detail::render(frame, app),
        Screen::SafetyDetail => safety_detail::render(frame, app),
        Screen::Purchase => purchase::render(frame, app),
        Screen::Library => library::render(frame, app),
        Screen::InstallDetail => install_detail::render(frame, app),
        Screen::Help => help::render(frame, app),
    }
}
```

### 画面 1: ホーム / カタログ (`src/ui/catalog.rs`)

レイアウト:

```
+---------------------------------------------------------------+
| タイトルバー: "harness-gacha"       検索: [query          ]   |
+---------------------------------------------------------------+
| タブ: [Featured] [Recent] [Recommended]  タグ: [tag1] [tag2] |
+---------------------------------------------------------------+
| Pack 一覧（リスト）                                            |
|   > Pack名    対応ツール    価格    Trust簡易表示              |
|     Pack名    対応ツール    価格    Trust簡易表示              |
|     ...                                                        |
+---------------------------------------------------------------+
| フッター: [Enter] 詳細  [/] 検索  [T] タグ  [R] Discovery    |
|           [L] 購入済み  [?] ヘルプ  [Q] 終了                  |
+---------------------------------------------------------------+
```

レイアウト分割（垂直）:

1. タイトルバー: 3 行固定
2. タブ / タグ行: 3 行固定
3. Pack 一覧: 残り全部（Min(0), ratatui::layout::Constraint::Fill(1)）
4. フッター: 3 行固定

Pack 一覧は `ratatui::widgets::List` を使用。選択行は `accent` 色で反転表示。購入済み pack は `owned` 色のラベル `[OWNED]` を末尾に付与。

キーバインド:

- `j` / `Down`: 下に移動
- `k` / `Up`: 上に移動
- `Tab` / `l` / `Right`: 次のタブ
- `BackTab` / `h` / `Left`: 前のタブ
- `Enter`: 選択中の pack の詳細画面へ
- `/`: 検索モード ON（入力フォーカス）
- `Esc`: 検索モード OFF
- `t`: タグ絞り込みトグル
- `r`: Discovery 画面へ
- `L`: 購入済みライブラリへ
- `?`: ヘルプ画面へ
- `q`: 終了

### 画面 2: Discovery (`src/ui/discovery.rs`)

3 つのサブ状態に応じて描画を切り替える。

#### 待機状態

レイアウト:

```
+---------------------------------------------------------------+
| Capsule Discovery                                              |
+---------------------------------------------------------------+
|                                                                |
|              ┌──────────────────┐                              |
|              │   CAPSULE GAME   │                              |
|              │      ○  ○  ○     │                              |
|              │    ○  ○  ○  ○    │                              |
|              │        ▒▒        │                              |
|              │       ─┼─        │                              |
|              └──────────────────┘                              |
|                                                                |
|          PRESS ENTER TO DISCOVER                               |
|       無料でおすすめ pack を 1 つ表示します                    |
|                                                                |
+---------------------------------------------------------------+
| [Enter] ランダムで探す  [B] 戻る                               |
+---------------------------------------------------------------+
```

中央配置は `Paragraph` + `alignment: Center`。マシン ASCII アートは `capsule_machine.rs` Widget で描画。CTA テキスト "PRESS ENTER TO DISCOVER" は `accent` 色。

キーバインド:

- `Enter`: 演出開始（DiscoveryState -> Animating）
- `b` / `Esc`: 前の画面に戻る

#### 演出状態

tick イベント（250ms 間隔）ごとに frame を進める。全 6 フレーム（約 1.5 秒）。

フレーム構成:

- Frame 0-1: レバー操作表現（`─┼─` → `\│/`）
- Frame 2-3: カプセル落下表現（`▒▒` が下に移動、`○` が消える）
- Frame 4-5: カプセル開封表現（色変化、`accent_alt` フラッシュ）

演出中はキー入力を無視（Enter で skip 可能にしてもよい）。

#### 結果状態

レイアウト:

```
+---------------------------------------------------------------+
| Discovery Result                                               |
+---------------------------------------------------------------+
|                                                                |
|              [ Capsule Opened ]                                |
|                                                                |
|           {pack_name}                                          |
|   {summary}                                                    |
|                                                                |
|   Creator: {author.name}                                       |
|   Targets: {targets joined}                                    |
|   Price: YEN {price}                                           |
|   Trust: {permission summary}                                  |
|   {contents_summary formatted}                                 |
|                                                                |
+---------------------------------------------------------------+
| [Enter] 詳細を見る  [R] もう一度探す  [B] 戻る                |
+---------------------------------------------------------------+
```

pack 情報は `pack_card.rs` Widget で描画。

キーバインド:

- `Enter`: Pack 詳細画面へ
- `r`: もう一度 Discovery（Idle に戻って即 Animating）
- `b` / `Esc`: カタログに戻る

### 画面 3: Pack 詳細 (`src/ui/pack_detail.rs`)

レイアウト:

```
+---------------------------------------------------------------+
| {pack_name}                                    YEN {price}    |
| Creator: {author}               Updated: {updated_at}        |
+---------------------------------------------------------------+
| Summary                                                        |
| {summary}                                                      |
|                                                                |
| Targets                                                        |
| - {tool} {version_range}                                       |
|                                                                |
| Included Summary                                               |
| - skills: {n}  hooks: {n}  templates: {n}                     |
|                                                                |
| Permission Summary                                             |
| - shell: {yes/no}  network: {yes/no}  ...                     |
|                                                                |
| Sample Preview                                                 |
| "{sample_preview}"                                             |
+---------------------------------------------------------------+
| [S] 安全性詳細  [P] 購入案内  [B] 戻る                        |
+---------------------------------------------------------------+
```

スクロール対応: コンテンツが画面に収まらない場合は `j/k` でスクロール。`scroll_offset: u16` を App に持たせるか、pack_detail 専用の状態で管理する。

Permission Summary では、`shell: yes` と `network: yes` は `danger` 色、`filesystem_write: yes` は `warning` 色、それ以外は `text_secondary` 色で表示する。`permission_badge.rs` Widget を使用。

キーバインド:

- `s`: 安全性詳細画面へ
- `p`: 購入案内画面へ
- `b` / `Esc`: 前の画面に戻る
- `j` / `Down`: スクロールダウン
- `k` / `Up`: スクロールアップ

### 画面 4: 安全性詳細 (`src/ui/safety_detail.rs`)

レイアウト:

```
+---------------------------------------------------------------+
| Safety Details: {pack_name}                                    |
+---------------------------------------------------------------+
| Permissions                                                    |
|   shell:            {enabled/disabled}  [DANGER色 or SUCCESS色]|
|   network:          {enabled/disabled}                         |
|   filesystem_read:  {enabled/disabled}                         |
|   filesystem_write: {enabled/disabled}                         |
|   git:              {enabled/disabled}                         |
|                                                                |
| Risks                                                          |
| - {risk_1}                                                     |
| - {risk_2}                                                     |
|                                                                |
| Review Notes                                                   |
| {review_notes}                                                 |
+---------------------------------------------------------------+
| [P] 購入案内  [B] 詳細へ戻る                                  |
+---------------------------------------------------------------+
```

各 permission 行は `permission_badge.rs` を使い、enabled かつ danger（shell, network）なら `danger` 色背景 + 白文字、enabled かつ warning（filesystem_write, git）なら `warning` 色、disabled なら `success` 色で "disabled" 表示。

キーバインド:

- `p`: 購入案内画面へ
- `b` / `Esc`: Pack 詳細に戻る

### 画面 5: 購入案内 (`src/ui/purchase.rs`)

レイアウト:

```
+---------------------------------------------------------------+
| Checkout: {pack_name}                                          |
+---------------------------------------------------------------+
| Price:  YEN {price}                                            |
| Seller: harness-gacha                                          |
|                                                                |
| Policy                                                         |
| - デジタル商品のため購入後の返金は条件付き                     |
| - 利用条件は Web の規約を参照                                  |
|                                                                |
| Checkout URL                                                   |
| {checkout_url}                                                 |
|                                                                |
| QR Code                                                        |
| {QR コードのテキスト表現}                                      |
|                                                                |
+---------------------------------------------------------------+
| [Enter] Checkout を開く  [C] URL コピー  [B] 戻る             |
+---------------------------------------------------------------+
```

QR コードは `qr_code.rs` Widget で描画。`qrcode` クレートで生成した QR をブロック文字（`█` と ` `）で表現する。

キーバインド:

- `Enter`: ブラウザで checkout_url を開く（`open::that()`）
- `c`: checkout_url をクリップボードにコピー（`arboard`）
- `b` / `Esc`: 前の画面に戻る

### 画面 6: 購入済みライブラリ (`src/ui/library.rs`)

レイアウト:

```
+---------------------------------------------------------------+
| My Library                                                     |
+---------------------------------------------------------------+
| > {pack_name}    v{version}    {Installed/Not Installed}      |
|                                {Update: none/available}        |
|   {pack_name}    v{version}    ...                             |
|   ...                                                          |
+---------------------------------------------------------------+
| [Enter] Install 詳細  [B] 戻る                                |
+---------------------------------------------------------------+
```

`ratatui::widgets::List` を使用。各行に pack 名、バージョン、install 状態、update 状態を表示。install 済みは `owned` 色、未 install は `text_secondary` 色。update available は `warning` 色ラベル。

表示する pack は `entitlements.json` の `status == "active"` のもののみ。各 pack の詳細情報（name, version 等）は `catalog.json` から pack_id で引く。

キーバインド:

- `j` / `Down`: 下に移動
- `k` / `Up`: 上に移動
- `Enter`: インストール詳細画面へ
- `b` / `Esc`: カタログに戻る

### 画面 7: インストール詳細 (`src/ui/install_detail.rs`)

レイアウト:

```
+---------------------------------------------------------------+
| Install: {pack_name}                                           |
+---------------------------------------------------------------+
| Method: {method}                                               |
|                                                                |
| Command                                                        |
| {steps[0]}                                                     |
| {steps[1]}                                                     |
|                                                                |
| Post Install                                                   |
| - {post_install[0]}                                            |
| - {post_install[1]}                                            |
|                                                                |
| Rollback                                                       |
| - 追加ファイルを削除                                           |
| - 変更前バックアップを復元                                     |
+---------------------------------------------------------------+
| [B] 戻る                                                       |
+---------------------------------------------------------------+
```

キーバインド:

- `b` / `Esc`: ライブラリに戻る
- `j` / `Down`: スクロールダウン
- `k` / `Up`: スクロールアップ

### 画面 8: ヘルプ / 法務情報 (`src/ui/help.rs`)

レイアウト:

```
+---------------------------------------------------------------+
| Help                                                           |
+---------------------------------------------------------------+
| Key Bindings                                                   |
| - j/k: 移動  Enter: 選択  b/Esc: 戻る                        |
| - /: 検索  t: タグ  r: Discovery  L: ライブラリ              |
| - q: 終了                                                      |
|                                                                |
| Legal                                                          |
| - 特定商取引法に基づく表示: {URL}                              |
| - 利用規約: {URL}                                              |
| - 返金ポリシー: {URL}                                          |
| - お問い合わせ: {email}                                        |
+---------------------------------------------------------------+
| [B] 戻る                                                       |
+---------------------------------------------------------------+
```

法務情報の URL/テキストはハードコードまたは設定ファイルから読み込み。MVP ではハードコードで可。

キーバインド:

- `b` / `Esc`: 前の画面に戻る

### カスタム Widget

#### CapsuleMachine (`src/ui/widgets/capsule_machine.rs`)

ASCII アートのガチャガチャマシンを描画する StatefulWidget。状態として `frame: u8` を受け取り、フレームに応じた ASCII アートを返す。

描画サイズ: 幅 22 文字 x 高さ 9 行（枠含む）。中央配置は呼び出し側で Rect を計算する。

#### PackCard (`src/ui/widgets/pack_card.rs`)

Discovery 結果や一覧で使う pack 情報カード。`CatalogEntry` の参照を受け取り、name, summary, creator, targets, price, trust summary, contents_summary を描画する。

#### PermissionBadge (`src/ui/widgets/permission_badge.rs`)

単一の permission 名と enabled/disabled を受け取り、色付きバッジとして描画する関数群。Widget ではなく `Span` を返すヘルパー関数として実装してもよい。

```rust
pub fn permission_span(name: &str, enabled: bool, theme: &Theme) -> Span {
    let (label, color) = if !enabled {
        ("disabled", theme.success)
    } else {
        match name {
            "shell" | "network" => ("enabled", theme.danger),
            "filesystem_write" | "git" => ("enabled", theme.warning),
            _ => ("enabled", theme.text_secondary),
        }
    };
    Span::styled(format!("{name}: {label}"), Style::default().fg(color))
}
```

#### QrCode (`src/ui/widgets/qr_code.rs`)

URL 文字列を受け取り、`qrcode` クレートで生成した QR を `█` と ` ` のブロック文字で `Paragraph` に変換する。表示サイズは URL 長に依存するが、概ね 30x30 文字以内。

## 制約・注意事項

- ratatui の immediate mode では毎フレーム全描画。Widget は状態を持たず、App の状態を参照して描画する
- 画面サイズ（ターミナルサイズ）が小さい場合のフォールバックは Phase 5 で対応。MVP では最低 80x24 を想定
- Discovery アニメーションの tick 間隔は `event.rs` の poll timeout で制御する（通常 250ms）
- フッターのキーバインド表示は全画面で統一フォーマット: `[Key] 説明` を `accent` 色で表示
- 検索モード中は通常のキーバインドを無効化し、テキスト入力を受け付ける。`Esc` で検索モード終了
