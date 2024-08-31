use prost_build;
use std::io::Result;

fn main() -> Result<()> {
    let proto_files = &["src/protobuf/protofiles/items.proto"];

    // The directory to search for imports; can be the same as where your .proto files are
    let proto_includes = &["src/protobuf/"];

    prost_build::Config::new()
        .out_dir("src/protobuf/")
        .compile_protos(proto_files, proto_includes)
        .expect("Failed to compile .proto files");

    Ok(())
}
