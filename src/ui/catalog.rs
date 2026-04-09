use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs};

use crate::app::{App, CatalogTab};
use crate::data::catalog as catalog_data;
use crate::ui::theme::Theme;

pub fn render(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_title_bar(frame, chunks[0], app, theme);
    render_tab_bar(frame, chunks[1], app, theme);
    render_pack_list(frame, chunks[2], app, theme);
    render_footer(frame, chunks[3], theme);
}

fn render_title_bar(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let title = if app.search_active {
        format!(
            "harness-gacha                    Search: [{}]",
            app.search_query
        )
    } else {
        "harness-gacha".to_string()
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border_style())
        .style(theme.panel_style());
    let p = Paragraph::new(title).style(theme.text_style()).block(block);
    frame.render_widget(p, area);
}

fn render_tab_bar(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let tab_titles = vec!["Featured", "Recent", "Recommended"];
    let selected = match app.catalog_state.current_tab {
        CatalogTab::Featured => 0,
        CatalogTab::Recent => 1,
        CatalogTab::Recommended => 2,
    };
    let tabs = Tabs::new(tab_titles)
        .select(selected)
        .style(theme.secondary_style())
        .highlight_style(Style::default().fg(theme.accent).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(theme.border_style()),
        );
    frame.render_widget(tabs, area);
}

fn render_pack_list(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let items: Vec<ListItem> = app
        .catalog_state
        .filtered_ids
        .iter()
        .filter_map(|id| catalog_data::find_pack_by_id(&app.catalog, id))
        .map(|pack| {
            let targets_str = pack
                .targets
                .iter()
                .map(|t| t.tool.as_str())
                .collect::<Vec<_>>()
                .join(" / ");
            let trust_label = if pack.permissions.has_danger() {
                "Risk: High"
            } else {
                "Safe: Low"
            };
            let line = format!(
                "  {}    {}    YEN {}    {}",
                pack.name, targets_str, pack.price, trust_label
            );
            ListItem::new(line).style(theme.text_style())
        })
        .collect();

    if items.is_empty() {
        let msg = Paragraph::new("pack が見つかりません")
            .style(theme.secondary_style())
            .alignment(Alignment::Center);
        frame.render_widget(msg, area);
        return;
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(theme.border_style()),
        )
        .highlight_style(theme.highlight_style())
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(Some(app.catalog_state.selected_index));
    frame.render_stateful_widget(list, area, &mut state);
}

fn render_footer(frame: &mut Frame, area: Rect, theme: &Theme) {
    let footer_text =
        "[Enter] 詳細  [/] 検索  [Tab] タブ  [r] Discovery  [L] 購入済み  [?] ヘルプ  [q] 終了";
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border_style());
    let p = Paragraph::new(footer_text)
        .style(theme.footer_text_style())
        .block(block);
    frame.render_widget(p, area);
}
