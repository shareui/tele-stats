// MIT License  
// Copyright (c) 2025 shareui  
  
use chrono::Local;
  
pub fn init() {  
    env_logger::Builder::from_default_env()  
        .format(|buf, record| {  
            use std::io::Write;  
            writeln!(  
                buf,  
                "[{}] [{}] {}",  
                Local::now().format("%Y-%m-%d %H:%M:%S"), // cur time
                record.level(), // log lvll
                record.args() // log msg
            )  
        })  
        .init(); // init logger
}  
  
pub fn bot_started() {  
    log::info!("bot started successfully"); // log bot start
}  
  
pub fn next_update(hours: u64) {  
    log::info!("next update in {} hours", hours); // log next upd time
}  
  
pub fn update_started(service: &str) {  
    log::info!("starting {} service update", service); // log service start
}  
  
pub fn update_completed(service: &str) {  
    log::info!("{} service update completed", service); // log service +
}  
  
pub fn update_failed(service: &str, error: &str) {  
    log::error!("{} service update failed: {}", service, error); // log service err
}