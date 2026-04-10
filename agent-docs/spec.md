# harness-gacha MVP 実装仕様

## 目的

Claude Code / Codex 向けの harness pack をキュレーション型で販売する TUI ストアの MVP を構築する。ユーザーが TUI 上で pack を発見し（無料ランダム discovery 含む）、購入前に安全性・互換性を判断し、Web checkout で決済し、購入後に pack を導入できる体験を提供する。併せて、運営者による審査・掲載管理と、クリエイターへの月次売上分配の基盤を整える。

## 技術選定

### TUI: Rust + ratatui

- 環境に Rust 1.94.1 がインストール済み
- ratatui は Rust TUI エコシステムの標準的選択肢で、カスタム描画・アニメーション表現に強い
- Discovery のガチャガチャ演出（フレーム切り替え、色変化）を TUI 内で実現するのに適する
- crossterm をバックエンドに使用（macOS / Linux 対応）

### データストア: ローカル JSON ファイル

- MVP ではサーバーサイド DB を持たない
- カタログデータ: `data/catalog.json` に pack メタデータの配列を格納
- Entitlement データ: `data/entitlements.json` にユーザーの購入情報を格納
- 各 pack の `manifest.json` は pack ZIP 内に同梱（カタログ用にはメタデータを抽出して保持）

### 決済: Stripe Payment Links / Checkout

- TUI 内ではカード情報を入力しない（法務・UX の双方で Web 遷移が必須）
- Stripe Payment Links を pack ごとに生成し、TUI から URL を開く / QR を表示する
- 決済完了の検知は MVP では手動確認 or Stripe webhook の簡易受信（後述の Open Questions）

### 認証: MVP では簡易トークン方式

- ローカルに保存した API key / token で entitlement を照合
- OAuth 等の本格認証は MVP スコープ外

## MVP スコープ

### 含むもの

- TUI カタログ画面（featured / recent / recommended / 検索 / タグ絞り込み）
- 無料ランダム Discovery（ガチャガチャ演出: 待機 -> 演出 -> 結果の 3 状態）
- Pack 詳細画面（購入前プレビュー: summary、targets、権限、同梱カテゴリ件数、サンプル抜粋）
- 安全性詳細画面（permissions 5 項目、影響範囲、外部通信、review notes）
- 購入案内画面（価格、seller 情報、返金ポリシー要約、Stripe checkout URL、QR）
- 購入済みライブラリ画面（所有 pack 一覧、version、install 状態、update 有無）
- インストール詳細画面（method、command、post-install、rollback）
- manifest.json の読み込みとバリデーション
- Cherry Cartridge カラーパレットによる UI テーマ
- 法務表示（特商法表示、利用規約、返金ポリシーへの導線）

### 含まないもの

- 有料ガチャ / 排出率 / コンプ要素
- Creator 直接販売 / Open marketplace
- TUI 内でのカード情報入力
- チームプラン / Org billing / Private registry
- 海外 Creator 対応 / Cross-border tax
- モバイルアプリ
- Creator 向け提出 UI（MVP では手動受付）
- 自動 install 成功判定
- アカウント画面の本格実装（簡易的な認証状態表示のみ）

## 主要機能一覧

### F-1: カタログ閲覧

- ユーザーが featured / recent / recommended の pack 一覧を閲覧できる
- タグまたはキーワードで絞り込みできる
- 一覧から pack 詳細へ遷移できる
- 受入条件: TUI 起動後にカタログが表示され、キー操作で pack を選択して詳細画面に遷移できること

### F-2: 無料ランダム Discovery

- ユーザーが無料でランダムに 1 pack を表示できる
- ガチャガチャスクリーン（待機 -> 演出 -> 結果）の 3 状態遷移
- 演出は 1-2 秒で完了
- 結果から pack 詳細へ遷移できる / もう一度探せる
- 受入条件: Discovery 実行時に課金が発生しないこと。待機状態で Enter -> 演出 -> 結果表示の流れが動作すること。結果から詳細画面へ遷移できること

### F-3: Pack 詳細表示

- manifest.json に基づき、name、creator、price、summary、targets、version、更新日、同梱カテゴリ件数、権限サマリ、サンプル抜粋を表示
- prompt 全文、hook 全文、template 完全版、script 詳細実装は表示しない
- 受入条件: 表示内容が manifest.json と一致していること。全文コピー可能な粒度の情報が露出していないこと

