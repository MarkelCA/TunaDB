# TunaDB 🐟
A key-value storage system written in Rust for learning purposes. It currently uses a simple length-prefixed binary encoding format for storage files and an in-memory byte offset HashMap as its indexing strategy. The server uses protocol buffers for communication with clients and is implemented using the gRPC framework. The client is a simple command-line interface that allows users to interact with the server using a TCP connection.

Implementation details, including the storage/indexing algorithms and communication protocols, are abstracted and subject to change. There are plans to implement more sophisticated data structures, such as B-Trees and LSM-Trees, in the future, as well as a custom protocol for communication between the server and clients.
## Build

### Requirements
- [Git](https://git-scm.com/)
- [Cargo](https://github.com/rust-lang/cargo)
- [protoc](https://grpc.io/docs/protoc-installation/)

```bash
git clone https://github.com/MarkelCA/tunadb.git
cd tunadb
# For cli
cargo install --path ./cli
# For tcp server
cargo install --path ./tcp
```


## Usage
### cli
```
$ tuna
TunaDB. A simple key-value storage written in Rust

Usage: tuna <COMMAND>

Commands:
  get     Get the value for the specified key
  set     Sets the value for the specified key
  del     Deletes the specified key
  config  Manages the database configuration
  list    Lists all keys in the database
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
### tcp
Start the server:
```
tuna-server --log-level info --port 8080
```
Connect to the server (in another terminal):
```
$ nc localhost 8080
help
Commands:
get <key> - Get the value for the specified key
set <key> <value> - Sets the value for the specified key
del <key> - Deletes the specified key
list - Lists all keys in the database
help - Prints the help message
```
