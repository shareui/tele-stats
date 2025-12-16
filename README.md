# Tele-stats

A self-hosted bot for displaying (auto-updating) your static content in Telegram. All settings are in the config files.

## Supported stats
- [x] GitLab
- [x] GitHub
- [ ] TikTok
- [ ] Faceit
- [ ] Dota 2
- [ ] And more...

## How to use?

The bot is completely self-hosted.

### If you are using VDS/VSL host
```bash
{pkg manager} install cargo && {pkg manager} install git && git clone https://github.com/shareui/tele-stats && cd tele-stats-main
```
Then change the bot settings in the configs/ folder
```bash
cargo build --release && export RUST_LOG=info && ./target/release/tele-stats
```

### If you use a different type of hosting

idk

## Examples

Examples of statistics output

### GitLab
User statistics for shareui on GitLab  
Total code lines: 40744  
Last updated: 2025-11-30 | 17:08:47  
Total languages: 12  
Favorite language: Kotlin  
Repositories: 15  
Public repositories: 2  
Last activity: 2025-11-18 08:00:37  

Languages  
• Kotlin: 71.17%  
• Rust: 7.15%  
• CSS: 5.71%  
• Python: 4.55%  
• HTML: 2.79%  
• Go: 2.08%  
• Elixir: 1.63%  
• Ruby: 1.51%  
• C: 1.43%  
• JavaScript: 0.91%  

## License
MIT license :3
