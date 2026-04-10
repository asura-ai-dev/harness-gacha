use std::fs;
use std::path::Path;

use crate::models::manifest::Manifest;

#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

pub fn load_manifest(path: &Path) -> Result<Manifest, String> {
    let content =
        fs::read_to_string(path).map_err(|error| format!("ファイル読み込みエラー: {}", error))?;

    serde_json::from_str::<Manifest>(&content)
        .map_err(|error| format!("JSON パースエラー: {}", error))
}

pub fn validate_manifest(manifest: &Manifest) -> ValidationResult {
    let mut errors = Vec::new();

    if manifest.id.trim().is_empty() {
        errors.push("id が空です".to_string());
    } else if !manifest
        .id
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
    {
        errors.push(format!(
            "id '{}' に不正な文字が含まれています（英小文字、数字、- のみ許可）",
            manifest.id
        ));
    }

    if manifest.name.trim().is_empty() {
        errors.push("name が空です".to_string());
    }

    if manifest.version.trim().is_empty() {
        errors.push("version が空です".to_string());
    }

    if manifest.summary.trim().is_empty() {
        errors.push("summary が空です".to_string());
    }

    if manifest.author.name.trim().is_empty() {
        errors.push("author.name が空です".to_string());
    }

    if manifest.targets.is_empty() {
        errors.push("targets が空です（最低 1 件必要）".to_string());
    }

    if manifest.contents.is_empty() {
        errors.push("contents が空です（最低 1 件必要）".to_string());
    }

    if !matches!(
        manifest.install.method.as_str(),
        "copy" | "script" | "manual"
    ) {
        errors.push(format!(
            "install.method '{}' は不正です（copy, script, manual のいずれか）",
            manifest.install.method
        ));
    }

    if manifest.license.license_type.trim().is_empty() {
        errors.push("license.type が空です".to_string());
    }

    ValidationResult {
        valid: errors.is_empty(),
        errors,
    }
}

pub fn validate_contents(manifest: &Manifest, pack_root: &Path) -> Vec<String> {
    let mut missing = Vec::new();

    for entry in &manifest.contents {
        let file_path = pack_root.join(entry);
        if !file_path.exists() {
            missing.push(format!("contents に記載された '{}' が存在しません", entry));
        }
    }

    missing
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::catalog_entry::{Author, InstallInfo, LicenseInfo, Target};
    use crate::models::permissions::Permissions;

    fn valid_manifest() -> Manifest {
        Manifest {
            schema_version: "1.0".to_string(),
            id: "test-pack".to_string(),
            name: "Test Pack".to_string(),
            version: "1.0.0".to_string(),
            summary: "A test pack".to_string(),
            description: None,
            author: Author {
                name: "Test".to_string(),
                url: None,
                email: None,
            },
            targets: vec![Target {
                tool: "claude-code".to_string(),
                version_range: ">=1.0.0".to_string(),
            }],
            contents: vec!["skills/test.md".to_string()],
            permissions: Permissions {
                shell: false,
                network: false,
                filesystem_read: true,
                filesystem_write: false,
                git: false,
            },
            install: InstallInfo {
                method: "copy".to_string(),
                entrypoint: None,
                steps: None,
            },
            license: LicenseInfo {
                license_type: "commercial".to_string(),
                text_url: None,
                spdx: None,
            },
            tags: None,
            risks: None,
            post_install: None,
            homepage: None,
            repository: None,
        }
    }

    #[test]
    fn test_valid_manifest() {
        let result = validate_manifest(&valid_manifest());
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_empty_id() {
        let mut manifest = valid_manifest();
        manifest.id = String::new();

        let result = validate_manifest(&manifest);

        assert!(!result.valid);
        assert!(result.errors.iter().any(|error| error.contains("id")));
    }

    #[test]
    fn test_invalid_id_chars() {
        let mut manifest = valid_manifest();
        manifest.id = "Test Pack".to_string();

        let result = validate_manifest(&manifest);

        assert!(!result.valid);
        assert!(result
            .errors
            .iter()
            .any(|error| error.contains("不正な文字")));
    }

    #[test]
    fn test_empty_targets() {
        let mut manifest = valid_manifest();
        manifest.targets = Vec::new();

        let result = validate_manifest(&manifest);

        assert!(!result.valid);
        assert!(result.errors.iter().any(|error| error.contains("targets")));
    }

    #[test]
    fn test_invalid_install_method() {
        let mut manifest = valid_manifest();
        manifest.install.method = "invalid".to_string();

        let result = validate_manifest(&manifest);

        assert!(!result.valid);
        assert!(result
            .errors
            .iter()
            .any(|error| error.contains("install.method")));
    }
}
