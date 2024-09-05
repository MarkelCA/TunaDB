use anyhow::Result;
use prost::Message;

use crate::{command::Command, proto, response::Response};

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

pub trait ResponseSerializer: Send + Sync {
    fn encode(&self, response: &Response, buf: &mut Vec<u8>) -> Result<()>;
    fn decode(&self, bytes: &[u8]) -> Result<Response>;
    fn encoded_len(&self, response: &Response) -> usize;
}

pub struct ProtoResponseSerializer;

impl ResponseSerializer for ProtoResponseSerializer {
    fn encode(&self, response: &Response, buf: &mut Vec<u8>) -> Result<()> {
        let proto_response = response.to_proto_response();
        proto_response.encode(buf)?;
        buf.reserve(proto_response.encoded_len());
        Ok(())
    }

    fn decode(&self, bytes: &[u8]) -> Result<Response> {
        let proto_response = proto::Response::decode(bytes)?;
        Ok(Response::from_proto_response(proto_response))
    }

    fn encoded_len(&self, response: &Response) -> usize {
        response.to_proto_response().encoded_len()
    }
}
