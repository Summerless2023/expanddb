pub trait FileHandler {
    fn create_file(&self, _filename: String);
    fn drop_file(&self, _filename: String);
    fn read_file(&self, _filename: String);
    fn write_file(&self, _filename: String, _is_create: bool);
}

#[derive(Debug)]
pub struct CommonFileHandler {}

impl FileHandler for CommonFileHandler {
    fn create_file(&self, _filename: String) {
        println!("create file");
    }
    fn drop_file(&self, _filename: String) {
        println!("drop file");
    }
    fn read_file(&self, _filename: String) {
        println!("read file");
    }
    fn write_file(&self, _filename: String, _is_create: bool) {
        println!("write file");
    }
}
