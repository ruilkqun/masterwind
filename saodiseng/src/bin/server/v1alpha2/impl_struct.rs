#[derive(Default)]
pub struct MyK8sRuntimeV1alpha2 {}

#[derive(Default)]
pub struct MyK8sImageV1alpha2 {}

pub mod k8s_v1alpha2 {
    tonic::include_proto!("runtime.v1alpha2");
}

