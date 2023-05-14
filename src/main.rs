pub mod config;
pub mod file;
pub use config::config::*;
pub use file::handler;
use log::warn;
use log4rs;

#[macro_use]
extern crate lazy_static;

fn main() {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    init_config_map();
    print!("{:?}", *MAP);
    for i in 1..100000 {
        warn!("-----  {}", i);
    }
}
