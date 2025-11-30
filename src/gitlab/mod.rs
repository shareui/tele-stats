// MIT License
// Copyright (c) 2025 shareui

pub mod types;
pub mod client;
pub mod analyzer;
pub mod formatter;

use analyzer::GitLabAnalyzer;       // anal  yzer for gl stats
use formatter::format_stats;         // formatter for out message
use types::GitLabConfig;             // cfg struct for gl
use std::error::Error;
use std::fs;

pub async fn run_gitlab_service(channel_id: i64, bot: teloxide::Bot) -> Result<(), Box<dyn Error>> {
    let config_path = "src/configs/gitlab_cfg.yml";                     // path to cfg
    let config_content = fs::read_to_string(config_path)?;              // read fg file
    let config: GitLabConfig = serde_yaml::from_str(&config_content)?;  // parse yml cfg

    log::info!("starting gitlab analysis for user: {}", config.gitlab_username);

    let analyzer = GitLabAnalyzer::new(config.clone())?;   // init anal  yzer instance
    let stats = analyzer.analyze().await?;                 // run anal  ysis

    let message = format_stats(&stats, &config);           // format result

    use teloxide::requests::Requester;                     // request traits
    use teloxide::prelude::*;
    
    if config.message_id != 0 {                            // if message id exists so, edit message
        bot.edit_message_text(
            teloxide::types::ChatId(channel_id),
            teloxide::types::MessageId(config.message_id as i32),
            message
        )
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;
    } else {                                               // else send new msg
        bot.send_message(teloxide::types::ChatId(channel_id), message)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
    }

    log::info!("gitlab stats sent successfully");          // logging success

    Ok(())
}