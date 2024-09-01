extern crate core;

use clap::Parser;
use command::Command as TunaCommand;
use core::command::Command;
use core::proto::command::Operation;
use core::proto::response::Status;
use core::proto::Command as ProtoCommand;
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

        let mut buf = Vec::new();
        buf.reserve(cmd.encoded_len());
        cmd.encode(&mut buf).unwrap();

        stream.write(&buf).expect("Couldn't write to server");

        let mut response_bytes = [0; 128];
        let n = stream
            .read(&mut response_bytes)
            .expect("Couldn't read from server");

        let response = Response::decode(&response_bytes[..n]).expect("Couldn't decode response");
        print_response(cmd, response);

        if buffer.trim() == "exit" {
            println!("bye");
            break;
        }
    }

    ExitCode::from(0)
}

fn print_response(command: ProtoCommand, response: Response) {
    match response.status() {
        Status::Unespecified => {
            println!("UNSPECIFIED");
        }
        Status::Ok => match command.operation() {
            Operation::Get => {
                println!("{}", response.content());
            }
            Operation::Set => {
                println!("ok");
            }
            Operation::Del => {
                println!("ok");
            }
            Operation::List => {
                println!("{}", response.content());
            }
        },
        Status::Error => {
            println!("error: {}", response.content());
        }
        Status::NotFound => {
            println!("(nil)");
        }
    }
}
