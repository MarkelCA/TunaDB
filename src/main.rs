use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[clap(short)]
    file_name: String,
    key: String,
    value: String,
}

mod storage;

fn main() {
    let args = CliArgs::parse();

    let mut engine = storage::new_engine(&args.file_name);
    // engine.set(&args.key, &args.value);
    match engine.get(&args.key) {
        Some(value) => println!("{}: {}", args.key, value),
        None => println!("{} not found", args.key),
    }
}
