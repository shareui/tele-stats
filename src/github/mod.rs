// MIT License
// Copyright (c) 2025 shareui

pub mod types;
pub mod client;
pub mod analyzer;
pub mod formatter;

use analyzer::GitHubAnalyzer;
use formatter::format_stats;
use types::GitHubConfig;
use std::error::Error;
use std::fs;

pub async fn run_github_service(channel_id: i64, bot: teloxide::Bot) -> Result<(), Box<dyn Error>> {
    let config_path = "src/configs/github_cfg.yml";
    let config_content = fs::read_to_string(config_path)?;
    let config: GitHubConfig = serde_yaml::from_str(&config_content)?;

    log::info!("starting github analysis for user: {}", config.github_username);

    let analyzer = GitHubAnalyzer::new(config.clone())?;
    let stats = analyzer.analyze().await?;

    let message = format_stats(&stats, &config);

    use teloxide::requests::Requester;
    use teloxide::prelude::*;
    
    if config.message_id != 0 {
        bot.edit_message_text(
            teloxide::types::ChatId(channel_id),
            teloxide::types::MessageId(config.message_id as i32),
            message
        )
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;
    } else {
        bot.send_message(teloxide::types::ChatId(channel_id), message)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
    }

    log::info!("github stats sent successfully");

    Ok(())
}