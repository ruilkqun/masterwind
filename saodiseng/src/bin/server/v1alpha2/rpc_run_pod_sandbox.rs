use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ RunPodSandboxRequest,RunPodSandboxResponse };

pub fn run_pod_sandbox(request:Request<RunPodSandboxRequest>) -> RunPodSandboxResponse {

        println!("Got a request: {:?}", request);
        let reply = RunPodSandboxResponse {
            pod_sandbox_id:"pod_sandbox_id".to_string()
        };
        return reply;
}