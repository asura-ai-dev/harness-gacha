# Data Models - データモデル詳細

## 概要

harness-gacha が扱う全データ構造を定義する。Rust の struct 定義案と、対応する JSON スキーマの構造を示す。全モデルは `src/models/` に配置する。

## 仕様からの対応

- F-1: カタログ閲覧 → CatalogEntry
- F-2: Discovery → CatalogEntry（ランダム選択対象）
- F-3: Pack 詳細 → CatalogEntry の各フィールド
- F-4: 安全性詳細 → Permissions
- F-5: 購入案内 → CatalogEntry の price, checkout_url
- F-6: 購入済みライブラリ → EntitlementStore, EntitlementEntry
- F-7: インストール案内 → InstallInfo
- F-8: manifest バリデーション → Manifest

## 設計

### Permissions (`src/models/permissions.rs`)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Permissions {
    pub shell: bool,
    pub network: bool,
    pub filesystem_read: bool,
    pub filesystem_write: bool,
    pub git: bool,
}

impl Permissions {
    /// danger レベルの権限が有効かどうか
    /// shell と network を danger とみなす
    pub fn has_danger(&self) -> bool {
        self.shell || self.network
    }

    /// 有効な権限の一覧を返す
    pub fn enabled_list(&self) -> Vec<&str> {
        let mut list = Vec::new();
        if self.shell { list.push("shell"); }
        if self.network { list.push("network"); }
        if self.filesystem_read { list.push("filesystem_read"); }
        if self.filesystem_write { list.push("filesystem_write"); }
        if self.git { list.push("git"); }
        list
    }
}
```

### Author (`src/models/catalog_entry.rs` 内)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
```

### Target (`src/models/catalog_entry.rs` 内)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub tool: String,
    pub version_range: String,
}
```

### ContentsSummary (`src/models/catalog_entry.rs` 内)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentsSummary {
    #[serde(default)]
    pub skills: u32,
    #[serde(default)]
    pub hooks: u32,
    #[serde(default)]
    pub templates: u32,
    #[serde(default)]
    pub other: u32,
}
```

### InstallInfo (`src/models/catalog_entry.rs` 内)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallInfo {
    pub method: String, // "copy" | "script" | "manual"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<String>>,
}
```

### LicenseInfo (`src/models/catalog_entry.rs` 内)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    #[serde(rename = "type")]
    pub license_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx: Option<String>,
}
```

### CatalogEntry (`src/models/catalog_entry.rs`)

`data/catalog.json` の各エントリに対応する。

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    pub id: String,
    pub name: String,
    pub version: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub author: Author,
    pub targets: Vec<Target>,
    pub contents_summary: ContentsSummary,
    pub permissions: Permissions,
    pub install: InstallInfo,
    pub license: LicenseInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risks: Option<Vec<String>>,
    pub price: u32,          // 日本円
    pub status: String,      // "listed" | "delisted" | "suspended"
    pub featured: bool,
    pub listed_at: String,   // ISO 8601
    pub updated_at: String,  // ISO 8601
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_preview: Option<String>,
    pub checkout_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_notes: Option<String>,
}
```

### EntitlementEntry (`src/models/entitlement.rs`)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementEntry {
    pub pack_id: String,
    pub purchased_at: String,         // ISO 8601
    pub version_at_purchase: String,
    pub status: String,               // "active" | "refunded" | "revoked"
    pub installed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementStore {
    pub user_id: String,
    pub entitlements: Vec<EntitlementEntry>,
}
```

