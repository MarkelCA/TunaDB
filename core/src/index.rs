use anyhow::Error;
use async_trait::async_trait;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek},
    sync::Arc,
};
use tokio::sync::Mutex;

use crate::storage::VALUE_LENGTH_SIZE;

#[async_trait]
pub trait OffsetIndexer: Send {
    async fn get(&self, key: &str) -> Result<Option<String>, Error>;
    async fn set(&mut self, key: &str, offset: u64);
    async fn delete(&mut self, key: &str);
}

#[derive(Clone)]
pub struct BinaryOffsetIndexer {
    file: Arc<Mutex<File>>,
    offsets: HashMap<String, u64>,
}

impl BinaryOffsetIndexer {
    pub fn new(file: Arc<Mutex<File>>) -> Self {
        BinaryOffsetIndexer {
            file,
            offsets: HashMap::new(),
        }
    }
}

#[async_trait]
impl OffsetIndexer for BinaryOffsetIndexer {
    async fn get(&self, key: &str) -> Result<Option<String>, Error> {
        let offset = self.offsets.get(key);

        if offset.is_none() {
            return Ok(None);
        }

        let offset = offset.unwrap(); // Safe to unwrap because we checked for None above

        let _ = self
            .file
            .lock()
            .await
            .seek(std::io::SeekFrom::Start(*offset));

        let mut value_length_buffer = [0; VALUE_LENGTH_SIZE];
        let _ = self.file.lock().await.read_exact(&mut value_length_buffer);
        let value_length = u16::from_be_bytes(value_length_buffer);

        let mut current_value: Vec<u8> = Vec::with_capacity(value_length as usize);
        current_value.resize(value_length as usize, 0);

        let _ = self.file.lock().await.read_exact(&mut current_value);
        let value_str = String::from_utf8(current_value)?;

        Ok(Some(value_str))
    }

    async fn set(&mut self, key: &str, offset: u64) {
        self.offsets.insert(key.to_string(), offset);
    }

    async fn delete(&mut self, key: &str) {
        self.offsets.remove(key);
    }
}
