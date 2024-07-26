use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[clap(short)]
    file_name: String,
    #[clap(subcommand)]
    command: Command,
}

mod storage;

#[derive(Parser, Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
}

fn main() {
    let args = CliArgs::parse();

    let mut engine = storage::new_engine(&args.file_name);

    match args.command {
        Command::Get { key } => match engine.get(&key) {
            Some(value) => println!("{}: {}", key, value),
            None => println!("{} not found", key),
        },
        Command::Set { key, value } => {
            engine.set(&key, &value);
        }
    }
}
