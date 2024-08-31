extern crate core;

use clap::Parser;
use command::Command as TunaCommand;
use core::proto::command::Operation;
use prost::Message;
use std::io::stdin;
use std::net::TcpStream;
use std::process::ExitCode;

mod command;
use std::io::{Read, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[clap(subcommand)]
    command: TunaCommand,
}

#[tokio::main]
async fn main() -> ExitCode {
    let mut buffer = String::new();

    let mut cmd = core::proto::Command::default();
    cmd.key = "x".to_string();
    cmd.operation = Operation::Get as i32;
    let mut buf = Vec::new();
    buf.reserve(cmd.encoded_len());
    cmd.encode(&mut buf).unwrap();

    println!("{:?}", buf);

    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Couldn't connect to server");

    loop {
        let _ = stdin().read_line(&mut buffer);

        stream.write(&buf).expect("Couldn't write to server");
        println!("Sent command");

        let mut response = [0; 128];
        stream
            .read(&mut response)
            .expect("Couldn't read from server");

        let response_str = String::from_utf8_lossy(&response);
        println!("Response: {:?}", response_str);

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
