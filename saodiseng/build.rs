fn main() {
    tonic_build::compile_protos("proto/v1/api.proto").unwrap();

    tonic_build::compile_protos("proto/v1alpha2/api.proto").unwrap();
}