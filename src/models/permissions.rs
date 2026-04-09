use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Permissions {
    pub shell: bool,
    pub network: bool,
    pub filesystem_read: bool,
    pub filesystem_write: bool,
    pub git: bool,
}

impl Permissions {
    /// danger レベルの権限が有効かどうか
    /// shell と network を danger とみなす
    pub fn has_danger(&self) -> bool {
        self.shell || self.network
    }

    /// 有効な権限の一覧を返す
    pub fn enabled_list(&self) -> Vec<&str> {
        let mut list = Vec::new();
        if self.shell { list.push("shell"); }
        if self.network { list.push("network"); }
        if self.filesystem_read { list.push("filesystem_read"); }
        if self.filesystem_write { list.push("filesystem_write"); }
        if self.git { list.push("git"); }
        list
    }
}
