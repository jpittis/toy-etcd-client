use std::io;

fn main() -> io::Result<()> {
    let mut config = prost_build::Config::new();
    config.disable_comments(["."]);
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_well_known_types(true)
        .include_file("mod.rs")
        .compile_with_config(
            config,
            &[
                "etcd/api/etcdserverpb/rpc.proto",
                "etcd/api/authpb/auth.proto",
                "etcd/api/mvccpb/kv.proto",
            ],
            &[".", "etcd/api", "protobuf", "googleapis", "grpc-gateway"],
        )?;
    Ok(())
}
