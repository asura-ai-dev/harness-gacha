use std::path::Path;

use crate::models::entitlement::{EntitlementEntry, EntitlementStore};

pub fn load_entitlements(path: &Path) -> EntitlementStore {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            serde_json::from_str::<EntitlementStore>(&content).unwrap_or_else(|_| default_store())
        }
        Err(_) => default_store(),
    }
}

pub fn save_entitlements(path: &Path, store: &EntitlementStore) -> Result<(), String> {
    let content = serde_json::to_string_pretty(store)
        .map_err(|e| format!("Entitlement データのシリアライズに失敗しました: {}", e))?;

    std::fs::write(path, content)
        .map_err(|e| format!("Entitlement データの書き込みに失敗しました: {}", e))
}

pub fn default_store() -> EntitlementStore {
    EntitlementStore {
        user_id: "local-user".to_string(),
        entitlements: Vec::new(),
    }
}

pub fn active_entitlements(store: &EntitlementStore) -> Vec<&EntitlementEntry> {
    store
        .entitlements
        .iter()
        .filter(|entry| entry.status == "active")
        .collect()
}

pub fn is_owned(store: &EntitlementStore, pack_id: &str) -> bool {
    store
        .entitlements
        .iter()
        .any(|entry| entry.pack_id == pack_id && entry.status == "active")
}

pub fn find_entitlement<'a>(
    store: &'a EntitlementStore,
    pack_id: &str,
) -> Option<&'a EntitlementEntry> {
    store
        .entitlements
        .iter()
        .find(|entry| entry.pack_id == pack_id)
}

#[cfg(test)]
mod tests {
    use super::{
        active_entitlements, default_store, find_entitlement, is_owned, EntitlementEntry,
        EntitlementStore,
    };

    fn sample_store() -> EntitlementStore {
        EntitlementStore {
            user_id: "local-user".to_string(),
            entitlements: vec![
                EntitlementEntry {
                    pack_id: "pack-a".to_string(),
                    purchased_at: "2026-04-05T10:00:00Z".to_string(),
                    version_at_purchase: "1.0.0".to_string(),
                    status: "active".to_string(),
                    installed: true,
                    installed_version: Some("1.0.0".to_string()),
                },
                EntitlementEntry {
                    pack_id: "pack-b".to_string(),
                    purchased_at: "2026-04-06T10:00:00Z".to_string(),
                    version_at_purchase: "1.0.0".to_string(),
                    status: "refunded".to_string(),
                    installed: false,
                    installed_version: None,
                },
            ],
        }
    }

    #[test]
    fn test_active_entitlements() {
        let store = sample_store();
        let active = active_entitlements(&store);

        assert_eq!(active.len(), 1);
        assert_eq!(active[0].pack_id, "pack-a");
    }

    #[test]
    fn test_is_owned() {
        let store = sample_store();

        assert!(is_owned(&store, "pack-a"));
        assert!(!is_owned(&store, "pack-b"));
        assert!(!is_owned(&store, "pack-c"));
    }

    #[test]
    fn test_find_entitlement() {
        let store = sample_store();

        assert!(find_entitlement(&store, "pack-a").is_some());
        assert!(find_entitlement(&store, "nonexistent").is_none());
    }

    #[test]
    fn test_default_store() {
        let store = default_store();

        assert_eq!(store.user_id, "local-user");
        assert!(store.entitlements.is_empty());
    }
}
