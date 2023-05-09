pub mod file;
pub use file::handler;
use file::FileHandler;
fn main() {
    let h: file::CommonFileHandler = handler::CommonFileHandler {};
    h.create_file("123".to_string());
}
