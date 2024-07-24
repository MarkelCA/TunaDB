use std::{fs::OpenOptions, io::Write, path::Path};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[clap(short)]
    file_name: String,
    key: String,
    value: String,
}

const ENCODING_VERSION: u8 = 1;

fn main() {
    let args = CliArgs::parse();

    let mut file_options = OpenOptions::new();
    file_options.append(true).write(true);

    let file_exists = !Path::new(&args.file_name).exists();
    if file_exists {
        file_options = file_options.create_new(true).append(true).clone();
    }

    let mut file = file_options
        .open(&args.file_name)
        .expect("Couldn't open file");

    if file_exists {
        if let Err(e) = file.write(&[ENCODING_VERSION]) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    let serialized = String::from(format!(
        "{}{}{}{}",
        args.key.len(),
        args.key,
        args.value.len(),
        args.value
    ));
    if let Err(e) = file.write(serialized.as_bytes()) {
        eprintln!("Couldn't write to file: {}", e);
    }

    println!("file_name: {}", args.file_name);
}
