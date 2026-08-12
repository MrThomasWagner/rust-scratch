
#[derive(Debug)]
pub struct Asparagus {
    pub length: u32
}

impl Asparagus {
    pub fn new() -> Self {
        Self { length: 12 }
    }
}

impl Default for Asparagus {
    fn default() -> Self {
        Self::new()
    }
}
