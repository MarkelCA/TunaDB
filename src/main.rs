use clap::Parser;

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

    let mut w = storage::Writer::new(&args.file_name);
    w.write(&args.key, &args.value);

    println!("file_name: {}", args.file_name);
}

mod storage {
    const ENCODING_VERSION: u8 = 1;

    use std::{fs::File, fs::OpenOptions, io::Write, path::Path};

    pub struct Writer {
        file: File,
    }

    impl Writer {
        pub fn new(file_path: &str) -> Writer {
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

            Writer { file }
        }

        pub fn write(&mut self, key: &str, value: &str) {
            let serialized = String::from(format!("{}{}{}{}", key.len(), key, value.len(), value));

            if let Err(e) = self.file.write(serialized.as_bytes()) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}
