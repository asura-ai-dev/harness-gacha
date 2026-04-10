/// URL をデフォルトブラウザで開く
/// 成功時は Ok(()), 失敗時はエラーメッセージを返す
pub fn open_url(url: &str) -> Result<(), String> {
    open::that(url).map_err(|e| format!("ブラウザの起動に失敗しました: {}", e))
}
