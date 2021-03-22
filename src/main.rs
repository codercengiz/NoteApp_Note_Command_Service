#![type_length_limit = "2297183"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

mod settings;

use crate::settings::Settings;
use std::env;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let settings = Settings::init();
    
    
}
