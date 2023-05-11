pub mod config;
pub mod file;
pub use config::config::*;
pub use file::handler;
fn main() {
    get_config();
}
