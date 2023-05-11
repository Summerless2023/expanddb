pub mod file;
pub use file::handler;
use ini::Ini;
fn main() {
    let conf = Ini::load_from_file("config.ini").unwrap();
    for (sec, prop) in &conf {
        println!("Section: {:?}", sec);
        for (key, value) in prop.iter() {
            println!("{:?}:{:?}", key, value);
        }
    }
}
