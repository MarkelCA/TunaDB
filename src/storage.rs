const ENCODING_VERSION: u8 = 1;
use std::io::Read;
use std::{fs::File, fs::OpenOptions, io::Write, path::Path};

pub trait Engine {
    fn set(&mut self, key: &str, value: &str);
    fn get(&self, key: &str) -> Option<String>;
}

fn open_file(file_path: &str) -> File {
    let mut file_options = OpenOptions::new();
    file_options.append(true).write(true).read(true);

    let file_exists = Path::new(file_path).exists();

    if !file_exists {
        file_options = file_options.create_new(true).append(true).clone();
    }

    let mut file = file_options.open(file_path).expect("Couldn't open file");

    if !file_exists {
        if let Err(e) = file.write(&[ENCODING_VERSION]) {
            panic!("Couldn't write to file: {}", e);
        }
    }

    file
}

pub struct BinaryEngineV1 {
    file: File,
}

pub fn match_version(file_path: &str) -> Box<dyn Engine> {
    println!("Opening file: {}", file_path);
    let mut file = open_file(file_path);
    let mut version = [0; 1];
    println!("Reading version {:?}", version);
    file.read_exact(&mut version)
        .expect("Couldn't read version");

    match version[0] {
        1 => Box::new(BinaryEngineV1::new(file_path)),
        2 => Box::new(BinaryEngineV2::new(file_path)),
        _ => panic!("Unsupported version"),
    }
}

impl BinaryEngineV1 {
    pub fn new(file_path: &str) -> Self {
        println!("Using BinaryEngineV1");
        let file_exists = Path::new(file_path).exists();
        let mut file = open_file(file_path);

        BinaryEngineV1 { file }
    }
}

impl Engine for BinaryEngineV1 {
    fn get(&self, key: &str) -> Option<String> {
        unimplemented!()
    }

    fn set(&mut self, key: &str, value: &str) {
        let mut bytes = Vec::with_capacity(1 + key.len() + 2 + value.len());

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

struct BinaryEngineV2 {
    file: File,
}

impl BinaryEngineV2 {
    pub fn new(file_path: &str) -> Self {
        println!("Using BinaryEngineV2");
        BinaryEngineV2 {
            file: open_file(file_path),
        }
    }
}

impl Engine for BinaryEngineV2 {
    fn get(&self, key: &str) -> Option<String> {
        unimplemented!()
    }

    fn set(&mut self, key: &str, value: &str) {
        unimplemented!()
    }
}
