#[derive(Default)]
pub struct MyK8sRuntimeV1 {}

#[derive(Default)]
pub struct MyK8sImageV1 {}

pub mod k8s_v1 {
    tonic::include_proto!("runtime.v1");
}

