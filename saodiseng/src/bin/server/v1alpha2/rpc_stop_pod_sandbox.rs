use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ StopPodSandboxRequest,StopPodSandboxResponse };

pub fn stop_pod_sandbox(request:Request<StopPodSandboxRequest>) -> StopPodSandboxResponse {

        println!("Got a request: {:?}", request);
        let reply = StopPodSandboxResponse {
        };
        return reply;
}