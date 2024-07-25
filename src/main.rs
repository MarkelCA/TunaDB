use clap::Parser;
use storage::Engine;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[clap(short)]
    file_name: String,
    key: String,
    value: String,
}

fn main() {
    let args = CliArgs::parse();

    let mut engine = storage::BinaryEngine::new(&args.file_name);
    engine.set(&args.key, &args.value);
    // engine.get(&args.key);
}

mod storage {
    pub trait Engine {
        fn new(file_path: &str) -> Self;
        fn set(&mut self, key: &str, value: &str);
        fn get(&self, key: &str) -> Option<String>;
    }

    const ENCODING_VERSION: u8 = 1;

    use std::{fs::File, fs::OpenOptions, io::Write, path::Path};

    pub struct BinaryEngine {
        file: File,
    }

    impl Engine for BinaryEngine {
        fn new(file_path: &str) -> Self {
            let mut file_options = OpenOptions::new();
            file_options.append(true).write(true);

            let file_exists = !Path::new(file_path).exists();
            if file_exists {
                file_options = file_options.create_new(true).append(true).clone();
            }

            let mut file = file_options.open(file_path).expect("Couldn't open file");

            if file_exists {
                if let Err(e) = file.write(&[ENCODING_VERSION]) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }

            BinaryEngine { file }
        }

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
}
