pub const ENCODING_VERSION: u8 = 1;
pub const KEY_LENGTH_SIZE: usize = 1;
pub const VALUE_LENGTH_SIZE: usize = 2;
pub const TOMBSTONE_SIZE: usize = 1;

use std::collections::HashSet;
use std::io::{Read, Seek};
use std::sync::Arc;
use std::{fs::File, fs::OpenOptions, io::Write, path::Path};

use crate::index::{BinaryOffsetIndexer, OffsetIndexer};
use anyhow::Error;
use async_trait::async_trait;
use tokio::sync::Mutex;

#[async_trait]
pub trait Engine: Send {
    async fn set(&mut self, key: &str, value: &str) -> std::io::Result<()>;
    async fn delete(&mut self, key: &str) -> std::io::Result<()>;
    async fn get(&mut self, key: &str) -> Result<Option<String>, Error>;
    async fn list(&mut self) -> anyhow::Result<HashSet<String>>;
}

fn open_file(file_path: &str) -> Result<File, std::io::Error> {
    let file_exists = Path::new(file_path).exists();
    let mut open_options = OpenOptions::new();
    let mut file_options = open_options.append(true).write(true).read(true);

    if !file_exists {
        file_options = file_options.create(true)
    }

    let mut file = file_options.open(file_path)?;

    if !file_exists {
        file.write(&[ENCODING_VERSION])?;
        // if let Err(e) = file.write(&[ENCODING_VERSION]) {
        //     panic!("Couldn't write to file: {}", e);
        // }
    }

    Ok(file)
}

/**
* BinaryEngineV1 is an implementation of the Engine
* trait that uses a binary encoding format to store key-value
* pairs in a file.
*
* The encoding format is as follows:
* byte 0: encoding version (1)
* byte 1: length of key (1 byte)
* bytes 2..n: key
* bytes n+1..n+2: length of value (2 bytes, big-endian)
* bytes n+3..n+3+m: value
* where n is the length of the key and m is the length of the value
* in bytes.
*
* Example:
*   01 03 6b 65 79 00 05 76 61 6c 75 65
*   encoding version: 1
*   key length: 3
*   key: "key"
*   value length: 5
*   value: "value"
*
* Note: the encoding version is stored as a single byte.
* Note: the length of the key is stored as a single byte.
* Note: the length of the value is stored as a 16-bit unsigned integer.
* Note: the key and value are stored as UTF-8 strings.
*/
pub struct BinaryEngineV1 {
    file: Arc<Mutex<File>>,
    indexer: Box<dyn OffsetIndexer>,
}

/**
* Factory method for Engine instances. It reads
* the encoding version from the file (first byte) and
* returns the appropriate Engine implementation.
*/
pub fn new_engine(file_path: &str) -> Result<Box<dyn Engine>, std::io::Error> {
    let mut file = open_file(file_path)?;
    let mut version = [0; 1];

    // We reset the file cursor to the start of the file
    file.seek(std::io::SeekFrom::Start(0))?;

    file.read_exact(&mut version)?;

    let file = Arc::new(Mutex::new(open_file(file_path)?));
    let indexer = Box::new(BinaryOffsetIndexer::new(file.clone()));

    match version[0] {
        1 => Ok(Box::new(BinaryEngineV1 { file, indexer })),
        2 => Ok(Box::new(LSMTreeEngine { _file: file })),
        _ => panic!("Unsupported encoding version ({})", version[0]),
    }
}

impl BinaryEngineV1 {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let file = Arc::new(Mutex::new(open_file(file_path)?));
        let indexer = Box::new(BinaryOffsetIndexer::new(file.clone()));

        Ok(BinaryEngineV1 { file, indexer })
    }
}

#[async_trait]
impl Engine for BinaryEngineV1 {
    async fn get(&mut self, key: &str) -> Result<Option<String>, Error> {
        let value = self.indexer.get(key).await?;

        if let Some(value) = value {
            return Ok(Some(value));
        }

        let mut value: Option<String> = None;
        self.file.lock().await.seek(std::io::SeekFrom::Start(1))?; // Skip encoding version byte

        let file_size = self.file.lock().await.metadata()?.len();

        while self.file.lock().await.stream_position()? < file_size {
            let mut key_length_buffer = [0; KEY_LENGTH_SIZE];
            let _ = self.file.lock().await.read_exact(&mut key_length_buffer);
            let key_length = key_length_buffer[0] as usize;

            let mut current_key: Vec<u8> = Vec::with_capacity(key_length as usize);
            current_key.resize(key_length as usize, 0);

            let _ = self.file.lock().await.read_exact(&mut current_key);

            let current_key_str = String::from_utf8(current_key)?;

            let mut value_length_buffer = [0; VALUE_LENGTH_SIZE];
            let _ = self.file.lock().await.read_exact(&mut value_length_buffer);
            let value_length = u16::from_be_bytes(value_length_buffer);

            let mut current_value: Vec<u8> = Vec::with_capacity(value_length as usize);
            current_value.resize(value_length as usize, 0);

            let _ = self.file.lock().await.read_exact(&mut current_value);
            let value_str = String::from_utf8(current_value)?;

            let mut tombstone = [0; 1];
            let _ = self.file.lock().await.read_exact(&mut tombstone);

            if current_key_str == key {
                if tombstone[0] == 1 {
                    value = None
                } else {
                    value = Some(value_str);
                }
            }
        }
        Ok(value)
    }

