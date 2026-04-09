use ratatui::prelude::*;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

use crate::app::App;
use crate::data::catalog::find_pack_by_id;
use crate::data::entitlement;
use crate::ui::theme::Theme;

pub fn render(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new("My Library")
        .style(
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        );
    frame.render_widget(header, chunks[0]);

    render_list(frame, chunks[1], app, theme);

    let footer = Paragraph::new("[Enter] Install 詳細  [b] 戻る")
        .style(theme.footer_text_style())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style()),
        );
    frame.render_widget(footer, chunks[2]);
}

fn render_list(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let active = entitlement::active_entitlements(&app.entitlements);
    if active.is_empty() {
        let msg = Paragraph::new("購入済み pack はありません")
            .style(theme.text_style())
            .alignment(Alignment::Center);
        frame.render_widget(msg, area);
        return;
    }

    let items: Vec<ListItem> = active
        .iter()
        .map(|ent| {
            let pack = find_pack_by_id(&app.catalog, &ent.pack_id);
            let name = pack
                .map(|p| p.name.as_str())
                .unwrap_or(ent.pack_id.as_str());
            let version = pack.map(|p| p.version.as_str()).unwrap_or("?");

            let install_status = if ent.installed {
                "Installed"
            } else {
                "Not Installed"
            };
            let install_color = if ent.installed {
                theme.owned
            } else {
                theme.text_secondary
            };

            let update_status = if ent.installed {
                match (&ent.installed_version, pack) {
                    (Some(installed_version), Some(pack)) if installed_version != &pack.version => {
                        "Update: available"
                    }
                    _ => "Update: none",
                }
            } else {
                ""
            };
            let update_color = if update_status == "Update: available" {
                theme.warning
            } else {
                theme.text_secondary
            };

            let line = Line::from(vec![
                Span::styled(
                    format!("  {:<25}", name),
                    Style::default().fg(theme.text_primary),
                ),
                Span::styled(
                    format!("v{:<10}", version),
                    Style::default().fg(theme.text_secondary),
                ),
                Span::styled(
                    format!("{:<16}", install_status),
                    Style::default().fg(install_color),
                ),
                Span::styled(update_status.to_string(), Style::default().fg(update_color)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(theme.border_style()),
        )
        .highlight_style(theme.highlight_style())
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(Some(app.library_state.selected_index));
    frame.render_stateful_widget(list, area, &mut state);
}
