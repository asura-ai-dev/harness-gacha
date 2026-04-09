# Manifest 仕様書

## 目的

`manifest.json` は、各 harness pack の内容、互換性、導入方法、必要権限、作者情報を機械可読で表現するための標準ファイルである。ストア表示、審査、インストール、将来の自動検証をこのファイルに依存して行う。

## 役割

- TUI 商品詳細の表示元になる
- 審査時のチェック対象を標準化する
- 危険権限や外部通信の有無を購入前に表示する
- 購入後の install 手順を自動生成する
- 将来の互換性チェックや差分検知の土台にする

## ファイル要件

- ファイル名は `manifest.json` とする
- 文字コードは UTF-8 とする
- JSON object をトップレベルに持つこと
- コメントは含めないこと
- pack のルートディレクトリ直下に置くこと

## 設計方針

- 人が読めることより、まず機械が安定して読めることを優先する
- ストア表示に必要な最低情報は必須項目にする
- 安全性判断に必要な情報は明示項目にする
- pack 内の実ファイルと矛盾しないこと
- 将来拡張できるように schema version を持つ

## 必須フィールド

### `schema_version`

- 型: `string`
- 目的: manifest 仕様のバージョン識別
- 例: `"1.0"`

### `id`

- 型: `string`
- 目的: pack の一意識別子
- ルール: 英小文字、数字、`-` のみを許可する
- 例: `"team-review-pack"`

### `name`

- 型: `string`
- 目的: ストア表示名
- 例: `"Team Review Pack"`

### `version`

- 型: `string`
- 目的: pack 自体のバージョン
- ルール: semver 形式を推奨する
- 例: `"1.0.0"`

### `summary`

- 型: `string`
- 目的: pack の短い説明
- 例: `"コードレビュー向けの hooks と skill をまとめた pack"`

### `author`

- 型: `object`
- 目的: pack の作成者情報
- 必須子項目:
- `name`: `string`
- 任意子項目:
- `url`: `string`
- `email`: `string`

### `targets`

- 型: `array`
- 目的: 対応ツールと対応バージョンの宣言
- 最低 1 件必要
- 各要素の必須子項目:
- `tool`: `string`
- `version_range`: `string`
- `tool` の例:
- `"claude-code"`
- `"codex"`

### `contents`

- 型: `array`
- 目的: 同梱ファイルまたは主要エントリの一覧
- 各要素は pack ルートからの相対パスとする
- 例:
- `"skills/review.md"`
- `"hooks/pre-commit.sh"`

### `permissions`

- 型: `object`
- 目的: 実行時に必要な権限を明示する
- 必須子項目:
- `shell`: `boolean`
- `network`: `boolean`
- `filesystem_read`: `boolean`
- `filesystem_write`: `boolean`
- `git`: `boolean`

### `install`

- 型: `object`
- 目的: 購入後の導入方法を定義する
- 必須子項目:
- `method`: `string`
- 任意子項目:
- `entrypoint`: `string`
- `steps`: `array`
- `method` の許可値:
- `"copy"`
- `"script"`
- `"manual"`

### `license`

- 型: `object`
- 目的: 利用条件を示す
- 必須子項目:
- `type`: `string`
- 任意子項目:
- `text_url`: `string`
- `spdx`: `string`

## 推奨フィールド

### `description`

- 型: `string`
- 目的: 長めの商品説明

### `homepage`

- 型: `string`
- 目的: pack または作者の紹介ページ

### `repository`

- 型: `string`
- 目的: ソースまたは参照リポジトリ

### `pricing_hint`

- 型: `object`
- 目的: ストア側の価格決定の参考情報
- これは表示保証項目ではなく内部運用向け

### `tags`

- 型: `array`
- 目的: 検索と分類用タグ
- 例:
- `"review"`
- `"typescript"`
- `"frontend"`

### `risks`

- 型: `array`
- 目的: 権限フラグだけでは伝わらない注意事項
- 例:
- `"外部 API に接続する"`
- `"ローカルファイルを書き換える"`

