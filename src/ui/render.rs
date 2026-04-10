use ratatui::prelude::*;

use crate::app::App;
use crate::screen::Screen;
use crate::ui::theme::cherry_cartridge;

use super::{catalog, discovery, install_detail, library, purchase, safety_detail};

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
        Screen::InstallDetail => install_detail::render(frame, app, &theme),
        Screen::Help => super::help::render(frame, app, &theme),
    }
}
