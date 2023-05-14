pub mod config;
pub mod file;
pub use config::config::*;
pub use file::handler;
use log::warn;

#[macro_use]
extern crate lazy_static;

fn main() {
    init_config_map();
    print!("{:?}", *MAP);
    warn!("-----");
}
