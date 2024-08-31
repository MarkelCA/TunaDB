pub mod command;
pub mod config;
pub mod index;
pub mod storage;

pub mod proto {
    include!("./protobuf/command.rs");
}
