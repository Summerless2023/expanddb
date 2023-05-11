pub mod config;
pub mod file;
pub use config::config::*;
pub use file::handler;

#[macro_use]
extern crate lazy_static;

fn main() {
    get_config();
    print!("{:?}", *MAP);
}
