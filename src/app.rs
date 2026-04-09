use crate::action::Action;
use crate::data::{accounting, catalog, entitlement};
use crate::models::{AccountingData, CatalogEntry, EntitlementStore};
use crate::screen::Screen;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogTab {
    Featured,
    Recent,
    Recommended,
}

#[derive(Debug, Clone)]
pub enum DiscoveryState {
    Idle,
    Animating { frame: u8, target_pack_id: String },
    Result { pack_id: String },
}

#[derive(Debug)]
pub struct CatalogState {
    pub current_tab: CatalogTab,
    pub selected_index: usize,
    pub filtered_ids: Vec<String>,
    pub active_tag: Option<String>,
}

#[derive(Debug)]
pub struct LibraryState {
    pub selected_index: usize,
}

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
    pub scroll_offset: u16,
    pub message: Option<String>,
}

impl App {
    pub fn new(data_dir: &Path) -> Self {
        let catalog = catalog::load_catalog(&data_dir.join("catalog.json"));
        let entitlements = entitlement::load_entitlements(&data_dir.join("entitlements.json"));
        let accounting_data = accounting::load_accounting(&data_dir.join("accounting.json"));
        let filtered_ids: Vec<String> = catalog
            .iter()
            .filter(|p| p.featured && p.status == "listed")
            .map(|p| p.id.clone())
            .collect();
        let message = if catalog.is_empty() {
            Some("カタログデータが見つかりません。data/catalog.json を確認してください。".to_string())
        } else {
            None
        };
        App {
            running: true,
            current_screen: Screen::Catalog,
            screen_stack: Vec::new(),
            catalog,
            entitlements,
            accounting: accounting_data,
            catalog_state: CatalogState {
                current_tab: CatalogTab::Featured,
                selected_index: 0,
                filtered_ids,
                active_tag: None,
            },
            discovery_state: DiscoveryState::Idle,
            library_state: LibraryState { selected_index: 0 },
            selected_pack_id: None,
            search_query: String::new(),
            search_active: false,
            tick_count: 0,
            scroll_offset: 0,
            message,
        }
    }

    pub fn navigate_to(&mut self, screen: Screen) {
        self.screen_stack.push(self.current_screen);
        self.current_screen = screen;
        self.scroll_offset = 0;
    }

    pub fn go_back(&mut self) {
        if let Some(prev) = self.screen_stack.pop() {
            self.current_screen = prev;
            self.scroll_offset = 0;
        }
    }

    pub fn refresh_filtered_ids(&mut self) {
        let tag_filter = self.catalog_state.active_tag.clone();
        let query = self.search_query.to_lowercase();

        let base_iter = self.catalog.iter().filter(|p| p.status == "listed");

        let tab_filtered: Vec<&CatalogEntry> = match self.catalog_state.current_tab {
            CatalogTab::Featured => base_iter.filter(|p| p.featured).collect(),
            CatalogTab::Recent => {
                let mut v: Vec<&CatalogEntry> = base_iter.collect();
                v.sort_by(|a, b| b.listed_at.cmp(&a.listed_at));
                v
            }
            CatalogTab::Recommended => base_iter.collect(),
        };

        self.catalog_state.filtered_ids = tab_filtered
            .into_iter()
            .filter(|p| {
                if let Some(ref tag) = tag_filter {
                    p.tags.as_ref().map_or(false, |tags| tags.contains(tag))
                } else {
                    true
                }
            })
            .filter(|p| {
                if query.is_empty() {
                    true
                } else {
                    p.name.to_lowercase().contains(&query)
                        || p.summary.to_lowercase().contains(&query)
                }
            })
            .map(|p| p.id.clone())
            .collect();

        self.catalog_state.selected_index = 0;
    }

    pub fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.running = false,
            Action::Back => self.go_back(),
            Action::Tick => self.tick_count += 1,
            _ => self.handle_screen_action(action),
        }
    }

    fn handle_screen_action(&mut self, action: Action) {
        match self.current_screen {
            Screen::Catalog => self.handle_catalog_action(action),
            Screen::PackDetail => match action {
                Action::OpenSafety => self.navigate_to(Screen::SafetyDetail),
                Action::OpenPurchase => self.navigate_to(Screen::Purchase),
                Action::Up => {
                    self.scroll_offset = self.scroll_offset.saturating_sub(1);
                }
                Action::Down => {
                    self.scroll_offset = self.scroll_offset.saturating_add(1);
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_catalog_action(&mut self, action: Action) {
        match action {
            Action::Up => {
                if self.catalog_state.selected_index > 0 {
                    self.catalog_state.selected_index -= 1;
                }
            }
            Action::Down => {
                let max = self.catalog_state.filtered_ids.len().saturating_sub(1);
                if self.catalog_state.selected_index < max {
                    self.catalog_state.selected_index += 1;
                }
            }
            Action::Tab => {
                self.catalog_state.current_tab = match self.catalog_state.current_tab {
                    CatalogTab::Featured => CatalogTab::Recent,
                    CatalogTab::Recent => CatalogTab::Recommended,
                    CatalogTab::Recommended => CatalogTab::Featured,
                };
                self.refresh_filtered_ids();
            }
            Action::BackTab => {
                self.catalog_state.current_tab = match self.catalog_state.current_tab {
                    CatalogTab::Featured => CatalogTab::Recommended,
                    CatalogTab::Recent => CatalogTab::Featured,
                    CatalogTab::Recommended => CatalogTab::Recent,
                };
                self.refresh_filtered_ids();
            }
            Action::Enter => {
                if let Some(id) = self
                    .catalog_state
                    .filtered_ids
                    .get(self.catalog_state.selected_index)
                {
                    self.selected_pack_id = Some(id.clone());
                    self.navigate_to(Screen::PackDetail);
                }
            }
            Action::ToggleDiscovery => {
                self.navigate_to(Screen::Discovery);
            }
            Action::OpenLibrary => {
                self.navigate_to(Screen::Library);
            }
            Action::OpenHelp => {
                self.navigate_to(Screen::Help);
            }
            Action::Search => {
                self.search_active = !self.search_active;
                if !self.search_active {
                    self.refresh_filtered_ids();
                }
            }
            Action::SearchInput(c) => {
                if self.search_active {
                    self.search_query.push(c);
                    self.refresh_filtered_ids();
                }
            }
            Action::SearchBackspace => {
                if self.search_active {
                    self.search_query.pop();
                    self.refresh_filtered_ids();
                }
            }
            Action::ToggleTag => {
                let tags = crate::data::catalog::all_tags(&self.catalog);
                if tags.is_empty() {
                    return;
                }
                self.catalog_state.active_tag = match &self.catalog_state.active_tag {
                    None => Some(tags[0].clone()),
                    Some(current) => {
                        if let Some(pos) = tags.iter().position(|t| t == current) {
                            if pos + 1 < tags.len() {
                                Some(tags[pos + 1].clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                };
                self.catalog_state.selected_index = 0;
                self.refresh_filtered_ids();
            }
            _ => {}
        }
    }
}
