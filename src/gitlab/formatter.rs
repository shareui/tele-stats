// MIT License
// Copyright (c) 2025 shareui

use super::types::{GitLabConfig, GitLabStats};
use chrono::Local;

fn escape_markdown(text: &str) -> String {
    // esc markdown symbols for tg
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

pub fn format_stats(stats: &GitLabStats, config: &GitLabConfig) -> String {
    let now = Local::now(); // curr time
    let date = now.format("%Y\\-%m\\-%d").to_string(); // formatted date
    let time = now.format("%H:%M:%S").to_string(); // formatted time

    let username = escape_markdown(&config.gitlab_username); // safe user
    let total_lines = escape_markdown(&stats.total_lines.to_string()); // total code lines
    let total_langs = escape_markdown(&stats.total_languages.to_string()); // langcount
    let fav_lang = escape_markdown(&stats.favorite_language); // top lang
    let total_repos = escape_markdown(&stats.total_repos.to_string()); // total repos
    let public_repos = escape_markdown(&stats.public_repos.to_string()); // public repos
    
    let activity_parsed = chrono::DateTime::parse_from_rfc3339(&stats.last_activity)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|_| stats.last_activity.clone()); // format last activity timestamp
    let activity = escape_markdown(&activity_parsed);
// sorry about that
    let mut message = format!(
        "User statistics for {} on [GitLab](https://gitlab\\.com/{})\n\
        *Total code lines:* {}\n\
        *Last updated:* {} \\| {}\n\
        *Total languages:* {}\n\
        *Favorite language:* {}\n\
        *Repositories:* {}\n\
        *Public repositories:* {}\n\
        *Last activity:* {}\n\n\
        *Languages*\n",
        username,
        config.gitlab_username,
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
        message.push_str(">\n"); // quote section
        for lang_stat in &stats.language_stats {
            let percentage = format!("{:.2}", lang_stat.percentage).replace(".", "\\."); // format percent
            message.push_str(&format!(">• {}: {}%\n", lang_stat.name, percentage)); // markdown out
        }
    } else {
        for lang_stat in &stats.language_stats {
            let lang_name = escape_markdown(&lang_stat.name); // safe lang name
            let percentage = escape_markdown(&format!("{:.2}", lang_stat.percentage)); // safe perc
            message.push_str(&format!("• {}: {}%\n", lang_name, percentage)); // normal out
        }
    }

    message // return formatted msg
}