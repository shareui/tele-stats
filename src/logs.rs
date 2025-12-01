// MIT License  
// Copyright (c) 2025 shareui  
  
use chrono::Local;
use std::io::{self, Write};
use std::process;
// the comments were eaten by AI when I asked for something to be corrected
pub fn init() {  
    env_logger::Builder::from_default_env()  
        .format(|buf, record| {  
            use std::io::Write;
            use log::Level;
            
            let level_string = match record.level() {
                Level::Error => format!("\x1b[31m{}\x1b[0m", record.level()),
                _ => format!("{}", record.level()),
            };
            
            writeln!(  
                buf,  
                "[{}] [{}] {}",  
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_string,
                record.args()
            )  
        })  
        .init();
}  
  
pub fn bot_started() {  
    log::info!("telestats started. version: 0.1.1");
}  
  
pub fn next_update(hours: u64) {  
    log::info!("next update in {} hours", hours);
}  
  
pub fn update_started(service: &str) {  
    log::info!("starting {} service update", service);
}  
  
pub fn update_completed(service: &str) {  
    log::info!("{} service update completed", service);
}  
  
pub fn update_failed(service: &str, error: &str) {  
    log::error!("{} service update failed: {}", service, error);
    handle_error();
}

pub fn error_with_message(message: &str) {
    log::error!("{}", message);
    handle_error();
}

fn handle_error() {
    println!("\nAn unexpected error occurred, please report it to issues: https://github.com/shareui/tele-stats/issues");
    
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    print!("Continue runtime? [y/N]: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let response = input.trim().to_lowercase();
    if response != "y" && response != "yes" {
        log::info!("shutting down bot by user request");
        process::exit(0);
    }
}