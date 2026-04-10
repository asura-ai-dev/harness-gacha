use ratatui::prelude::*;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::app::App;
use crate::data::catalog::find_pack_by_id;
use crate::models::CatalogEntry;
use crate::ui::theme::Theme;

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
    render_content(frame, chunks[1], pack, app.scroll_offset, theme);
    render_footer(frame, chunks[2], theme);
}

fn render_not_found(frame: &mut Frame, theme: &Theme) {
    let paragraph = Paragraph::new("Pack not found")
        .style(theme.text_style())
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, frame.area());
}

fn render_header(frame: &mut Frame, area: Rect, pack: &CatalogEntry, theme: &Theme) {
    let paragraph = Paragraph::new(format!("Install: {}", pack.name))
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
    frame.render_widget(paragraph, area);
}

fn render_content(frame: &mut Frame, area: Rect, pack: &CatalogEntry, scroll: u16, theme: &Theme) {
    let mut lines = vec![section_title("Method", theme)];
    lines.push(Line::from(Span::styled(
        format!("  {}", pack.install.method),
        Style::default().fg(theme.text_primary),
    )));
    lines.push(blank_line());

    if let Some(entrypoint) = pack.install.entrypoint.as_deref() {
        lines.push(section_title("Entrypoint", theme));
        lines.push(Line::from(Span::styled(
            format!("  {}", entrypoint),
            Style::default().fg(theme.text_primary),
        )));
        lines.push(blank_line());
    }

    if let Some(steps) = &pack.install.steps {
        if !steps.is_empty() {
            lines.push(section_title("Command", theme));
            for step in steps {
                lines.push(Line::from(Span::styled(
                    format!("  $ {}", step),
                    Style::default().fg(theme.owned),
                )));
            }
            lines.push(blank_line());
        }
    }

    lines.push(section_title("Post Install", theme));
    lines.push(Line::from(Span::styled(
        "  - 設定ファイルを確認してください",
        Style::default().fg(theme.text_secondary),
    )));
    lines.push(Line::from(Span::styled(
        "  - 必要に応じて hooks を有効化してください",
        Style::default().fg(theme.text_secondary),
    )));
    lines.push(blank_line());

    lines.push(section_title("Rollback", theme));
    lines.push(Line::from(Span::styled(
        "  - 追加ファイルを削除してください",
        Style::default().fg(theme.text_secondary),
    )));
    lines.push(Line::from(Span::styled(
        "  - 変更前のバックアップを復元してください",
        Style::default().fg(theme.text_secondary),
    )));

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0))
        .style(theme.text_style());
    frame.render_widget(paragraph, area);
}

fn render_footer(frame: &mut Frame, area: Rect, theme: &Theme) {
    let paragraph = Paragraph::new("[j/k] スクロール  [b] 戻る")
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
