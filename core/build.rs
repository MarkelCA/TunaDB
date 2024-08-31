use prost_build;
use std::{fs::read_dir, io::Result};

const PROTO_FILES_DIR: &str = "./src/protobuf/protofiles";
const PROTO_OUT_DIR: &str = "./src/protobuf/";

fn main() -> Result<()> {
    let proto_files: Vec<String> = read_dir(PROTO_FILES_DIR)
        .expect("Failed to read protofiles directory")
        .into_iter()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .to_str()
                    .filter(|s| s.ends_with(".proto"))
                    .map(|s| s.to_string())
            })
        })
        .collect();

    // The directory to search for imports; can be the same as where your .proto files are
    let proto_includes = &[PROTO_OUT_DIR];

    prost_build::Config::new()
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir(PROTO_OUT_DIR)
        .compile_protos(&proto_files, proto_includes)
        .expect("Failed to compile .proto files");

    Ok(())
}
