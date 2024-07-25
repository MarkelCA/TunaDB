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

mod storage;

fn main() {
    let args = CliArgs::parse();

    let mut engine = storage::match_version(&args.file_name);
    engine.set(&args.key, &args.value);
    // engine.get(&args.key);
}
