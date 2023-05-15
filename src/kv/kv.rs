pub struct Key {
    pub data: u32,
}

impl Key {
    pub fn get(&self) -> u32 {
        self.data
    }
    pub fn set(&mut self, _data: u32) -> bool {
        self.data = _data;
        true
    }
}

pub struct Value {
    pub data: u32,
}

impl Value {
    pub fn get(&self) -> u32 {
        self.data
    }
    pub fn set(&mut self, _data: u32) -> bool {
        self.data = _data;
        true
    }
}
