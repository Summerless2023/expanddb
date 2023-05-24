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
        let mut file: File = self
            .file_handler
            .create_file(self.data_file_path.to_string());
        file.write_all(key.data.to_string().as_bytes()).unwrap();
        file.write_all(value.data.to_string().as_bytes()).unwrap();
    }
}
