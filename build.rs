fn main() {
    tonic_build::compile_protos("proto/TradeLogGrpc.proto").unwrap();
}