### F-4: 安全性詳細表示

- shell / network / filesystem_read / filesystem_write / git の権限有無を表示
- 外部接続先の概要と影響範囲を表示
- 危険度の高い権限は視覚的に強調（danger カラー）
- 受入条件: permissions の 5 項目がすべて表示されること。danger 権限が目視で識別できること

### F-5: 購入案内と Web checkout 遷移

- 価格、seller 情報、返金ポリシー要約を表示
- Stripe checkout URL を表示し、ブラウザで開く / URL コピーの操作を提供
- QR コード表示（テキストベース QR）
- 受入条件: checkout URL が正しい Stripe Payment Link であること。ブラウザが起動すること

### F-6: 購入済みライブラリ

- entitlement に基づき購入済み pack を一覧表示
- version、install 状態、update 有無を表示
- install 詳細へ遷移できる
- 受入条件: 購入記録のある pack が一覧に表示されること。未購入 pack が表示されないこと

### F-7: インストール案内

- install method、command、手動手順、post-install 手順、rollback 注意を表示
- 受入条件: manifest.json の install フィールドに基づく手順が表示されること

### F-8: manifest.json バリデーション

- pack 読み込み時に必須フィールドの存在を検証
- contents に記載されたファイルの存在を検証（ローカル pack の場合）
- 受入条件: 必須フィールドが欠けた manifest でエラーが報告されること

## 画面一覧と遷移

### 画面一覧（7 画面 + Discovery 3 状態）

| #   | 画面名             | 役割                                            |
| --- | ------------------ | ----------------------------------------------- |
| 1   | ホーム / カタログ  | ストア入口。pack 一覧、検索、Discovery 導線     |
| 2   | Discovery（待機）  | ガチャガチャマシン待機。PRESS ENTER TO DISCOVER |
| 2a  | Discovery（演出）  | レバー操作 -> カプセル排出の記号的表現。1-2 秒  |
| 2b  | Discovery（結果）  | 推薦 pack のカード表示。詳細遷移 / 再実行       |
| 3   | Pack 詳細          | 購入判断の中心。metadata + プレビュー           |
| 4   | 安全性詳細         | permissions と影響範囲の詳細                    |
| 5   | 購入案内           | 決済前最終確認。Stripe checkout への遷移        |
| 6   | 購入済みライブラリ | 所有 pack 管理                                  |
| 7   | インストール詳細   | 購入後の導入支援                                |

### 画面遷移

```
ホーム / カタログ
  -> Discovery（待機）
  -> Pack 詳細
  -> 購入済みライブラリ
  -> ヘルプ / 法務情報（MVP では簡易表示）

Discovery（待機） -> Discovery（演出） -> Discovery（結果）
Discovery（結果）
  -> Pack 詳細
  -> Discovery（待機）  [もう一度探す]
  -> ホーム / カタログ

Pack 詳細
  -> 安全性詳細
  -> 購入案内
  -> ホーム / カタログ

安全性詳細
  -> Pack 詳細
  -> 購入案内

購入案内
  -> Web checkout（外部ブラウザ）
  -> Pack 詳細

購入済みライブラリ
  -> インストール詳細

インストール詳細
  -> 購入済みライブラリ
```

## データモデル概要

### カタログデータ (`data/catalog.json`)

manifest.json から抽出した pack メタデータの配列。各エントリは以下を含む:

```
{
  id: string,              // pack 一意識別子
  name: string,            // 表示名
  version: string,         // semver
  summary: string,         // 短い説明
  description?: string,    // 長い説明
  author: { name, url?, email? },
  targets: [{ tool, version_range }],
  contents_summary: {      // カテゴリ別件数（全ファイルパスは含まない）
    skills: number,
    hooks: number,
    templates: number,
    other: number
  },
  permissions: { shell, network, filesystem_read, filesystem_write, git },
  install: { method, entrypoint?, steps? },
  license: { type, text_url?, spdx? },
  tags?: string[],
  risks?: string[],
  price: number,           // 日本円（manifest 外、ストア側で設定）
  status: "listed" | "delisted" | "suspended",
  featured: boolean,
  listed_at: string,       // ISO 8601
  updated_at: string,
  sample_preview?: string, // 数行のプレビューテキスト
  checkout_url: string,    // Stripe Payment Link URL
  review_notes?: string    // 審査時の補足
}
```

