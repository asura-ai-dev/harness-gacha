use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::app::App;
use crate::screen::Screen;
use crate::ui::theme::cherry_cartridge;

use super::{catalog, discovery, library, purchase, safety_detail};

pub fn render(frame: &mut Frame, app: &App) {
    let theme = cherry_cartridge();

    let bg = ratatui::widgets::Block::default().style(Style::default().bg(theme.primary_bg));
    frame.render_widget(bg, frame.area());

    match app.current_screen {
        Screen::Catalog => catalog::render(frame, app, &theme),
        Screen::Discovery => discovery::render(frame, app, &theme),
        Screen::Library => library::render(frame, app, &theme),
        Screen::PackDetail => super::pack_detail::render(frame, app, &theme),
        Screen::SafetyDetail => safety_detail::render(frame, app, &theme),
        Screen::Purchase => purchase::render(frame, app, &theme),
        Screen::Help => super::help::render(frame, app, &theme),
        other => {
            let text = format!("{:?} - Coming Soon\n\nPress b or Esc to go back", other);
            let p = Paragraph::new(text)
                .style(theme.text_style())
                .alignment(Alignment::Center);
            frame.render_widget(p, frame.area());
        }
    }
}
