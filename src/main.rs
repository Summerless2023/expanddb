pub mod config;
pub mod file;
pub use config::config::*;
pub use file::handler;
use file::{CommonFileHandler, FileHandler};
use log4rs;

#[macro_use]
extern crate lazy_static;

fn main() {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    init_config_map();
    let data_handler: CommonFileHandler = CommonFileHandler {};
    let result = data_handler.create_file("./src/data/1.txt".to_string());
}
