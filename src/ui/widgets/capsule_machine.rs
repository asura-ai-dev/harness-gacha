use ratatui::prelude::*;
use ratatui::text::{Line, Span};

use crate::ui::theme::Theme;

pub fn capsule_art(frame: u8) -> Vec<&'static str> {
    match frame {
        0 => vec![
            "┌──────────────────┐",
            "│   CAPSULE GAME   │",
            "│                  │",
            "│      ○  ○  ○     │",
            "│    ○  ○  ○  ○    │",
            "│        ▒▒        │",
            "│       ─┼─        │",
            "└──────────────────┘",
        ],
        1 => vec![
            "┌──────────────────┐",
            "│   CAPSULE GAME   │",
            "│                  │",
            "│      ○  ○  ○     │",
            "│    ○  ○  ○  ○    │",
            "│        ▒▒        │",
            "│       \\│/        │",
            "└──────────────────┘",
        ],
        2 => vec![
            "┌──────────────────┐",
            "│   CAPSULE GAME   │",
            "│                  │",
            "│      ○  ○  ○     │",
            "│    ○  ○     ○    │",
            "│        ▒▒        │",
            "│       ─┼─    ○   │",
            "└──────────────────┘",
        ],
        3 => vec![
            "┌──────────────────┐",
            "│   CAPSULE GAME   │",
            "│                  │",
            "│      ○  ○  ○     │",
            "│    ○  ○     ○    │",
            "│                  │",
            "│       ─┼─        │",
            "└────────○─────────┘",
        ],
        4 => vec![
            "┌──────────────────┐",
            "│   CAPSULE GAME   │",
            "│                  │",
            "│      ○  ○  ○     │",
            "│    ○  ○     ○    │",
            "│                  │",
            "│       ─┼─        │",
            "└──────────────────┘",
            "",
            "        * ○ *        ",
        ],
        _ => vec![
            "┌──────────────────┐",
            "│   CAPSULE GAME   │",
            "│                  │",
            "│      ○  ○  ○     │",
            "│    ○  ○     ○    │",
            "│                  │",
            "│       ─┼─        │",
            "└──────────────────┘",
            "",
            "     [ OPENED! ]     ",
        ],
    }
}

pub fn capsule_lines(frame: u8, theme: &Theme) -> Vec<Line<'static>> {
    let art = capsule_art(frame);
    let color = match frame {
        4..=u8::MAX => theme.accent_alt,
        _ => theme.border,
    };

    art.into_iter()
        .map(|line| Line::from(Span::styled(line.to_string(), Style::default().fg(color))))
        .collect()
}

pub fn idle_art(theme: &Theme) -> Vec<Line<'static>> {
    capsule_lines(0, theme)
}

pub const TOTAL_FRAMES: u8 = 6;
