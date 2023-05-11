use ini::Ini;

pub fn get_config() {
    let conf = Ini::load_from_file("config.ini").unwrap();
    for (sec, prop) in &conf {
        println!("Section: {:?}", sec);
        for (key, value) in prop.iter() {
            println!("{:?}:{:?}", key, value);
        }
    }
}
