use anyhow::Result;
use prost::Message;

use crate::{command::Command, proto};

pub trait CommandSerializer: Send + Sync {
    fn encode(&self, command: &Command, buf: &mut Vec<u8>) -> Result<()>;
    fn decode(&self, bytes: &[u8]) -> Result<Command>;
    fn encoded_len(&self, command: &Command) -> usize;
}

pub struct ProtoCommandSerializer;

impl CommandSerializer for ProtoCommandSerializer {
    fn encode(&self, command: &Command, buf: &mut Vec<u8>) -> Result<()> {
        let proto_command = command.to_proto_command();
        proto_command.encode(buf)?;
        buf.reserve(proto_command.encoded_len());
        Ok(())
    }

    fn decode(&self, bytes: &[u8]) -> Result<Command> {
        let proto_command = proto::Command::decode(bytes)?;
        Ok(Command::from_proto_command(proto_command))
    }

    fn encoded_len(&self, command: &Command) -> usize {
        command.to_proto_command().encoded_len()
    }
}
