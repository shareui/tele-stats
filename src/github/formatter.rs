// MIT License
// Copyright (c) 2025 shareui

use super::types::{GitHubConfig, GitHubStats};
use chrono::Local;

fn escape_markdown(text: &str) -> String {
    text.replace("_", "\\_")
        .replace("*", "\\*")
        .replace("[", "\\[")
        .replace("]", "\\]")
        .replace("(", "\\(")
        .replace(")", "\\)")
        .replace("~", "\\~")
        .replace("`", "\\`")
        .replace(">", "\\>")
        .replace("#", "\\#")
        .replace("+", "\\+")
        .replace("-", "\\-")
        .replace("=", "\\=")
        .replace("|", "\\|")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace(".", "\\.")
        .replace("!", "\\!")
}

pub fn format_stats(stats: &GitHubStats, config: &GitHubConfig) -> String {
    let now = Local::now();
    let date = now.format("%Y\\-%m\\-%d").to_string();
    let time = now.format("%H:%M:%S").to_string();

    let username = escape_markdown(&config.github_username);
    let total_lines = escape_markdown(&stats.total_lines.to_string());
    let total_langs = escape_markdown(&stats.total_languages.to_string());
    let fav_lang = escape_markdown(&stats.favorite_language);
    let total_repos = escape_markdown(&stats.total_repos.to_string());
    let public_repos = escape_markdown(&stats.public_repos.to_string());
    
    let activity_parsed = chrono::DateTime::parse_from_rfc3339(&stats.last_activity)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|_| stats.last_activity.clone());
    let activity = escape_markdown(&activity_parsed);

    let mut message = format!(
        "User statistics for {} on [GitHub](https://github\\.com/{})\n\
        *Total code lines:* {}\n\
        *Last updated:* {} \\| {}\n\
        *Total languages:* {}\n\
        *Favorite language:* {}\n\
        *Repositories:* {}\n\
        *Public repositories:* {}\n\
        *Last activity:* {}\n\n\
        *Languages*",
        username,
        config.github_username,
        total_lines,
        date,
        time,
        total_langs,
        fav_lang,
        total_repos,
        public_repos,
        activity
    );

    if config.quote {
        for lang_stat in &stats.language_stats {
            let percentage = format!("{:.2}", lang_stat.percentage).replace(".", "\\.");
            message.push_str(&format!("\n>• {}: {}%", lang_stat.name, percentage));
        }
    } else {
        for lang_stat in &stats.language_stats {
            let lang_name = escape_markdown(&lang_stat.name);
            let percentage = escape_markdown(&format!("{:.2}", lang_stat.percentage));
            message.push_str(&format!("\n• {}: {}%", lang_name, percentage));
        }
    }

    message
}