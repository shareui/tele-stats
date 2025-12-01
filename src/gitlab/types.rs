// MIT License
// Copyright (c) 2025 shareui

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// config parsing, nothing interesting
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitLabConfig {
    pub gitlab_token: String,
    pub message_id: i64,
    pub private_repo: bool,
    pub quote: bool,
    pub max_langs: usize,
    pub gitlab_username: String,
    pub languages: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub default_branch: Option<String>,
    pub visibility: String,
    pub last_activity_at: String,
}

#[derive(Debug, Deserialize)]
pub struct TreeItem {
    pub path: String,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Debug, Clone)]
pub struct LanguageStats {
    pub name: String,
    pub percentage: f64,
}

#[derive(Debug)]
pub struct GitLabStats {
    pub total_lines: usize,
    pub language_stats: Vec<LanguageStats>,
    pub total_languages: usize,
    pub favorite_language: String,
    pub total_repos: usize,
    pub public_repos: usize,
    pub last_activity: String,
}