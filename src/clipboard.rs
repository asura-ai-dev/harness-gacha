/// テキストをシステムクリップボードにコピーする
/// 成功時は Ok(()), 失敗時はエラーメッセージを返す
pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    use arboard::Clipboard;

    let mut clipboard =
        Clipboard::new().map_err(|e| format!("クリップボードの初期化に失敗しました: {}", e))?;
    clipboard
        .set_text(text.to_string())
        .map_err(|e| format!("クリップボードへのコピーに失敗しました: {}", e))
}
