fn main() {
    prost_build::compile_protos(&["storeddata.proto"], &["."]).unwrap();
    capnpc::CompilerCommand::new()
        .file("storeddata.capnp")
        .output_path(".")
        .run()
        .expect("schema compiler command");
}
