// MIT License
// Copyright (c) 2025 shareui

use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::requests::Requester;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "show bot information")]
    Start,
}

pub async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    tg_username: String, // import from cfg
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match cmd {
        Command::Start => {
            let message = format!(
                "Tele\\-stats this is a self\\-hosted bot, written in Rust using the [teloxide](https://github.com/teloxide/teloxide) framework\\.\n\
                Author: @shareui\n\
                Version: 0\\.1\\.0\n\
                Hosted by: @{}\n\
                Repository: [GitHub](https://github.com/shareui/tele\\-stats)",
                tg_username.replace("-", "\\-").replace(".", "\\.") // markdown(((((
            );

            bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .await?;
        }
    }

    Ok(())
}