


fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/hello.proto"], &["protos"])?;
    Ok(())
}