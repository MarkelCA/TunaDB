# sallydb

Key-value storage written in rust for learning purposes. Right now uses a simple length-prefixed encoding for the binary serialization. But there's plans to use other strategies such as LSM trees.

## Build

### Requirements
- [Git](https://git-scm.com/)
- [Cargo](https://github.com/rust-lang/cargo)

```bash
git clone https://github.com/MarkelCA/sallydb.git
cd sallydb
# For cli
cargo install --path ./cli
# For tcp server
cargo install --path ./tcp
```


## Usage
### cli
```
$ sallydb
SallyDB. A simple key-value storage

Usage: sallydb <COMMAND>

Commands:
  get     Get the value for the specified key
  set     Sets the value for the specified key
  config  Manages the database configuration
  list    Lists all keys in the database
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
