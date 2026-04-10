use crate::ui::theme::Theme;
use ratatui::prelude::*;
use ratatui::text::Span;

pub fn permission_span<'a>(name: &'a str, enabled: bool, theme: &Theme) -> Span<'a> {
    let (label, color) = if !enabled {
        ("disabled", theme.success)
    } else {
        match name {
            "shell" | "network" => ("ENABLED", theme.danger),
            "filesystem_write" | "git" => ("enabled", theme.warning),
            _ => ("enabled", theme.text_secondary),
        }
    };
    Span::styled(format!("{}: {}", name, label), Style::default().fg(color))
}

pub fn permission_line<'a>(name: &'a str, enabled: bool, theme: &Theme) -> Line<'a> {
    let (label, color, bold) = if !enabled {
        ("disabled", theme.success, false)
    } else {
        match name {
            "shell" | "network" => ("ENABLED", theme.danger, true),
            "filesystem_write" | "git" => ("enabled", theme.warning, false),
            _ => ("enabled", theme.text_secondary, false),
        }
    };
    let mut style = Style::default().fg(color);
    if bold {
        style = style.add_modifier(Modifier::BOLD);
    }
    Line::from(vec![
        Span::styled(
            format!("  {:<20}", name),
            Style::default().fg(theme.text_primary),
        ),
        Span::styled(label.to_string(), style),
    ])
}

pub fn permission_summary_text(
    shell: bool,
    network: bool,
    filesystem_write: bool,
    git: bool,
    theme: &Theme,
) -> Vec<Span<'static>> {
    let mut parts: Vec<Span<'static>> = Vec::new();
    let mut dangerous: Vec<&str> = Vec::new();
    let mut safe: Vec<&str> = Vec::new();

    if shell {
        dangerous.push("shell");
    } else {
        safe.push("shell");
    }
    if network {
        dangerous.push("network");
    } else {
        safe.push("network");
    }
    if filesystem_write {
        dangerous.push("write");
    }
    if git {
        dangerous.push("git");
    }

    if !dangerous.is_empty() {
        parts.push(Span::styled(
            dangerous.join(" / ") + " あり",
            Style::default().fg(theme.danger),
        ));
    }
    if !dangerous.is_empty() && !safe.is_empty() {
        parts.push(Span::raw(", "));
    }
    if !safe.is_empty() && !dangerous.is_empty() {
        parts.push(Span::styled(
            safe.join(" / ") + " なし",
            Style::default().fg(theme.success),
        ));
    }
    parts
}
