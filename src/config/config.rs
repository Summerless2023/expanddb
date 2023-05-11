use ini::Ini;
use std::collections::HashMap;

pub fn get_config() -> HashMap<String, String> {
    let conf = Ini::load_from_file("config.ini").unwrap();
    let mut map: HashMap<String, String> = HashMap::new();
    map.clear();
    for (_sec, prop) in &conf {
        for (key, value) in prop.iter() {
            map.insert((*key).to_string(), (*value).to_string());
        }
    }
    map
}

lazy_static! {
    pub static ref MAP: HashMap<String, String> = {
        let map = get_config();
        map
    };
}
