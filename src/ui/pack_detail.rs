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
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_header(frame, chunks[0], pack, theme);
    render_content(frame, chunks[1], app, pack, theme);
    render_footer(frame, chunks[2], theme);
}

fn render_not_found(frame: &mut Frame, theme: &Theme) {
    let paragraph = Paragraph::new("Pack not found")
        .style(theme.text_style())
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, frame.area());
}

fn render_header(frame: &mut Frame, area: Rect, pack: &CatalogEntry, theme: &Theme) {
    let lines = vec![
        Line::from(vec![
            Span::styled(
                pack.name.as_str(),
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                format!("YEN {}", pack.price),
                Style::default().fg(theme.accent_alt),
            ),
        ]),
        Line::from(vec![
            Span::styled("Creator: ", Style::default().fg(theme.text_secondary)),
            Span::styled(pack.author.name.as_str(), Style::default().fg(theme.text_primary)),
            Span::raw("  "),
            Span::styled("Updated: ", Style::default().fg(theme.text_secondary)),
            Span::styled(pack.updated_at.as_str(), Style::default().fg(theme.text_primary)),
        ]),
    ];

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style())
            .style(theme.panel_style()),
    );
    frame.render_widget(paragraph, area);
}

fn render_content(frame: &mut Frame, area: Rect, app: &App, pack: &CatalogEntry, theme: &Theme) {
    let description = pack.description.as_deref().unwrap_or("No description");
    let target_lines = if pack.targets.is_empty() {
        vec![Line::from("None")]
    } else {
        pack.targets
            .iter()
            .map(|target| Line::from(format!("- {} {}", target.tool, target.version_range)))
            .collect::<Vec<_>>()
    };
    let included_summary_lines = vec![
        Line::from(format!("skills: {}", pack.contents_summary.skills)),
        Line::from(format!("hooks: {}", pack.contents_summary.hooks)),
        Line::from(format!("templates: {}", pack.contents_summary.templates)),
        Line::from(format!("other: {}", pack.contents_summary.other)),
    ];
    let permission_lines = vec![
        permission_summary_line("shell", pack.permissions.shell, theme),
        permission_summary_line("network", pack.permissions.network, theme),
        permission_summary_line("filesystem_read", pack.permissions.filesystem_read, theme),
        permission_summary_line(
            "filesystem_write",
            pack.permissions.filesystem_write,
            theme,
        ),
        permission_summary_line("git", pack.permissions.git, theme),
    ];

    let mut lines = vec![
        section_title("Summary", theme),
        Line::from(pack.summary.as_str()),
        blank_line(),
        Line::from(description),
        blank_line(),
        section_title("Targets", theme),
    ];
    lines.extend(target_lines);
    lines.push(blank_line());
    lines.push(section_title("Included Summary", theme));
    lines.extend(included_summary_lines);
    lines.push(blank_line());
    lines.push(section_title("Permission Summary", theme));
    lines.extend(permission_lines);
    lines.push(blank_line());
    lines.push(section_title("Sample Preview", theme));
    lines.push(sample_preview_line(pack.sample_preview.as_deref(), theme));

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.scroll_offset, 0))
        .style(theme.text_style());
    frame.render_widget(paragraph, area);
}

fn render_footer(frame: &mut Frame, area: Rect, theme: &Theme) {
    let paragraph = Paragraph::new(
        "[s] 安全性詳細  [p] 購入案内  [b] 戻る  [j/k] スクロール",
    )
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

fn permission_summary_line<'a>(label: &'a str, enabled: bool, theme: &Theme) -> Line<'a> {
    let (status, color) = if !enabled {
        ("no", theme.success)
    } else {
        match label {
            "shell" | "network" => ("YES", theme.danger),
            "filesystem_write" | "git" => ("yes", theme.warning),
            "filesystem_read" => ("yes", theme.text_secondary),
            _ => ("yes", theme.text_secondary),
        }
    };

    Line::from(vec![
        Span::styled(
            format!("{label}: "),
            Style::default().fg(theme.text_primary),
        ),
        Span::styled(status, Style::default().fg(color)),
    ])
}

fn sample_preview_line(preview: Option<&str>, theme: &Theme) -> Line<'static> {
    match preview {
        Some(text) => Line::from(Span::styled(
            text.to_string(),
            Style::default()
                .fg(theme.text_secondary)
                .add_modifier(Modifier::ITALIC),
        )),
        None => Line::from(Span::styled(
            "None".to_string(),
            Style::default().fg(theme.text_secondary),
        )),
    }
}
