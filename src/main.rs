
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

mod models;
mod services;
mod settings;


use crate::settings::Settings;
use std::env;

#[tokio::main]
#[warn(unused_must_use)]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let settings = Settings::init();
    
    services::run(settings).await;
}
