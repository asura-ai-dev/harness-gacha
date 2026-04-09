use serde::{Deserialize, Serialize};

use crate::models::catalog_entry::{Author, InstallInfo, LicenseInfo, Target};
use crate::models::permissions::Permissions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub schema_version: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub author: Author,
    pub targets: Vec<Target>,
    pub contents: Vec<String>,
    pub permissions: Permissions,
    pub install: InstallInfo,
    pub license: LicenseInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_install: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}
