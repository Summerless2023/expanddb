pub mod config;
pub mod file;
pub mod kv;
pub mod relation;

use config::config::*;
use file::CommonFileHandler;
use kv::{Key, Value};
use log4rs;
use relation::Relation;

#[macro_use]
extern crate lazy_static;

fn main() {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    init_config_map();

    let key: Key = Key { data: 1 };
    let value: Value = Value { data: 2 };
    let relation: Relation = Relation {
        table_space_id: 1,
        data_file_path: "t1".to_string(),
        file_handler: CommonFileHandler {},
    };
    relation.insert(key, value);

    // let mut file_name = "src/data/".to_string();
    // file_name += &relation.data_file_path;
    // print!("[{}]", file_name);
    // let data_file: bool = data_handler.create_file(file_name);
}
