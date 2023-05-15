use std::fs::File;
use std::io::{Error, Write};
pub trait FileHandler {
    fn create_file(&self, _filename: String) -> Result<(), Error>;
    fn drop_file(&self, _filename: String) -> Result<(), Error>;
    fn read_file(&self, _filename: String) -> Result<(), Error>;
    fn write_file(&self, _filename: String, _is_create: bool) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct CommonFileHandler {}

impl FileHandler for CommonFileHandler {
    fn create_file(&self, _filename: String) -> Result<(), Error> {
        let mut buffer = File::create(_filename)?;

        buffer.write(b"some bytes")?;

        buffer.write_all(b"more bytes")?;

        buffer.flush()?;
        println!("create file");
        Ok(())
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
        Ok(())
    }
}
