use std::fs::File;
use std::io::Error;
pub trait FileHandler {
    fn create_file(&self, _filename: String) -> File;
    fn drop_file(&self, _filename: String) -> Result<(), Error>;
    fn read_file(&self, _filename: String) -> Result<(), Error>;
    fn write_file(&self, _filename: String, _is_create: bool) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct CommonFileHandler {}

impl FileHandler for CommonFileHandler {
    fn create_file(&self, _filename: String) -> File {
        let file = match File::create(&_filename) {
            Err(_why) => {
                panic!("create file {} error", _filename)
            }
            Ok(file) => file,
        };
        println!("create file");
        // self.file = file,
        // true
        file
    }
    fn drop_file(&self, _filename: String) -> Result<(), Error> {
        println!("drop file");
        Ok(())
    }
    fn read_file(&self, _filename: String) -> Result<(), Error> {
        println!("read file");
        Ok(())
    }
    fn write_file(&self, _filename: String, _is_create: bool) -> Result<(), Error> {
        println!("write file");
        // file.write(b"some bytes");

        // file.write_all(b"more bytes");

        // file.flush();
        Ok(())
    }
}
