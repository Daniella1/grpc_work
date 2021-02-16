fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/unifmu_fmi2.proto")?;
    tonic_build::compile_protos("proto/handshake.proto")?;
    Ok(())
}
