# TunaDB 🐟
A key-value store written in Rust for learning purposes. Inspired by the book "Designing Data-Intensive Applications" by Martin Kleppmann.

### Technical details
It currently uses a simple length-prefixed binary encoding format for storage files and an in-memory byte offset HashMap as its indexing strategy. The server communicates with clients over TCP sockets and uses protocol buffers for data serialization.

Implementation details, including the storage/indexing algorithms and communication protocols, are abstracted and subject to change. There are plans to implement more sophisticated data structures, such as B-Trees and LSM-Trees, in the future, as well as a custom communication protocol.
## Build

### Requirements
- [Git](https://git-scm.com/)
- [Cargo](https://github.com/rust-lang/cargo)
- [protoc](https://grpc.io/docs/protoc-installation/)

```bash
git clone https://github.com/MarkelCA/tunadb.git
cd tunadb
# For the server
cargo install --path ./server
# For the client
cargo install --path ./cli
```

## Usage
### server
You can check the server parameters with `tuna-server --help`:
```
$ tuna-server --help
TunaDB. A simple key-value storage written in Rust.

Usage: tuna-server [OPTIONS]

Options:
  -l, --log-level <LOG_LEVEL>  [default: info] [possible values: error, warn, info, debug, trace]
  -p, --port <PORT>            [default: 5880]
  -h, --help                   Print help
  -V, --version                Print version

```
Start the server:
```
$ tuna-server
[2024-09-01T10:08:57Z INFO  tuna_server] Starting server in port 5880...
[2024-09-01T10:08:57Z INFO  tuna_server] Server started
```

### cli
You can check the client parameters with `tuna --help`:
```
$ tuna --help
TunaDB client. The command line interface for the Tuna database.

Usage: tuna [OPTIONS]

Options:
      --host <HOST>  Host to connect to [default: 127.0.0.1]
      --port <PORT>  Port to connect to [default: 5880]
  -h, --help         Print help
  -V, --version      Print version
```
Start the client:
```
$ tuna
Connecting to 127.0.0.1:5880...
Connected to server. Type 'help' for a list of commands.
help
Available commands:
  get <key>
  set <key> <value
  del <key>
  list
  exit
```
