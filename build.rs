use glob::glob;
use std::{fs, io};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let protos: Vec<PathBuf> = glob("data-plane-api/envoy/**/*.proto")
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    for proto in protos {
        let relative_path = proto.strip_prefix("data-plane-api").unwrap();
        let out_dir = Path::new("src").join(relative_path.parent().unwrap());
        fs::create_dir_all(&out_dir)?;

        let mut config = prost_build::Config::new();
        config.disable_comments(["."]);

        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .compile_well_known_types(true)
            .include_file("mod.rs")
            .out_dir(out_dir)
            .compile_with_config(
                config,
                &[proto],
                &[
                    "data-plane-api",
                    "xds",
                    "protoc-gen-validate",
                    "googleapis",
                    "opentelemetry-proto",
                    "opencensus-proto/src",
                    "client_model",
                ],
            )?;
    }

    Ok(())
}
