fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .build_server(true)
        .build_client(true)
        .out_dir("src/infrastructure/grpc/generated")
        .compile_protos(
            &[
                "proto/bucket.proto",
                "proto/access_key.proto",
                "proto/cluster.proto",
                "proto/node.proto",
                "proto/block.proto",
                "proto/worker.proto",
                "proto/utility.proto",
                "proto/object.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
