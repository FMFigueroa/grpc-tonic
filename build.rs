fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        //.build_server(true)
        //.out_dir("proto/helloworld/generated")
        // you can change the generated code's location
        .compile(
            &["proto/helloworld/helloworld.proto"],
            &["proto"],
            // specify the root location to search proto dependencies
        )?;

    tonic_build::configure()
        //.build_server(true)
        //.out_dir("proto/job/generated")
        // you can change the generated code's location
        .compile(
            &["proto/job/job.proto"],
            &["proto"],
            // specify the root location to search proto dependencies
        )?;

    tonic_build::configure()
        .type_attribute("routeguide.Point", "#[derive(Hash)]")
        .compile(&["proto/streaming/streaming.proto"], &["proto"])?;

    Ok(())
}
