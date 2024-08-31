use std::collections::HashMap;

pub trait OffsetIndexer: Send + Clone {
    fn get(&self, key: &str);
    fn set(&mut self, key: &str, offset: u64);
}

#[derive(Clone)]
pub struct BinaryOffsetIndexer {
    offsets: HashMap<String, u64>,
}

impl BinaryOffsetIndexer {
    pub fn new() -> Self {
        BinaryOffsetIndexer {
            offsets: HashMap::new(),
        }
    }
}

impl OffsetIndexer for BinaryOffsetIndexer {
    fn get(&self, key: &str) {
        println!(
            "Offset for key {} is {}",
            key,
            self.offsets.get(key).unwrap_or(&0)
        );
    }

    fn set(&mut self, key: &str, offset: u64) {
        self.offsets.insert(key.to_string(), offset);
    }
}
