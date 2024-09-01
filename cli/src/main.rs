extern crate core;

use clap::Parser;
use command::Command as TunaCommand;
use core::command::Command;
use core::proto::command::Operation;
use core::proto::Response;
use prost::Message;
use std::io::stdin;
use std::net::TcpStream;
use std::process::ExitCode;

mod command;
use std::io::{Read, Write};

use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[clap(subcommand)]
    command: TunaCommand,
}

#[tokio::main]
async fn main() -> ExitCode {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Couldn't connect to server");

    loop {
        let mut buffer = String::new();
        let _ = stdin().read_line(&mut buffer);

        let com = Command::from_str(&buffer).expect("Couldn't parse command");
        let cmd = com.to_proto_command();

        println!("Command: {:?}", cmd);

        let mut buf = Vec::new();
        buf.reserve(cmd.encoded_len());
        cmd.encode(&mut buf).unwrap();

        stream.write(&buf).expect("Couldn't write to server");

        let mut response = [0; 128];
        let n = stream
            .read(&mut response)
            .expect("Couldn't read from server");

        let r = Response::decode(&response[..n]).expect("Couldn't decode response");

        println!("Response: {:?}", r);

        if buffer.trim() == "exit" {
            println!("bye");
            break;
        }
    }

    ExitCode::from(0)
}

// fn main_old() {
//     let args = CliArgs::parse();
//     let config = config::parse().expect("config couldn't be found");
//     let mut engine = storage::new_engine(&config.file_path).expect("Couldn't create engine");
//
//     match command::run(config, &mut engine, args.command).await {
//         Err(err) => {
//             println!("Error: {}", err.to_string());
//             ExitCode::from(1)
//         }
//         Ok(_) => ExitCode::from(0),
//     }
// }
