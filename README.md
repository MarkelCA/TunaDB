# TunaDB 🐟

Key-value storage written in rust for learning purposes. Working on cli and tcp-server implementations. Right now uses a simple length-prefixed binary encoding format for the storage files, but there's plans to use other strategies such as LSM trees in the future.

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
