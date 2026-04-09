use crate::action::Action;
use crate::screen::Screen;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub fn poll_action(timeout: Duration, _current_screen: Screen, search_active: bool) -> Action {
    if event::poll(timeout).unwrap_or(false) {
        if let Ok(Event::Key(key)) = event::read() {
            return key_to_action(key, search_active);
        }
    }
    Action::Tick
}

fn key_to_action(key: KeyEvent, search_active: bool) -> Action {
    if search_active {
        return match key.code {
            KeyCode::Esc => Action::Search,
            KeyCode::Backspace => Action::SearchBackspace,
            KeyCode::Char(c) => Action::SearchInput(c),
            KeyCode::Enter => Action::Search,
            _ => Action::None,
        };
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Action::Quit;
    }

    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('j') | KeyCode::Down => Action::Down,
        KeyCode::Char('k') | KeyCode::Up => Action::Up,
        KeyCode::Char('h') | KeyCode::Left => Action::Left,
        KeyCode::Char('l') | KeyCode::Right => Action::Right,
        KeyCode::Tab => Action::Tab,
        KeyCode::BackTab => Action::BackTab,
        KeyCode::Enter => Action::Enter,
        KeyCode::Char('b') | KeyCode::Esc => Action::Back,
        KeyCode::Char('/') => Action::Search,
        KeyCode::Char('t') => Action::ToggleTag,
        KeyCode::Char('r') => Action::ToggleDiscovery,
        KeyCode::Char('s') => Action::OpenSafety,
        KeyCode::Char('p') => Action::OpenPurchase,
        KeyCode::Char('c') => Action::CopyUrl,
        KeyCode::Char('?') => Action::OpenHelp,
        KeyCode::Char('L') => Action::OpenLibrary,
        _ => Action::None,
    }
}
