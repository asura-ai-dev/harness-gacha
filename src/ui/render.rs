use crate::app::App;
use crate::ui::theme::cherry_cartridge;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub fn render(frame: &mut Frame, app: &App) {
    let theme = cherry_cartridge();
    let area = frame.area();

    let bg = ratatui::widgets::Block::default().style(Style::default().bg(theme.primary_bg));
    frame.render_widget(bg, area);

    let text = format!(
        "harness-gacha - {:?}\nPacks: {}\nPress q to quit",
        app.current_screen,
        app.catalog.len()
    );
    let paragraph = Paragraph::new(text).style(theme.text_style());
    frame.render_widget(paragraph, area);
}