### Accounting (`src/models/accounting.rs`)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub pack_id: String,
    pub user_id: String,
    pub amount: i64,         // 日本円（refund は負値ではなく type で区別）
    #[serde(rename = "type")]
    pub tx_type: String,     // "purchase" | "refund"
    pub timestamp: String,   // ISO 8601
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_payment_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorShare {
    pub share_rate: f64,     // 0.0 - 1.0
    pub packs: Vec<String>,  // pack_id list
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingData {
    pub transactions: Vec<Transaction>,
    pub creator_shares: std::collections::HashMap<String, CreatorShare>,
}
```

### Manifest (`src/models/manifest.rs`)

manifest.json の完全な型表現。カタログデータとは別に、バリデーション用に使う。

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub schema_version: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub author: Author,
    pub targets: Vec<Target>,
    pub contents: Vec<String>,
    pub permissions: Permissions,
    pub install: InstallInfo,
    pub license: LicenseInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_install: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}
```

## JSON ファイル構造

### data/catalog.json

```json
[
  {
    "id": "team-review-pack",
    "name": "Team Review Pack",
    "version": "1.0.0",
    "summary": "コードレビュー向けの hooks と skill をまとめた pack",
    "author": { "name": "Example Creator" },
    "targets": [{ "tool": "claude-code", "version_range": ">=1.0.0" }],
    "contents_summary": { "skills": 3, "hooks": 2, "templates": 4, "other": 0 },
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
      "steps": ["chmod +x install.sh", "./install.sh"]
    },
    "license": { "type": "commercial" },
    "tags": ["review", "team"],
    "price": 1800,
    "status": "listed",
    "featured": true,
    "listed_at": "2026-04-01T00:00:00Z",
    "updated_at": "2026-04-10T00:00:00Z",
    "sample_preview": "PR review の際は、変更の意図と回帰リスクを先に...",
    "checkout_url": "https://buy.stripe.com/example_team_review"
  }
]
```

### data/entitlements.json

```json
{
  "user_id": "local-user",
  "entitlements": [
    {
      "pack_id": "team-review-pack",
      "purchased_at": "2026-04-05T10:00:00Z",
      "version_at_purchase": "1.0.0",
      "status": "active",
      "installed": true,
      "installed_version": "1.0.0"
    }
  ]
}
```

### data/accounting.json

```json
{
  "transactions": [
    {
      "pack_id": "team-review-pack",
      "user_id": "local-user",
      "amount": 1800,
      "type": "purchase",
      "timestamp": "2026-04-05T10:00:00Z",
      "stripe_payment_id": "pi_example_123"
    }
  ],
  "creator_shares": {
    "Example Creator": {
      "share_rate": 0.7,
      "packs": ["team-review-pack"]
    }
  }
}
```

## App 状態構造体 (`src/app.rs`)

```rust
pub struct App {
    pub running: bool,
    pub current_screen: Screen,
    pub screen_stack: Vec<Screen>,
    pub catalog: Vec<CatalogEntry>,
    pub entitlements: EntitlementStore,
    pub accounting: AccountingData,
    pub catalog_state: CatalogState,
    pub discovery_state: DiscoveryState,
    pub library_state: LibraryState,
    pub selected_pack_id: Option<String>,
    pub search_query: String,
    pub search_active: bool,
    pub tick_count: u64,
    pub message: Option<String>,  // ステータスバー用メッセージ
}

pub struct CatalogState {
    pub current_tab: CatalogTab, // Featured, Recent, Recommended
    pub selected_index: usize,
    pub filtered_ids: Vec<String>,
    pub active_tag: Option<String>,
}

pub enum CatalogTab {
    Featured,
    Recent,
    Recommended,
}

pub enum DiscoveryState {
    Idle,
    Animating { frame: u8, target_pack_id: String },
    Result { pack_id: String },
}

pub struct LibraryState {
    pub selected_index: usize,
}
```

## 制約・注意事項

- `price` は `u32` で日本円の整数値。小数点以下なし
- `status` フィールドは文字列型。enum にする場合は serde の rename を使う
- `CatalogEntry` と `Manifest` は似ているが役割が異なる。CatalogEntry はストア表示用（price, checkout_url 等を含む）、Manifest は pack 同梱の生データ
- JSON ファイルの読み書きは `src/data/` モジュールに集約する。直接 `serde_json::from_str` を各所で呼ばない
- `EntitlementStore` は MVP では単一ユーザー前提。user_id は固定値 "local-user" でよい
