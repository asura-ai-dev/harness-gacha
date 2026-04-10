use serde::{Deserialize, Serialize};

use crate::models::permissions::Permissions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub tool: String,
    pub version_range: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentsSummary {
    #[serde(default)]
    pub skills: u32,
    #[serde(default)]
    pub hooks: u32,
    #[serde(default)]
    pub templates: u32,
    #[serde(default)]
    pub other: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallInfo {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    #[serde(rename = "type")]
    pub license_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    pub id: String,
    pub name: String,
    pub version: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub author: Author,
    pub targets: Vec<Target>,
    pub contents_summary: ContentsSummary,
    pub permissions: Permissions,
    pub install: InstallInfo,
    pub license: LicenseInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risks: Option<Vec<String>>,
    pub price: u32,
    pub status: String,
    pub featured: bool,
    pub listed_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_preview: Option<String>,
    pub checkout_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_notes: Option<String>,
}
