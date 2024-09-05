extern crate core;

use anyhow::Error;
use clap::Parser;
use core::command::Command;
use core::proto::response::Status;
use core::proto::Response;
use core::serializer::CommandSerializer;
use core::serializer::ProtoCommandSerializer;
use prost::Message;
use std::io::stdin;
use std::net::TcpStream;
use std::process::ExitCode;
use std::str::FromStr;

mod command;
use std::io::{Read, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Host to connect to
    #[clap(long, default_value = "127.0.0.1")]
    host: String,
    /// Port to connect to
    #[clap(long, default_value = "5880")]
    port: u16,
}

// TODO: refactor
fn get_serializer() -> Box<dyn CommandSerializer> {
    Box::new(ProtoCommandSerializer)
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = CliArgs::parse();

    let command_serializer = get_serializer();

    println!("Connecting to {}:{}...", args.host, args.port);
    let mut stream = match TcpStream::connect(format!("{}:{}", args.host, args.port).as_str()) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect to server: {}", e);
            return ExitCode::from(1);
        }
    };

    println!("Connected to server. Type 'help' for a list of commands.");

    loop {
        // Read command (TODO: refactor)
        let mut buffer = String::new();
        let _ = stdin().read_line(&mut buffer);

        if buffer.trim() == "exit" {
            println!("bye!");
            break;
        }

        if buffer.trim() == "help" {
            print_help();
            continue;
        }

        let cmd = match Command::from_str(&buffer) {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("error: {}", e);
                continue;
            }
        };

        // Send command
        if let Err(e) = send_command(&cmd, &command_serializer, &mut stream) {
            println!("error: {}", e);
            continue;
        }

        // Read response
        match read_response(&mut stream) {
            Ok(response) => print_response(cmd, response),
            Err(e) => println!("error: {}", e),
        }
    }

    ExitCode::from(0)
}

fn print_help() {
    println!("Available commands:");
    println!("  get <key>");
    println!("  set <key> <value");
    println!("  del <key>");
    println!("  list");
    println!("  exit");
}

fn send_command(
    cmd: &Command,
    serializer: &Box<dyn CommandSerializer>,
    stream: &mut TcpStream,
) -> Result<(), Error> {
    let mut buf = Vec::new();
    buf.reserve(serializer.encoded_len(&cmd));
    serializer.encode(cmd, &mut buf)?;
    stream.write(&buf)?;
    Ok(())
}

fn read_response(stream: &mut TcpStream) -> Result<Response, Error> {
    let mut response_bytes = [0; 128];
    let n = stream.read(&mut response_bytes)?;
    Ok(Response::decode(&response_bytes[..n])?)
}

fn print_response(command: Command, response: Response) {
    match response.status() {
        Status::Unespecified => {
            println!("UNSPECIFIED");
        }
        Status::Ok => match command {
            Command::Get { .. } => {
                println!("{}", response.content());
            }
            Command::Set { .. } => {
                println!("ok");
            }
            Command::Del { .. } => {
                println!("ok");
            }
            Command::List => {
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
