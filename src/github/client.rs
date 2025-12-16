// MIT License
// Copyright (c) 2025 shareui

use super::types::{Repository, TreeResponse, ContentResponse};
use reqwest::{Client, header};
use std::error::Error;

pub struct GitHubClient {
    client: Client,
    base_url: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Result<Self, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("tele-stats"),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/vnd.github+json"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            base_url: "https://api.github.com".to_string(),
        })
    }

    pub async fn get_user_repos(&self, username: &str, include_private: bool) -> Result<Vec<Repository>, Box<dyn Error>> {
        let mut all_repos = Vec::new();
        let mut page = 1;
        let per_page = 100;

        loop {
            let url = format!(
                "{}/user/repos?page={}&per_page={}&affiliation=owner",
                self.base_url, page, per_page
            );

            let response = self.client.get(&url).send().await?;
            
            if !response.status().is_success() {
                return Err(format!("failed to fetch repositories: {}", response.status()).into());
            }

            let repos: Vec<Repository> = response.json().await?;
            
            if repos.is_empty() {
                break;
            }

            for repo in repos {
                if include_private || !repo.private {
                    all_repos.push(repo);
                }
            }

            page += 1;
        }

        Ok(all_repos)
    }

    pub async fn get_repo_tree(&self, owner: &str, repo: &str, branch: &str) -> Result<Vec<super::types::TreeItem>, Box<dyn Error>> {
        let url = format!(
            "{}/repos/{}/{}/git/trees/{}?recursive=1",
            self.base_url, owner, repo, branch
        );

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("failed to fetch tree: {}", response.status()).into());
        }

        let tree_response: TreeResponse = response.json().await?;
        Ok(tree_response.tree)
    }

    pub async fn get_file_content(&self, owner: &str, repo: &str, file_path: &str, branch: &str) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "{}/repos/{}/{}/contents/{}?ref={}",
            self.base_url, owner, repo, file_path, branch
        );

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("failed to fetch file: {}", response.status()).into());
        }

        let content_response: ContentResponse = response.json().await?;
        
        if content_response.encoding == "base64" {
            use base64::{Engine as _, engine::general_purpose};
            let decoded = general_purpose::STANDARD.decode(&content_response.content.replace("\n", ""))?;
            Ok(String::from_utf8_lossy(&decoded).to_string())
        } else {
            Err("unsupported encoding".into())
        }
    }
}