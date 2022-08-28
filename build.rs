use std::{env, path::PathBuf};
fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir("./src")
        .compile(
            &[
                "./proto/labelpbtypes.proto",
                "./proto/prompbtypes.proto",
                "./proto/rpc.proto",
                "./proto/types.proto",
            ],
            &["./proto/"],
        )
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", "./proto/labelpbtypes.proto");
}