### `post_install`

- 型: `array`
- 目的: 導入後にユーザーが行うべき追加設定

## フィールド定義の詳細

### `author`

```json
{
  "name": "Example Creator",
  "url": "https://example.com",
  "email": "creator@example.com"
}
```

### `targets`

```json
[
  {
    "tool": "claude-code",
    "version_range": ">=1.0.0"
  },
  {
    "tool": "codex",
    "version_range": ">=0.9.0"
  }
]
```

### `permissions`

```json
{
  "shell": true,
  "network": false,
  "filesystem_read": true,
  "filesystem_write": true,
  "git": false
}
```

### `install`

```json
{
  "method": "script",
  "entrypoint": "install.sh",
  "steps": [
    "chmod +x install.sh",
    "./install.sh"
  ]
}
```

## 完全な例

```json
{
  "schema_version": "1.0",
  "id": "team-review-pack",
  "name": "Team Review Pack",
  "version": "1.0.0",
  "summary": "コードレビュー向けの hooks と skill をまとめた pack",
  "description": "レビュー観点、handoff、pre-commit hook を含むチーム向け pack。",
  "author": {
    "name": "Example Creator",
    "url": "https://example.com"
  },
  "targets": [
    {
      "tool": "claude-code",
      "version_range": ">=1.0.0"
    },
    {
      "tool": "codex",
      "version_range": ">=0.9.0"
    }
  ],
  "contents": [
    "skills/review.md",
    "hooks/pre-commit.sh",
    "templates/handoff.md"
  ],
  "permissions": {
    "shell": true,
    "network": false,
    "filesystem_read": true,
    "filesystem_write": true,
    "git": false
  },
  "install": {
    "method": "script",
    "entrypoint": "install.sh",
    "steps": [
      "chmod +x install.sh",
      "./install.sh"
    ]
  },
  "license": {
    "type": "commercial",
    "text_url": "https://example.com/license"
  },
  "tags": [
    "review",
    "team",
    "typescript"
  ],
  "risks": [
    "hooks が作業ディレクトリ内のファイルを書き換える"
  ]
}
```

## バリデーションルール

- 必須フィールドがすべて存在すること
- 未知のトップレベルフィールドは許可するが、ストア側が無視できること
- `id` はストア全体で一意であること
- `contents` に列挙されたパスが実際に存在すること
- `install.entrypoint` が指定される場合、そのファイルが存在すること
- `permissions` が実際の pack 挙動と矛盾しないこと
- `targets` は少なくとも 1 つ以上あること
- `version` の更新時に、旧版との差分が審査で確認できること

## 禁止事項

- 実際より弱い権限申告
- 作者情報の偽装
- 互換性の虚偽表示
- 同梱していないファイルの記載
- 外部通信の未申告
- 実行時に追加取得する危険な payload の未申告

## 審査との関係

manifest は審査の入口であり、審査の代替ではない。審査では以下を manifest と突き合わせる。

- `contents` と実ファイルの一致
- `permissions` とスクリプト内容の一致
- `targets` と実際の対応環境
- `license` と配布許諾の整合

## ストア表示との関係

ストア画面では manifest から少なくとも以下を表示する。

- `name`
- `summary`
- `author.name`
- `targets`
- `contents`
- `permissions`
- `license.type`
- `version`

## バージョニング方針

- `schema_version` は manifest 仕様自体の版とする
- `version` は pack の版とする
- manifest 仕様が非互換変更を含む場合は `schema_version` を更新する
- pack 内容の変更は `version` を更新する

## 今後の拡張候補

- `screenshots`
- `checksum`
- `signature`
- `min_platform_capabilities`
- `supported_os`
- `post_purchase_notice`
- `review_notes`

## Open Questions

- `permissions` を boolean だけで表すか、より詳細な scope を導入するか
- `install.method` に `package-manager` 系を追加するか
- `license.type` の許可値を固定 enum にするか
- `targets.tool` の正式な許可値一覧をどこで管理するか
