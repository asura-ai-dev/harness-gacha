use ratatui::prelude::*;
use ratatui::text::{Line, Span};

use crate::models::CatalogEntry;
use crate::ui::theme::Theme;

pub fn pack_card_lines(pack: &CatalogEntry, theme: &Theme) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    lines.push(Line::from(Span::styled(
        pack.name.clone(),
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    )));

    lines.push(Line::from(Span::styled(
        pack.summary.clone(),
        Style::default().fg(theme.text_primary),
    )));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(
        format!("Creator: {}", pack.author.name),
        Style::default().fg(theme.text_secondary),
    )));

    let targets_str: String = pack
        .targets
        .iter()
        .map(|t| format!("{} {}", t.tool, t.version_range))
        .collect::<Vec<_>>()
        .join(" / ");
    lines.push(Line::from(Span::styled(
        format!("Targets: {}", targets_str),
        Style::default().fg(theme.text_secondary),
    )));

    lines.push(Line::from(Span::styled(
        format!("Price: YEN {}", pack.price),
        Style::default().fg(theme.accent_alt),
    )));

    let trust = if pack.permissions.has_danger() {
        Span::styled("Risk: High".to_string(), Style::default().fg(theme.danger))
    } else {
        Span::styled("Safe: Low".to_string(), Style::default().fg(theme.success))
    };
    lines.push(Line::from(vec![
        Span::styled(
            "Trust: ".to_string(),
            Style::default().fg(theme.text_secondary),
        ),
        trust,
    ]));

    let cs = &pack.contents_summary;
    lines.push(Line::from(Span::styled(
        format!(
            "skills {}件 / hooks {}件 / templates {}件",
            cs.skills, cs.hooks, cs.templates
        ),
        Style::default().fg(theme.text_secondary),
    )));

    lines
}
