use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::app::App;
use crate::ui::theme::Theme;

pub fn render(frame: &mut Frame, _app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new("Help")
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

    render_content(frame, chunks[1], theme);

    let footer = Paragraph::new("[b] 戻る")
        .style(theme.footer_text_style())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style()),
        );
    frame.render_widget(footer, chunks[2]);
}

fn render_content(frame: &mut Frame, area: Rect, theme: &Theme) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::styled(
        "Key Bindings",
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    let keys = [
        ("j / Down", "下に移動"),
        ("k / Up", "上に移動"),
        ("Enter", "選択 / 決定"),
        ("b / Esc", "戻る"),
        ("Tab", "次のタブ"),
        ("/", "検索"),
        ("t", "タグ絞り込み"),
        ("r", "Discovery（ランダムで探す）"),
        ("s", "安全性詳細"),
        ("p", "購入案内"),
        ("c", "URL コピー"),
        ("L", "購入済みライブラリ"),
        ("?", "ヘルプ（この画面）"),
        ("q", "終了"),
    ];
    for (key, desc) in keys {
        lines.push(Line::from(Span::styled(
            format!("  {:<14} {}", key, desc),
            Style::default().fg(theme.text_primary),
        )));
    }
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(
        "Legal",
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  特定商取引法に基づく表示",
        Style::default()
            .fg(theme.text_primary)
            .add_modifier(Modifier::BOLD),
    )));
    for item in [
        "販売事業者: harness-gacha 運営",
        "所在地: 日本",
        "販売価格: 各商品ページに表示",
        "支払時期: 購入時即時決済",
        "引渡時期: 決済完了後即時",
        "返品・返金: デジタル商品のため原則不可。不具合がある場合は個別対応",
    ] {
        lines.push(Line::from(Span::styled(
            format!("    {}", item),
            Style::default().fg(theme.text_secondary),
        )));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  利用規約",
        Style::default()
            .fg(theme.text_primary)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "    https://example.com/terms",
        Style::default().fg(theme.owned),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  返金ポリシー",
        Style::default()
            .fg(theme.text_primary)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "    https://example.com/refund-policy",
        Style::default().fg(theme.owned),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  お問い合わせ",
        Style::default()
            .fg(theme.text_primary)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "    support@example.com",
        Style::default().fg(theme.owned),
    )));

    let block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_style(theme.border_style())
        .style(theme.panel_style());
    let p = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(p, area);
}
