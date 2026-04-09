use ratatui::prelude::*;
use ratatui::text::Line;

/// URL 文字列から QR コードのテキスト表現を Line ベクタとして返す
/// qrcode クレートで QR を生成し、ブロック文字で描画する
/// エラー時は "QR generation failed" を返す
pub fn generate_qr_lines(url: &str, fg: Color, bg: Color) -> Vec<Line<'static>> {
    use qrcode::QrCode;

    let code = match QrCode::new(url.as_bytes()) {
        Ok(code) => code,
        Err(_) => {
            return vec![Line::from(Span::styled(
                "QR generation failed".to_string(),
                Style::default().fg(fg),
            ))];
        }
    };

    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    string
        .lines()
        .map(|line| {
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(fg).bg(bg),
            ))
        })
        .collect()
}
