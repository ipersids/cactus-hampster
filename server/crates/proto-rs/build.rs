use std::fs::create_dir_all;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_path: &Path = Path::new("./src/generated");

    create_dir_all(out_path)?;

    let proto_files = vec![
        "../../../proto/controller-types/controller_types_v1.proto",
        "../../../proto/host-types/host_types_v1.proto",
    ];

    prost_build::Config::new()
        .out_dir(out_path)
        .compile_protos(&proto_files, &["../../../proto"])?;

    Ok(())
}
