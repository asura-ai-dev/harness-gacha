use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;
use std::time::Duration;

mod action;
mod app;
mod browser;
mod clipboard;
mod data;
mod discovery;
mod event;
mod models;
mod screen;
mod ui;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let data_dir = std::path::Path::new("data");
    let mut app = app::App::new(data_dir);

    let tick_rate = Duration::from_millis(250);
    while app.running {
        terminal.draw(|frame| ui::render::render(frame, &app))?;
        let action = event::poll_action(tick_rate, app.current_screen, app.search_active);
        app.update(action);
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
