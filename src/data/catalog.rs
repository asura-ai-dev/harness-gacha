use std::path::Path;

use crate::models::CatalogEntry;

pub fn load_catalog(path: &Path) -> Vec<CatalogEntry> {
    match std::fs::read_to_string(path) {
        Ok(content) => serde_json::from_str::<Vec<CatalogEntry>>(&content).unwrap_or_else(|e| {
            eprintln!("catalog.json パースエラー: {}", e);
            Vec::new()
        }),
        Err(e) => {
            eprintln!("catalog.json 読み込みエラー: {}", e);
            Vec::new()
        }
    }
}

pub fn listed_packs(catalog: &[CatalogEntry]) -> Vec<&CatalogEntry> {
    catalog.iter().filter(|p| p.status == "listed").collect()
}

pub fn featured_packs(catalog: &[CatalogEntry]) -> Vec<&CatalogEntry> {
    catalog.iter().filter(|p| p.featured && p.status == "listed").collect()
}

pub fn recent_packs(catalog: &[CatalogEntry]) -> Vec<&CatalogEntry> {
    let mut packs: Vec<&CatalogEntry> = listed_packs(catalog);
    packs.sort_by(|a, b| b.listed_at.cmp(&a.listed_at));
    packs
}

pub fn find_pack_by_id<'a>(catalog: &'a [CatalogEntry], id: &str) -> Option<&'a CatalogEntry> {
    catalog.iter().find(|p| p.id == id)
}
