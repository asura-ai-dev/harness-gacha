use std::io;
use std::time::Duration;
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::prelude::*;

mod app;
mod screen;
mod action;
mod event;
mod discovery;
mod clipboard;
mod browser;
mod error;
mod models;
mod data;
mod ui;

fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        original_hook(panic_info);
    }));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    install_panic_hook();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = run_app(&mut terminal);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = std::path::Path::new("data");
    let mut app = app::App::new(data_dir);
    let tick_rate = Duration::from_millis(250);
    while app.running {
        terminal.draw(|frame| ui::render::render(frame, &app))?;
        let action = event::poll_action(tick_rate, app.current_screen, app.search_active);
        app.update(action);
    }
    Ok(())
}
