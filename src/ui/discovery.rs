use ratatui::prelude::*;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, DiscoveryState};
use crate::data::catalog::find_pack_by_id;
use crate::models::CatalogEntry;
use crate::ui::theme::Theme;
use crate::ui::widgets::{capsule_machine, pack_card};

pub fn render(frame: &mut Frame, app: &App, theme: &Theme) {
    match &app.discovery_state {
        DiscoveryState::Idle => render_idle(frame, theme),
        DiscoveryState::Animating {
            frame: anim_frame, ..
        } => render_animating(frame, *anim_frame, theme),
        DiscoveryState::Result { pack_id } => {
            let pack = find_pack_by_id(&app.catalog, pack_id);
            render_result(frame, pack, theme);
        }
    }
}

fn render_idle(frame: &mut Frame, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new("Capsule Discovery")
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

    let mut lines = capsule_machine::idle_art(theme);
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "PRESS ENTER TO DISCOVER",
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "無料でおすすめ pack を 1 つ表示します",
        Style::default().fg(theme.text_secondary),
    )));

    let content = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_style(theme.border_style())
            .style(theme.panel_style()),
    );
    frame.render_widget(content, chunks[1]);

    let footer = Paragraph::new("[Enter] ランダムで探す  [b] 戻る")
        .style(theme.footer_text_style())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        );
    frame.render_widget(footer, chunks[2]);
}

fn render_animating(frame: &mut Frame, anim_frame: u8, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new("Capsule Discovery")
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

    let status_text = match anim_frame {
        0 | 1 => "turning...",
        2 | 3 => "clack... clack...",
        4 => "* opening *",
        _ => "OPENED!",
    };
    let status_color = if anim_frame >= 4 {
        theme.accent_alt
    } else {
        theme.text_secondary
    };

    let mut lines = capsule_machine::capsule_lines(anim_frame, theme);
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        status_text,
        Style::default()
            .fg(status_color)
            .add_modifier(Modifier::BOLD),
    )));

    let content = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_style(theme.border_style())
            .style(theme.panel_style()),
    );
    frame.render_widget(content, chunks[1]);

    let footer = Paragraph::new("[ ... ]")
        .alignment(Alignment::Center)
        .style(theme.footer_text_style())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        );
    frame.render_widget(footer, chunks[2]);
}

fn render_result(frame: &mut Frame, pack: Option<&CatalogEntry>, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new("Discovery Result")
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

    let content_lines = match pack {
        Some(pack) => {
            let mut lines = vec![
                Line::from(Span::styled(
                    "[ Capsule Opened ]",
                    Style::default()
                        .fg(theme.accent_alt)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
            ];
            lines.extend(pack_card::pack_card_lines(pack, theme));
            lines
        }
        None => vec![Line::from(Span::styled(
            "Pack not found",
            Style::default()
                .fg(theme.danger)
                .add_modifier(Modifier::BOLD),
        ))],
    };

    let content = Paragraph::new(content_lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        );
    frame.render_widget(content, chunks[1]);

    let footer = Paragraph::new("[Enter] 詳細を見る [r] もう一度探す [b] 戻る")
        .style(theme.footer_text_style())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style())
                .style(theme.panel_style()),
        );
    frame.render_widget(footer, chunks[2]);
}
