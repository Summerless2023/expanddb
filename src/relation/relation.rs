use std::{fs::File, io::Write};

use crate::{
    file::{CommonFileHandler, FileHandler},
    kv::{Key, Value},
};

pub struct Relation {
    pub file_handler: CommonFileHandler,
    pub table_space_id: u32,
    pub data_file_path: String,
}

impl Relation {
    pub fn insert(&self, key: Key, value: Value) {
        let file: Result<File, std::io::Error> =
            self.file_handler.open_file(self.data_file_path.to_string());
        match file {
            Ok(mut _file) => {
                let str = format!("{} {}\n", key.data, value.data);
                let _result = _file.write(str.as_bytes());
            }
            Err(e) => {
                panic!("open file error, error reason {}", e);
            }
        }
        // let mut file: File = self
        //     .file_handler
        //     .create_file(self.data_file_path.to_string());
    }
}
