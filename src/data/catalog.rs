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
    catalog
        .iter()
        .filter(|p| p.featured && p.status == "listed")
        .collect()
}

pub fn recent_packs(catalog: &[CatalogEntry]) -> Vec<&CatalogEntry> {
    let mut packs: Vec<&CatalogEntry> = listed_packs(catalog);
    packs.sort_by(|a, b| b.listed_at.cmp(&a.listed_at));
    packs
}

pub fn find_pack_by_id<'a>(catalog: &'a [CatalogEntry], id: &str) -> Option<&'a CatalogEntry> {
    catalog.iter().find(|p| p.id == id)
}

pub fn search_packs<'a>(catalog: &'a [CatalogEntry], query: &str) -> Vec<&'a CatalogEntry> {
    if query.trim().is_empty() {
        return listed_packs(catalog);
    }
    let q = query.to_lowercase();
    catalog
        .iter()
        .filter(|p| p.status == "listed")
        .filter(|p| {
            p.name.to_lowercase().contains(&q)
                || p.summary.to_lowercase().contains(&q)
                || p
                    .description
                    .as_ref()
                    .is_some_and(|d| d.to_lowercase().contains(&q))
        })
        .collect()
}

pub fn filter_by_tag<'a>(catalog: &'a [CatalogEntry], tag: &str) -> Vec<&'a CatalogEntry> {
    catalog
        .iter()
        .filter(|p| p.status == "listed")
        .filter(|p| {
            p.tags
                .as_ref()
                .is_some_and(|tags| tags.iter().any(|t| t == tag))
        })
        .collect()
}

pub fn all_tags(catalog: &[CatalogEntry]) -> Vec<String> {
    let mut tags: Vec<String> = catalog
        .iter()
        .filter(|p| p.status == "listed")
        .filter_map(|p| p.tags.as_ref())
        .flatten()
        .cloned()
        .collect();
    tags.sort();
    tags.dedup();
    tags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::catalog_entry::*;
    use crate::models::permissions::Permissions;

    fn sample_catalog() -> Vec<CatalogEntry> {
        let perms = Permissions {
            shell: false,
            network: false,
            filesystem_read: true,
            filesystem_write: false,
            git: false,
        };
        vec![
            CatalogEntry {
                id: "pack-a".to_string(),
                name: "Alpha Pack".to_string(),
                version: "1.0.0".to_string(),
                summary: "A test alpha pack".to_string(),
                description: None,
                author: Author {
                    name: "Test".to_string(),
                    url: None,
                    email: None,
                },
                targets: vec![Target {
                    tool: "claude-code".to_string(),
                    version_range: ">=1.0.0".to_string(),
                }],
                contents_summary: ContentsSummary {
                    skills: 1,
                    hooks: 0,
                    templates: 0,
                    other: 0,
                },
                permissions: perms.clone(),
                install: InstallInfo {
                    method: "copy".to_string(),
                    entrypoint: None,
                    steps: None,
                },
                license: LicenseInfo {
                    license_type: "commercial".to_string(),
                    text_url: None,
                    spdx: None,
                },
                tags: Some(vec!["review".to_string(), "team".to_string()]),
                risks: None,
                price: 1000,
                status: "listed".to_string(),
                featured: true,
                listed_at: "2026-04-01T00:00:00Z".to_string(),
                updated_at: "2026-04-01T00:00:00Z".to_string(),
                sample_preview: None,
                checkout_url: "https://example.com".to_string(),
                review_notes: None,
            },
            CatalogEntry {
                id: "pack-b".to_string(),
                name: "Beta Kit".to_string(),
                version: "1.0.0".to_string(),
                summary: "A beta frontend kit".to_string(),
                description: None,
                author: Author {
                    name: "Test".to_string(),
                    url: None,
                    email: None,
                },
                targets: vec![Target {
                    tool: "codex".to_string(),
                    version_range: ">=0.9.0".to_string(),
                }],
                contents_summary: ContentsSummary {
                    skills: 0,
                    hooks: 1,
                    templates: 2,
                    other: 0,
                },
                permissions: perms.clone(),
                install: InstallInfo {
                    method: "copy".to_string(),
                    entrypoint: None,
                    steps: None,
                },
                license: LicenseInfo {
                    license_type: "commercial".to_string(),
                    text_url: None,
                    spdx: None,
                },
                tags: Some(vec!["frontend".to_string()]),
                risks: None,
                price: 500,
                status: "listed".to_string(),
                featured: false,
                listed_at: "2026-04-05T00:00:00Z".to_string(),
                updated_at: "2026-04-05T00:00:00Z".to_string(),
                sample_preview: None,
                checkout_url: "https://example.com".to_string(),
                review_notes: None,
            },
        ]
    }

    #[test]
    fn test_search_empty_query_returns_all_listed() {
        let catalog = sample_catalog();
        let results = search_packs(&catalog, "");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_by_name() {
        let catalog = sample_catalog();
        let results = search_packs(&catalog, "Alpha");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "pack-a");
    }

    #[test]
    fn test_filter_by_tag() {
        let catalog = sample_catalog();
        let results = filter_by_tag(&catalog, "frontend");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "pack-b");
    }

    #[test]
    fn test_all_tags_sorted_and_unique() {
        let catalog = sample_catalog();
        let tags = all_tags(&catalog);
        assert_eq!(tags, vec!["frontend", "review", "team"]);
    }
}
