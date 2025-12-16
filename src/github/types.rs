// MIT License
// Copyright (c) 2025 shareui

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitHubConfig {
    pub github_token: String,
    pub message_id: i64,
    pub private_repo: bool,
    pub quote: bool,
    pub max_langs: usize,
    pub github_username: String,
    pub languages: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub default_branch: String,
    pub private: bool,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct TreeItem {
    pub path: String,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Debug, Deserialize)]
pub struct TreeResponse {
    pub tree: Vec<TreeItem>,
}

#[derive(Debug, Deserialize)]
pub struct ContentResponse {
    pub content: String,
    pub encoding: String,
}

#[derive(Debug, Clone)]
pub struct LanguageStats {
    pub name: String,
    pub percentage: f64,
}

#[derive(Debug)]
pub struct GitHubStats {
    pub total_lines: usize,
    pub language_stats: Vec<LanguageStats>,
    pub total_languages: usize,
    pub favorite_language: String,
    pub total_repos: usize,
    pub public_repos: usize,
    pub last_activity: String,
}