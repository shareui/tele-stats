// MIT License
// Copyright (c) 2025 shareui

use super::client::GitLabClient; // gl api client
use super::types::{GitLabConfig, GitLabStats, LanguageStats}; // stats and cfg structs
use std::collections::HashMap; // map for line counting
use std::error::Error; // err h-ng
use std::path::Path; // path ext extracting

pub struct GitLabAnalyzer {
    client: GitLabClient, // api client instance
    config: GitLabConfig, // analyzer cfg
}

impl GitLabAnalyzer {
    pub fn new(config: GitLabConfig) -> Result<Self, Box<dyn Error>> {
        let client = GitLabClient::new(config.gitlab_token.clone())?; // auth client
        Ok(Self { client, config })
    }

    pub async fn analyze(&self) -> Result<GitLabStats, Box<dyn Error>> {
        let repos = self.client
            .get_user_repos(&self.config.gitlab_username, self.config.private_repo)
            .await?; // fetch user repos

        let mut language_lines: HashMap<String, usize> = HashMap::new(); // map lang->lines
        let mut total_lines = 0; // total code lines
        let mut public_repos = 0; // count pub repos
        let mut last_activity = String::new(); // last activ-y time-p

        for repo in &repos {
            if repo.visibility == "public" {
                public_repos += 1; // count pub repo
            }

            if last_activity.is_empty() || repo.last_activity_at > last_activity {
                last_activity = repo.last_activity_at.clone(); // upd last activity
            }

            if let Some(branch) = &repo.default_branch {
                match self.analyze_repository(repo.id, branch).await {
                    Ok(repo_stats) => {
                        for (lang, lines) in repo_stats {
                            *language_lines.entry(lang).or_insert(0) += lines; // accumulate per language
                            total_lines += lines; // accumulate total lines
                        }
                    }
                    Err(e) => {
                        log::warn!("failed to analyze repo {}: {}", repo.name, e); // repo fetch fail
                    }
                }
            }
        }

        let mut language_stats: Vec<LanguageStats> = language_lines
            .into_iter()
            .map(|(name, lines)| {
                let percentage = if total_lines > 0 {
                    (lines as f64 / total_lines as f64) * 100.0 // compute percentage
                } else {
                    0.0
                };
                LanguageStats {
                    name,
                    percentage,
                }
            })
            .collect();

        language_stats.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap()); // sort desc

        let favorite_language = language_stats
            .first()
            .map(|s| s.name.clone())
            .unwrap_or_else(|| "None".to_string()); // top language or None

        let total_languages = language_stats.len(); // languages count

        if language_stats.len() > self.config.max_langs {
            language_stats.truncate(self.config.max_langs); // trim if too many
        }

        Ok(GitLabStats {
            total_lines,
            language_stats,
            total_languages,
            favorite_language,
            total_repos: repos.len(), // total repos
            public_repos,
            last_activity,
        })
    }

    async fn analyze_repository(&self, project_id: u64, branch: &str) -> Result<HashMap<String, usize>, Box<dyn Error>> {
        let tree = self.client.get_repo_tree(project_id, branch).await?; // get file tree
        let mut language_lines: HashMap<String, usize> = HashMap::new();

        for item in tree {
            if item.item_type == "blob" { // skip non-files
                if let Some(ext) = Path::new(&item.path)
                    .extension()
                    .and_then(|e| e.to_str())
                {
                    if let Some(lang_name) = self.config.languages.get(ext) { // match ext to lang
                        log::info!("reading file: {}", item.path);
                        match self.client.get_file_content(project_id, &item.path, branch).await {
                            Ok(content) => {
                                let lines = content.lines().count(); // count file lines
                                *language_lines.entry(lang_name.clone()).or_insert(0) += lines; // accumulate
                            }
                            Err(e) => {
                                log::warn!("failed to fetch file {}: {}", item.path, e); // file read fail
                            }
                        }
                    }
                }
            }
        }

        Ok(language_lines)
    }
}