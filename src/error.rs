use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("データの読み込みに失敗しました: {0}")]
    DataLoad(String),
    #[error("JSON の解析に失敗しました: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("入出力エラーが発生しました: {0}")]
    Io(#[from] std::io::Error),
    #[error("ブラウザを開けませんでした: {0}")]
    BrowserOpen(String),
    #[error("クリップボードへのコピーに失敗しました: {0}")]
    Clipboard(String),
}
