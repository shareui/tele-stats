// MIT License
// Copyright (c) 2025 shareui

mod commands; // cmd handling mod
mod logs; // log mod
mod gitlab; // gl service mod

use commands::{Command, handle_command}; // import cmd types and handler
use serde::Deserialize; // yaml
use std::fs; // file ops
use std::sync::Arc; // th-sf reference cou-ing
use teloxide::prelude::*; // telegram bot essentials
use tokio::time::{sleep, Duration}; // async sleep

#[derive(Debug, Deserialize, Clone)]
struct GlobalConfig { // main cfg
    bot_token: String,
    update_time: u64,
    channel_id: i64,
    tg_username: String,
    services: Services,
}

#[derive(Debug, Deserialize, Clone)]
struct Services { // on/off services
    gitlab: bool,
    github: bool,
    tiktok: bool,
    faceit: bool,
    dota: bool,
}

#[tokio::main]
async fn main() {
    logs::init(); // initialize logging

    let config = load_config().expect("failed to load global config"); // load cfg
    let bot = Bot::new(&config.bot_token); // create bot insts

    logs::bot_started(); // log bot start

    let bot_clone = bot.clone();
    let config_clone = Arc::new(config.clone());
    
    tokio::spawn(async move {
        update_loop(bot_clone, config_clone).await; // run upd loop
    });

    let handler = Update::filter_message()
        .filter_command::<Command>() // filter bot cmds
        .endpoint(move |bot: Bot, msg: Message, cmd: Command| {
            let username = config.tg_username.clone();
            async move {
                handle_command(bot, msg, cmd, username).await // handle command
            }
        });

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler() // graceful shutdown
        .build()
        .dispatch()
        .await;
}

fn load_config() -> Result<GlobalConfig, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string("src/configs/global_cfg.yml")?; // read yml
    let config: GlobalConfig = serde_yaml::from_str(&config_content)?; // parse yml
    Ok(config)
}

async fn update_loop(bot: Bot, config: Arc<GlobalConfig>) {
    run_updates(&bot, &config).await; // init update

    loop {
        logs::next_update(config.update_time); // log next update
        sleep(Duration::from_secs(config.update_time * 3600)).await; // wait
        run_updates(&bot, &config).await; // run upd again
    }
}

async fn run_updates(bot: &Bot, config: &GlobalConfig) {
    if config.services.gitlab {
        logs::update_started("gitlab"); // log gitlab start
        match gitlab::run_gitlab_service(config.channel_id, bot.clone()).await {
            Ok(_) => logs::update_completed("gitlab"), // log success
            Err(e) => logs::update_failed("gitlab", &e.to_string()), // log error
        }
    }

    if config.services.github {
        logs::update_started("github"); // github not ready
        logs::update_failed("github", "not implemented yet");
    }

    if config.services.tiktok {
        logs::update_started("tiktok"); // tiktok not ready
        logs::update_failed("tiktok", "not implemented yet");
    }

    if config.services.faceit {
        logs::update_started("faceit"); // faceit not ready
        logs::update_failed("faceit", "not implemented yet");
    }

    if config.services.dota {
        logs::update_started("dota"); // dota not ready
        logs::update_failed("dota", "not implemented yet");
    }
}