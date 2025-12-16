// MIT License
// Copyright (c) 2025 shareui

mod commands;
mod logs;
mod gitlab;
mod github;

use commands::{Command, handle_command};
use serde::Deserialize;
use std::fs;
use std::sync::Arc;
use teloxide::prelude::*;
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize, Clone)]
struct GlobalConfig {
    bot_token: String,
    update_time: u64,
    channel_id: i64,
    tg_username: String,
    ignore_errors: bool,
    services: Services,
}

#[derive(Debug, Deserialize, Clone)]
struct Services {
    gitlab: bool,
    github: bool,
    tiktok: bool,
    faceit: bool,
    dota: bool,
}

#[tokio::main]
async fn main() {
    logs::init();

    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            logs::error_with_message(&format!("failed to load global config: {}", e));
            return;
        }
    };
    
    let bot = Bot::new(&config.bot_token);

    logs::bot_started();

    let bot_clone = bot.clone();
    let config_clone = Arc::new(config.clone());
    
    tokio::spawn(async move {
        update_loop(bot_clone, config_clone).await;
    });

    let handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(move |bot: Bot, msg: Message, cmd: Command| {
            let username = config.tg_username.clone();
            async move {
                handle_command(bot, msg, cmd, username).await
            }
        });

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn load_config() -> Result<GlobalConfig, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string("src/configs/global_cfg.yml")?;
    let config: GlobalConfig = serde_yaml::from_str(&config_content)?;
    Ok(config)
}

async fn update_loop(bot: Bot, config: Arc<GlobalConfig>) {
    run_updates(&bot, &config).await;

    loop {
        logs::next_update(config.update_time);
        sleep(Duration::from_secs(config.update_time * 3600)).await;
        run_updates(&bot, &config).await;
    }
}

async fn run_updates(bot: &Bot, config: &GlobalConfig) {
    if config.services.gitlab {
        logs::update_started("gitlab");
        match gitlab::run_gitlab_service(config.channel_id, bot.clone()).await {
            Ok(_) => logs::update_completed("gitlab"),
            Err(e) => {
                if config.ignore_errors {
                    log::error!("gitlab service update failed: {}", e);
                } else {
                    logs::update_failed("gitlab", &e.to_string());
                }
            }
        }
    }

    if config.services.github {
        logs::update_started("github");
        match github::run_github_service(config.channel_id, bot.clone()).await {
            Ok(_) => logs::update_completed("github"),
            Err(e) => {
                if config.ignore_errors {
                    log::error!("github service update failed: {}", e);
                } else {
                    logs::update_failed("github", &e.to_string());
                }
            }
        }
    }

    if config.services.tiktok {
        logs::update_started("tiktok");
        if config.ignore_errors {
            log::error!("tiktok service update failed: not implemented yet");
        } else {
            logs::update_failed("tiktok", "not implemented yet");
        }
    }

    if config.services.faceit {
        logs::update_started("faceit");
        if config.ignore_errors {
            log::error!("faceit service update failed: not implemented yet");
        } else {
            logs::update_failed("faceit", "not implemented yet");
        }
    }

    if config.services.dota {
        logs::update_started("dota");
        if config.ignore_errors {
            log::error!("dota service update failed: not implemented yet");
        } else {
            logs::update_failed("dota", "not implemented yet");
        }
    }
}