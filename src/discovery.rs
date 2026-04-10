use rand::seq::SliceRandom;

use crate::models::CatalogEntry;

pub fn pick_random_pack(catalog: &[CatalogEntry]) -> Option<String> {
    let listed: Vec<&CatalogEntry> = catalog.iter().filter(|p| p.status == "listed").collect();
    let mut rng = rand::thread_rng();
    listed.choose(&mut rng).map(|p| p.id.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_random_from_empty() {
        let catalog: Vec<CatalogEntry> = vec![];
        assert!(pick_random_pack(&catalog).is_none());
    }
}
