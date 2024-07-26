const ENCODING_VERSION: u8 = 1;
const KEY_LENGTH_SIZE: usize = 1;
const VALUE_LENGTH_SIZE: usize = 2;

use std::io::{Read, Seek};
use std::{fs::File, fs::OpenOptions, io::Write, path::Path};

use anyhow::Context;

pub trait Engine {
    fn set(&mut self, key: &str, value: &str);
    fn get(&mut self, key: &str) -> Option<String>;
}

fn open_file(file_path: &str) -> File {
    let file_exists = Path::new(file_path).exists();
    let mut open_options = OpenOptions::new();
    let mut file_options = open_options.append(true).write(true).read(true);

    if !file_exists {
        file_options = file_options.create(true)
    }

    let mut file = file_options.open(file_path).expect("Couldn't open file");

    if !file_exists {
        if let Err(e) = file.write(&[ENCODING_VERSION]) {
            panic!("Couldn't write to file: {}", e);
        }
    }

    file
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
    file: File,
}

/**
* Factory method for Engine instances. It reads
* the encoding version from the file (first byte) and
* returns the appropriate Engine implementation.
*/
pub fn new_engine(file_path: &str) -> Box<dyn Engine> {
    let mut file = open_file(file_path);
    let mut version = [0; 1];

    // We reset the file cursor to the start of the file
    file.seek(std::io::SeekFrom::Start(0))
        .with_context(|| format!("Seeking to start of file {}", file_path))
        .expect("Couldn't seek to start");

    file.read_exact(&mut version)
        .with_context(|| format!("Reading encoding version from file {}", file_path))
        .expect("Couldn't read version");

    match version[0] {
        1 => Box::new(BinaryEngineV1::new(file_path)),
        2 => Box::new(LSMTreeEngine::new(file_path)),
        _ => panic!("Unsupported encoding version ({})", version[0]),
    }
}

impl BinaryEngineV1 {
    pub fn new(file_path: &str) -> Self {
        let file = open_file(file_path);

        BinaryEngineV1 { file }
    }
}

impl Engine for BinaryEngineV1 {
    fn get(&mut self, key: &str) -> Option<String> {
        let mut value: Option<String> = None;
        self.file
            .seek(std::io::SeekFrom::Start(1))
            .with_context(|| format!("Seeking to start of data in file"))
            .expect("Couldn't seek to start");

        let file_size = self.file.metadata().unwrap().len();

        while self.file.stream_position().unwrap() < file_size {
            let mut key_length_buffer = [0; KEY_LENGTH_SIZE];
            let _ = self.file.read_exact(&mut key_length_buffer);
            let key_length = key_length_buffer[0] as usize;

            let mut current_key: Vec<u8> = Vec::with_capacity(key_length as usize);
            current_key.resize(key_length as usize, 0);

            let _ = self.file.read_exact(&mut current_key);

            let current_key_str = String::from_utf8(current_key).unwrap();

            let mut value_length_buffer = [0; VALUE_LENGTH_SIZE];
            let _ = self.file.read_exact(&mut value_length_buffer);
            let value_length = u16::from_be_bytes(value_length_buffer);

            let mut current_value: Vec<u8> = Vec::with_capacity(value_length as usize);
            current_value.resize(value_length as usize, 0);

            let _ = self.file.read_exact(&mut current_value);
            let value_str = String::from_utf8(current_value).unwrap();

            if current_key_str == key {
                value = Some(value_str);
            }
        }
        value
    }

    fn set(&mut self, key: &str, value: &str) {
        let mut bytes =
            Vec::with_capacity(KEY_LENGTH_SIZE + key.len() + VALUE_LENGTH_SIZE + value.len());

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

        if let Err(e) = self.file.write_all(&bytes) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

/**
* Uses a LSM-tree to store key-value pairs in a file.
*/
struct LSMTreeEngine {
    file: File,
}

impl LSMTreeEngine {
    pub fn new(file_path: &str) -> Self {
        let file = open_file(file_path);

        LSMTreeEngine { file }
    }
}

impl Engine for LSMTreeEngine {
    fn get(&mut self, key: &str) -> Option<String> {
        unimplemented!()
    }

    fn set(&mut self, key: &str, value: &str) {
        unimplemented!()
    }
}