### Entitlement データ (`data/entitlements.json`)

```
{
  user_id: string,
  entitlements: [
    {
      pack_id: string,
      purchased_at: string,    // ISO 8601
      version_at_purchase: string,
      status: "active" | "refunded" | "revoked",
      installed: boolean,
      installed_version?: string
    }
  ]
}
```

### Pack 配布物

- 各 pack は ZIP 形式
- ZIP ルートに `manifest.json` を含む
- `contents` フィールドに記載されたファイルを同梱
- 購入前はカタログデータのみ参照。購入後に ZIP をダウンロード可能

### 売上記録データ (`data/accounting.json`)

```
{
  transactions: [
    {
      pack_id: string,
      user_id: string,
      amount: number,
      type: "purchase" | "refund",
      timestamp: string,
      stripe_payment_id?: string
    }
  ],
  creator_shares: {
    [creator_name: string]: {
      share_rate: number,     // 0.0 - 1.0
      packs: string[]         // pack_id list
    }
  }
}
```

## 実装フェーズ分割案

### Phase 1: 基盤とカタログ表示

成果物:

- Rust プロジェクト初期化（Cargo.toml、ratatui + crossterm 依存）
- Cherry Cartridge カラーテーマの定義
- manifest.json パーサーとバリデーター
- カタログデータの読み込み
- ホーム / カタログ画面の描画と基本操作（一覧表示、選択、タブ切替）

依存: なし。他の全フェーズの土台。

### Phase 2: Pack 詳細 + 安全性詳細

成果物:

- Pack 詳細画面（metadata 表示、プレビュー境界の実装）
- 安全性詳細画面（permissions 表示、danger カラー強調）
- カタログ -> 詳細 -> 安全性の画面遷移

依存: Phase 1 のカタログ画面と manifest パーサー。

### Phase 3: Discovery（ガチャガチャ演出）

成果物:

- Discovery 画面の 3 状態（待機、演出、結果）
- ASCII アートによるガチャガチャマシン描画
- フレーム切り替えアニメーション（1-2 秒）
- ランダム pack 選択ロジック
- 結果から詳細画面への遷移

依存: Phase 1 のカタログデータ、Phase 2 の詳細画面。

### Phase 4: 購入導線と Entitlement

成果物:

- 購入案内画面（価格、seller、ポリシー、checkout URL、QR）
- ブラウザ起動 / URL コピー機能
- Entitlement データの読み書き
- 購入済みライブラリ画面
- インストール詳細画面

依存: Phase 2 の詳細画面。Stripe Payment Links の事前設定（外部作業）。

### Phase 5: 運用基盤と仕上げ

成果物:

- 売上記録データの読み書き
- 月次分配額の算出ロジック
- 法務表示（特商法表示、利用規約、返金ポリシーの導線）
- 検索・タグ絞り込みの実装
- エラーハンドリングの整備
- サンプル pack データの作成

依存: Phase 1-4 の全機能。

## 非機能要件サマリ

### セキュリティ

- 購入前に危険権限（shell、network）を明示表示する
- permissions 表示は manifest.json の実データに基づく
- TUI 内でカード情報・credential を入力させない
- pack の権限申告が虚偽でないことは審査（手動）で担保

### パフォーマンス

- カタログ画面の初期表示は体感的に即時（ローカル JSON 読み込みのため問題なし）
- Discovery 演出は 1-2 秒で完了
- TUI の入力応答は滑らかに保つ（60fps ターゲットは不要だが、キー入力に対して 100ms 以内の反応）

### 法務表示

- 日本向け特定商取引法に基づく表示（seller identity、価格、支払時期、引渡時期、返金ポリシー）
- 「有料ガチャ」ではなく「キュレーション型ストア」として説明
- Discovery は「無料のランダム推薦」であり、排出率・当たりハズレ・コンプ要素を前提としない
- OpenAI / Anthropic との公式提携を誤認させる表現を避ける

### 保守性

- pack 情報は manifest.json を正本とし、カタログデータは manifest から派生
- review checklist は文書化済み（docs/review-checklist.md）

