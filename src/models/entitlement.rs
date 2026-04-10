use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementEntry {
    pub pack_id: String,
    pub purchased_at: String,
    pub version_at_purchase: String,
    pub status: String,
    pub installed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementStore {
    pub user_id: String,
    pub entitlements: Vec<EntitlementEntry>,
}
