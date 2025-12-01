// MIT License
// Copyright (c) 2025 shareui

use super::types::{Repository, TreeItem}; // repo and tree structs
use reqwest::{Client, header}; // http client
use std::error::Error; // err trait

pub struct GitLabClient {
    client: Client, // http client inst
    base_url: String, // gl api base url
}

impl GitLabClient {
    pub fn new(token: String) -> Result<Self, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new(); // req headers
        headers.insert(
            "PRIVATE-TOKEN",
            header::HeaderValue::from_str(&token)?, // auth token header
        );

        let client = Client::builder()
            .default_headers(headers) // attach auth headers
            .build()?; // build client

        Ok(Self {
            client,
            base_url: "https://gitlab.com/api/v4".to_string(), // gl api root
        })
    }

    pub async fn get_user_repos(&self, username: &str, include_private: bool) -> Result<Vec<Repository>, Box<dyn Error>> {
        let mut all_repos = Vec::new(); // collected repos
        let mut page = 1; // pagination index
        let per_page = 100; // items per page

        loop {
            let url = format!(
                "{}/users/{}/projects?page={}&per_page={}&simple=true",
                self.base_url, username, page, per_page // repos fetch url
            );

            let response = self.client.get(&url).send().await?; // exec request
            
            if !response.status().is_success() {
                return Err(format!("failed to fetch repositories: {}", response.status()).into()); // request failed
            }

            let repos: Vec<Repository> = response.json().await?; // decode json
            
            if repos.is_empty() {
                break; // stop if no more repos
            }

            for repo in repos {
                if include_private || repo.visibility == "public" { // private filter
                    all_repos.push(repo);
                }
            }

            page += 1; // next page
        }

        Ok(all_repos)
    }

    pub async fn get_repo_tree(&self, project_id: u64, branch: &str) -> Result<Vec<TreeItem>, Box<dyn Error>> {
        let mut all_items = Vec::new(); // store all tree items
        let mut page = 1;
        let per_page = 100;

        loop {
            let url = format!(
                "{}/projects/{}/repository/tree?ref={}&recursive=true&page={}&per_page={}",
                self.base_url, project_id, branch, page, per_page // repo tree url
            );

            let response = self.client.get(&url).send().await?;
            
            if !response.status().is_success() {
                return Err(format!("failed to fetch tree: {}", response.status()).into()); // request failure
            }

            let items: Vec<TreeItem> = response.json().await?;
            
            if items.is_empty() {
                break; // exit pagin..
            }

            all_items.extend(items); // add page results
            page += 1;
        }

        Ok(all_items)
    }

    pub async fn get_file_content(&self, project_id: u64, file_path: &str, branch: &str) -> Result<String, Box<dyn Error>> {
        let encoded_path = urlencoding::encode(file_path); // url-safe path
        let url = format!(
            "{}/projects/{}/repository/files/{}?ref={}",
            self.base_url, project_id, encoded_path, branch // file content url
        );

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("failed to fetch file: {}", response.status()).into()); // file request fail
        }

        let json: serde_json::Value = response.json().await?; // decode json
        
        if let Some(content) = json.get("content").and_then(|v| v.as_str()) { // base64 field
            use base64::{Engine as _, engine::general_purpose};
            let decoded = general_purpose::STANDARD.decode(content)?; // decode base64
            Ok(String::from_utf8_lossy(&decoded).to_string()) // convert to utf8 string
        } else {
            Err("no content in response".into()) // missing file data
        }
    }
}