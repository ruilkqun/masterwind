use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ RemovePodSandboxRequest,RemovePodSandboxResponse };

pub fn remove_pod_sandbox(request:Request<RemovePodSandboxRequest>) -> RemovePodSandboxResponse {

        println!("Got a request: {:?}", request);
        let reply = RemovePodSandboxResponse {
        };
        return reply;
}