# tunadb

Key-value storage written in rust for learning purposes. Working on cli and tcp-server implementations. Right now uses a simple length-prefixed binary encoding format for the storage files, but there's plans to use other strategies such as LSM trees in the future.

## Build

### Requirements
- [Git](https://git-scm.com/)
- [Cargo](https://github.com/rust-lang/cargo)

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
$ tunadb
TunaDB. A simple key-value storage

Usage: tunadb <COMMAND>

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
