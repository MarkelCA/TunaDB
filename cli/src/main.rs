extern crate core;

use anyhow::Error;
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
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect to server: {}", e);
            return ExitCode::from(1);
        }
    };

    loop {
        let mut buffer = String::new();
        let cmd = match read_command(&mut buffer) {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("error: {}", e);
                continue;
            }
        };

        if buffer.trim() == "exit" {
            println!("bye");
            break;
        }

        if let Err(e) = send_command(&cmd, &mut stream) {
            println!("error: {}", e);
            continue;
        }

        match read_response(&mut stream) {
            Ok(response) => print_response(cmd, response),
            Err(e) => println!("error: {}", e),
        }
    }

    ExitCode::from(0)
}

fn read_command(mut buffer: &mut String) -> Result<ProtoCommand, Error> {
    let _ = stdin().read_line(&mut buffer);
    let cmd = Command::from_str(&buffer)?;
    Ok(cmd.to_proto_command())
}

fn send_command(cmd: &ProtoCommand, stream: &mut TcpStream) -> Result<(), Error> {
    let mut buf = Vec::new();
    buf.reserve(cmd.encoded_len());
    cmd.encode(&mut buf)?;
    stream.write(&buf)?;
    Ok(())
}

fn read_response(stream: &mut TcpStream) -> Result<Response, Error> {
    let mut response_bytes = [0; 128];
    let n = stream.read(&mut response_bytes)?;
    Ok(Response::decode(&response_bytes[..n])?)
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
