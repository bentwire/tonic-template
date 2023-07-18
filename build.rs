fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .build_transport(true)
        .compile_well_known_types(true)
        .out_dir("src/proto/")
        .include_file("mod.rs")
        .protoc_arg("--experimental_allow_proto3_optional")

        .compile(
            //&["proto/ServiceProtos/progressiveControllerSrv.proto", "proto/ServiceProtos/commonDataSrv.proto", "proto/Progressives/progressiveModels.proto", "proto/Common/common.proto"],
            &["proto/simple.proto"],
            &["proto/"])?;
    Ok(())
}
