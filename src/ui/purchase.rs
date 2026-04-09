use ratatui::prelude::*;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::app::App;
use crate::data::catalog::find_pack_by_id;
use crate::models::CatalogEntry;
use crate::ui::theme::Theme;
use crate::ui::widgets::qr_code;

pub fn render(frame: &mut Frame, app: &App, theme: &Theme) {
    let pack = match &app.selected_pack_id {
        Some(id) => find_pack_by_id(&app.catalog, id),
        None => None,
    };
    let pack = match pack {
        Some(pack) => pack,
        None => {
            let msg = Paragraph::new("Pack not found")
                .style(theme.text_style())
                .alignment(Alignment::Center);
            frame.render_widget(msg, frame.area());
            return;
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new(format!("Checkout: {}", pack.name))
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

    render_content(frame, chunks[1], pack, theme);

    let footer_text = app
        .message
        .clone()
        .unwrap_or_else(|| "[Enter] Checkout を開く  [c] URL コピー  [b] 戻る".to_string());
    let footer = Paragraph::new(footer_text)
        .style(theme.footer_text_style())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        );
    frame.render_widget(footer, chunks[2]);
}

fn render_content(frame: &mut Frame, area: Rect, pack: &CatalogEntry, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    let mut lines = Vec::new();
    lines.push(section_title("Price", theme));
    lines.push(Line::from(Span::styled(
        format!("  YEN {}", pack.price),
        Style::default().fg(theme.accent_alt),
    )));
    lines.push(blank_line());

    lines.push(section_title("Seller", theme));
    lines.push(Line::from(Span::styled(
        "  harness-gacha",
        Style::default().fg(theme.text_primary),
    )));
    lines.push(blank_line());

    lines.push(section_title("Policy", theme));
    lines.push(Line::from(Span::styled(
        "  デジタル商品のため購入後の返金は条件付き",
        Style::default().fg(theme.text_secondary),
    )));
    lines.push(Line::from(Span::styled(
        "  利用条件は Web の規約を参照",
        Style::default().fg(theme.text_secondary),
    )));
    lines.push(blank_line());

    lines.push(section_title("Checkout URL", theme));
    lines.push(Line::from(Span::styled(
        format!("  {}", pack.checkout_url),
        Style::default().fg(theme.owned),
    )));

    let left = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        )
        .wrap(Wrap { trim: false })
        .style(theme.text_style());
    frame.render_widget(left, chunks[0]);

    let mut qr_lines = vec![section_title("QR Code", theme), blank_line()];
    qr_lines.extend(qr_code::generate_qr_lines(
        &pack.checkout_url,
        theme.text_primary,
        theme.panel_bg,
    ));

    let right = Paragraph::new(qr_lines)
        .block(
            Block::default()
                .borders(Borders::RIGHT)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        )
        .style(theme.panel_style());
    frame.render_widget(right, chunks[1]);
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