    async fn set(&mut self, key: &str, value: &str) -> std::io::Result<()> {
        let mut bytes = Vec::with_capacity(
            KEY_LENGTH_SIZE + key.len() + VALUE_LENGTH_SIZE + value.len() + TOMBSTONE_SIZE,
        );

        // Add the length of the key (1 byte)
        bytes.push(key.len() as u8);
        // Add the key bytes
        bytes.extend_from_slice(key.as_bytes());

        // Add the length of the value (2 bytes, zero-padded)
        let value_len = value.len() as u16;
        bytes.push((value_len >> 8) as u8); // High byte
        bytes.push((value_len & 0xFF) as u8); // Low byte

        // Add the value bytes
        bytes.extend_from_slice(value.as_bytes());

        // We add the tombstone byte (not deleted by default)
        bytes.push(0);

        let size = self.file.lock().await.seek(std::io::SeekFrom::End(0))?;

        self.indexer
            .set(key, size + KEY_LENGTH_SIZE as u64 + key.len() as u64)
            .await;

        self.file.lock().await.write_all(&bytes)?;

        Ok(())
    }

    async fn list(&mut self) -> anyhow::Result<HashSet<String>> {
        let mut keys: HashSet<String> = HashSet::new();

        let _ = self.file.lock().await.seek(std::io::SeekFrom::Start(1));

        let file_size = self.file.lock().await.metadata()?.len();
        while self.file.lock().await.stream_position()? < file_size {
            let mut key_length_buffer = [0; KEY_LENGTH_SIZE];
            let _ = self.file.lock().await.read_exact(&mut key_length_buffer);
            let key_length = key_length_buffer[0] as usize;

            let mut current_key: Vec<u8> = Vec::with_capacity(key_length as usize);
            current_key.resize(key_length as usize, 0);

            let _ = self.file.lock().await.read_exact(&mut current_key);

            let current_key_str = String::from_utf8(current_key)?;

            let mut value_length_buffer = [0; VALUE_LENGTH_SIZE];
            let _ = self.file.lock().await.read_exact(&mut value_length_buffer);
            let value_length = u16::from_be_bytes(value_length_buffer);

            let mut current_value: Vec<u8> = Vec::with_capacity(value_length as usize);
            current_value.resize(value_length as usize, 0);

            let _ = self.file.lock().await.read_exact(&mut current_value);
            let _ = String::from_utf8(current_value)?;

            let mut tombstone = [0; 1];
            let _ = self.file.lock().await.read_exact(&mut tombstone);

            if tombstone[0] == 0 {
                keys.insert(current_key_str);
            } else {
                keys.remove(&current_key_str);
            }
        }
        Ok(keys)
    }

    async fn delete(&mut self, key: &str) -> std::io::Result<()> {
        let mut bytes = Vec::with_capacity(KEY_LENGTH_SIZE + key.len() + VALUE_LENGTH_SIZE + 1);

        // Add the length of the key (1 byte)
        bytes.push(key.len() as u8);
        // Add the key bytes
        bytes.extend_from_slice(key.as_bytes());

        // Add the length of the value (2 bytes, zero-padded)
        let value_len = 1 as u16;
        bytes.push((value_len >> 8) as u8); // High byte
        bytes.push((value_len & 0xFF) as u8); // Low byte

        // Add the value bytes
        bytes.push(0);

        // We add the tombstone byte (deleted)
        bytes.push(1);

        self.file.lock().await.write_all(&bytes)?;
        self.indexer.delete(key).await;
        Ok(())
    }
}

/**
* Uses a LSM-tree to store key-value pairs in a file.
*/
pub struct LSMTreeEngine {
    _file: Arc<Mutex<File>>,
}

impl LSMTreeEngine {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let file = Arc::new(Mutex::new(open_file(file_path)?));

        Ok(LSMTreeEngine { _file: file })
    }
}

#[async_trait]
impl Engine for LSMTreeEngine {
    async fn get(&mut self, _key: &str) -> Result<Option<String>, Error> {
        unimplemented!()
    }

    async fn set(&mut self, _key: &str, _value: &str) -> std::io::Result<()> {
        unimplemented!()
    }

    async fn list(&mut self) -> anyhow::Result<HashSet<String>> {
        unimplemented!()
    }

    async fn delete(&mut self, _key: &str) -> std::io::Result<()> {
        unimplemented!()
    }
}
