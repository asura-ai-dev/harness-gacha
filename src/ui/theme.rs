use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub primary_bg: Color,
    pub panel_bg: Color,
    pub border: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub accent: Color,
    pub accent_alt: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub owned: Color,
}

pub fn cherry_cartridge() -> Theme {
    Theme {
        primary_bg: Color::Rgb(0x15, 0x10, 0x18),
        panel_bg: Color::Rgb(0x24, 0x1A, 0x28),
        border: Color::Rgb(0x5B, 0x3A, 0x4A),
        text_primary: Color::Rgb(0xF7, 0xE8, 0xD8),
        text_secondary: Color::Rgb(0xD9, 0xB8, 0xA7),
        accent: Color::Rgb(0xFF, 0x7A, 0x59),
        accent_alt: Color::Rgb(0xFF, 0xD1, 0x66),
        success: Color::Rgb(0x7B, 0xD3, 0x89),
        warning: Color::Rgb(0xF4, 0xB9, 0x42),
        danger: Color::Rgb(0xE1, 0x56, 0x56),
        owned: Color::Rgb(0x6E, 0xC5, 0xE9),
    }
}

impl Theme {
    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text_primary).bg(self.primary_bg)
    }

    pub fn secondary_style(&self) -> Style {
        Style::default().fg(self.text_secondary).bg(self.primary_bg)
    }

    pub fn panel_style(&self) -> Style {
        Style::default().fg(self.text_primary).bg(self.panel_bg)
    }

    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    pub fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.primary_bg)
            .bg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    pub fn footer_key_style(&self) -> Style {
        Style::default().fg(self.accent).add_modifier(Modifier::BOLD)
    }

    pub fn footer_text_style(&self) -> Style {
        Style::default().fg(self.text_secondary)
    }
}