## Open Questions

docs 内の未決事項を集約する。実装開始前に方針を決定するか、MVP では仮定を置いて進める。

| #    | 質問                                                             | 出典             | 仮定（未決定時）                                                         |
| ---- | ---------------------------------------------------------------- | ---------------- | ------------------------------------------------------------------------ |
| OQ-1 | 初期リリースで匿名閲覧を許可するか、checkout 前に sign-in 必須か | mvp-prd.md       | MVP ではカタログ閲覧は匿名可、購入時に sign-in 必須                      |
| OQ-2 | Install は CLI command ベースか、手動 ZIP 展開のみか             | mvp-prd.md       | MVP では両方サポート（manifest の install.method に依存）                |
| OQ-3 | Creator revenue share は全体固定か pack ごと変更可か             | mvp-prd.md       | MVP では pack ごとに設定可能（accounting.json の creator_shares で管理） |
| OQ-4 | 配布物は ZIP のみか、git-based install も扱うか                  | mvp-prd.md       | MVP では ZIP のみ                                                        |
| OQ-5 | Discovery 演出をどこまで強くするか                               | ui-design.md     | 記号・罫線・色変化による 1-2 秒の演出。物理シミュレーション不要          |
| OQ-6 | 商品詳細で主要ファイル名をどこまで見せるか                       | ui-design.md     | カテゴリ別件数 + 代表ファイル名（最大 3 件）まで                         |
| OQ-7 | permissions を boolean のみか、より詳細な scope を導入するか     | manifest-spec.md | MVP では boolean のみ                                                    |
| OQ-8 | Stripe webhook による決済完了の自動検知を MVP に含めるか         | --               | MVP では手動確認で開始。webhook 受信は Phase 5 以降で検討                |
| OQ-9 | install 成功状態を TUI 内でどこまで検知するか                    | ui-design.md     | MVP では手動で installed フラグを更新。自動検知は対象外                  |

## 受け入れ条件

- [ ] TUI を起動してカタログ画面が表示される
- [ ] カタログから pack を選択して詳細画面に遷移できる
- [ ] 詳細画面で manifest に基づく metadata が正しく表示される
- [ ] 詳細画面で prompt 全文や hook 全文が露出していない
- [ ] 安全性詳細画面で 5 つの permissions が表示され、危険権限が色で強調される
- [ ] Discovery で Enter -> 演出（1-2 秒） -> 結果の流れが動作する
- [ ] Discovery が無料で実行でき、課金が発生しない
- [ ] Discovery 結果から詳細画面へ遷移できる
- [ ] 購入案内画面で Stripe checkout URL が表示され、ブラウザが起動する
- [ ] 購入済みライブラリに entitlement のある pack が表示される
- [ ] インストール詳細画面で install 手順が表示される
- [ ] Cherry Cartridge カラーパレットが適用されている
- [ ] 法務表示（特商法表示への導線）が存在する
- [ ] 必須フィールドが欠けた manifest.json でバリデーションエラーが出る
- [ ] 月次分配額の算出ロジックが accounting データに基づいて動作する

## 評価観点

### 主要ユーザーフロー

- カタログ閲覧 -> pack 選択 -> 詳細確認 -> 購入案内 -> checkout URL 遷移
- Discovery 実行 -> 結果確認 -> 詳細遷移 -> 購入案内
- 購入済みライブラリ -> インストール詳細 -> 手順確認

### データ整合性

- manifest.json のバリデーションが正しく動作するか
- カタログデータと manifest の内容が一致するか
- Entitlement の状態遷移（active / refunded / revoked）が正しいか

### 情報境界

- 購入前に pack の価値判断に十分な情報が見えるか
- 購入前に pack を複製できる粒度の情報が露出していないか

### エラー時挙動

- カタログデータが空の場合に適切なメッセージが出るか
- manifest.json が不正な場合にクラッシュしないか
- ネットワーク未接続で checkout URL を開こうとした場合の挙動

### UI / UX

- Cherry Cartridge パレットが全画面で一貫して適用されているか
- Discovery 演出のテンポが適切か（長すぎない、短すぎない）
- キー操作が直感的か（wireframe のキーバインドに準拠）
- danger 権限の視覚的強調が十分か
