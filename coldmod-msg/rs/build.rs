fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "proto")]
    {
        tonic_build::configure()
            .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
            .compile(&["trace.proto", "source.proto"], &["../proto"])?;
    }
    Ok(())
}
