// MIT License
// Copyright (c) 2025 shareui

use super::client::GitHubClient;
use super::types::{GitHubConfig, GitHubStats, LanguageStats};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

pub struct GitHubAnalyzer {
    client: GitHubClient,
    config: GitHubConfig,
}

impl GitHubAnalyzer {
    pub fn new(config: GitHubConfig) -> Result<Self, Box<dyn Error>> {
        let client = GitHubClient::new(config.github_token.clone())?;
        Ok(Self { client, config })
    }

    pub async fn analyze(&self) -> Result<GitHubStats, Box<dyn Error>> {
        let repos = self.client
            .get_user_repos(&self.config.github_username, self.config.private_repo)
            .await?;

        let mut language_lines: HashMap<String, usize> = HashMap::new();
        let mut total_lines = 0;
        let mut public_repos = 0;
        let mut last_activity = String::new();

        for repo in &repos {
            if !repo.private {
                public_repos += 1;
            }

            if last_activity.is_empty() || repo.updated_at > last_activity {
                last_activity = repo.updated_at.clone();
            }

            match self.analyze_repository(&self.config.github_username, &repo.name, &repo.default_branch).await {
                Ok(repo_stats) => {
                    for (lang, lines) in repo_stats {
                        *language_lines.entry(lang).or_insert(0) += lines;
                        total_lines += lines;
                    }
                }
                Err(e) => {
                    log::warn!("failed to analyze repo {}: {}", repo.name, e);
                }
            }
        }

        let mut language_stats: Vec<LanguageStats> = language_lines
            .into_iter()
            .map(|(name, lines)| {
                let percentage = if total_lines > 0 {
                    (lines as f64 / total_lines as f64) * 100.0
                } else {
                    0.0
                };
                LanguageStats {
                    name,
                    percentage,
                }
            })
            .collect();

        language_stats.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());

        let favorite_language = language_stats
            .first()
            .map(|s| s.name.clone())
            .unwrap_or_else(|| "None".to_string());

        let total_languages = language_stats.len();

        if language_stats.len() > self.config.max_langs {
            language_stats.truncate(self.config.max_langs);
        }

        Ok(GitHubStats {
            total_lines,
            language_stats,
            total_languages,
            favorite_language,
            total_repos: repos.len(),
            public_repos,
            last_activity,
        })
    }

    async fn analyze_repository(&self, owner: &str, repo: &str, branch: &str) -> Result<HashMap<String, usize>, Box<dyn Error>> {
        let tree = self.client.get_repo_tree(owner, repo, branch).await?;
        let mut language_lines: HashMap<String, usize> = HashMap::new();

        for item in tree {
            if item.item_type == "blob" {
                if let Some(ext) = Path::new(&item.path)
                    .extension()
                    .and_then(|e| e.to_str())
                {
                    if let Some(lang_name) = self.config.languages.get(ext) {
                        log::info!("reading file: {}", item.path);
                        match self.client.get_file_content(owner, repo, &item.path, branch).await {
                            Ok(content) => {
                                let lines = content.lines().count();
                                *language_lines.entry(lang_name.clone()).or_insert(0) += lines;
                            }
                            Err(e) => {
                                log::warn!("failed to fetch file {}: {}", item.path, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(language_lines)
    }
}