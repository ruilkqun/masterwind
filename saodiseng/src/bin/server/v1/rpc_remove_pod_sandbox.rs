use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ RemovePodSandboxRequest,RemovePodSandboxResponse };

pub fn remove_pod_sandbox(request:Request<RemovePodSandboxRequest>) -> RemovePodSandboxResponse {

        println!("Got a request: {:?}", request);
        let reply = RemovePodSandboxResponse {
        };
        return reply;
}