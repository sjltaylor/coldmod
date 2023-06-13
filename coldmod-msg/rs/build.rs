fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(cfg!(feature = "grpc-client"))
        .build_server(cfg!(feature = "grpc-server"))
        .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .compile(&["trace.proto", "source.proto", "ops.proto"], &["../proto"])?;

    Ok(())
}
