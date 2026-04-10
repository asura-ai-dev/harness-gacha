use ratatui::prelude::*;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::app::App;
use crate::data::catalog::find_pack_by_id;
use crate::models::CatalogEntry;
use crate::ui::theme::Theme;
use crate::ui::widgets::permission_badge;

pub fn render(frame: &mut Frame, app: &App, theme: &Theme) {
    let Some(selected_pack_id) = app.selected_pack_id.as_deref() else {
        render_not_found(frame, theme);
        return;
    };

    let Some(pack) = find_pack_by_id(&app.catalog, selected_pack_id) else {
        render_not_found(frame, theme);
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_header(frame, chunks[0], pack, theme);
    render_content(frame, chunks[1], pack, theme);
    render_footer(frame, chunks[2], theme);
}

fn render_not_found(frame: &mut Frame, theme: &Theme) {
    let paragraph = Paragraph::new("Pack not found")
        .style(theme.text_style())
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, frame.area());
}

fn render_header(frame: &mut Frame, area: Rect, pack: &CatalogEntry, theme: &Theme) {
    let title = format!("Safety Details: {}", pack.name);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border_style())
        .style(theme.panel_style());
    let paragraph = Paragraph::new(title)
        .style(
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )
        .block(block);
    frame.render_widget(paragraph, area);
}

fn render_content(frame: &mut Frame, area: Rect, pack: &CatalogEntry, theme: &Theme) {
    let mut lines: Vec<Line> = vec![section_title("Permissions", theme), blank_line()];

    let permissions = [
        ("shell", pack.permissions.shell),
        ("network", pack.permissions.network),
        ("filesystem_read", pack.permissions.filesystem_read),
        ("filesystem_write", pack.permissions.filesystem_write),
        ("git", pack.permissions.git),
    ];

    for (name, enabled) in permissions {
        lines.push(permission_badge::permission_line(name, enabled, theme));
    }

    if let Some(risks) = &pack.risks {
        if !risks.is_empty() {
            lines.push(blank_line());
            lines.push(section_title("Risks", theme));
            lines.push(blank_line());
            for risk in risks {
                lines.push(Line::from(Span::styled(
                    format!("- {}", risk),
                    Style::default().fg(theme.warning),
                )));
            }
        }
    }

    if let Some(notes) = &pack.review_notes {
        lines.push(blank_line());
        lines.push(section_title("Review Notes", theme));
        lines.push(blank_line());
        lines.push(Line::from(Span::styled(
            notes.clone(),
            Style::default().fg(theme.text_secondary),
        )));
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        )
        .wrap(Wrap { trim: false })
        .style(theme.text_style());
    frame.render_widget(paragraph, area);
}

fn render_footer(frame: &mut Frame, area: Rect, theme: &Theme) {
    let paragraph = Paragraph::new("[p] 購入案内  [b] 詳細へ戻る")
        .style(theme.footer_text_style())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        );
    frame.render_widget(paragraph, area);
}

fn section_title<'a>(title: &'a str, theme: &Theme) -> Line<'a> {
    Line::from(Span::styled(
        title,
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    ))
}

fn blank_line() -> Line<'static> {
    Line::from("")
}
