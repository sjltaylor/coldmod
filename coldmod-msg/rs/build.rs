fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "proto")]
    {
        tonic_build::compile_protos("../proto/trace.proto")?;
        tonic_build::compile_protos("../proto/source.proto")?;
    }
    Ok(())
}
