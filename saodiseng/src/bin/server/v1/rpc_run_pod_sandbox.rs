use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ RunPodSandboxRequest,RunPodSandboxResponse };

pub fn run_pod_sandbox(request:Request<RunPodSandboxRequest>) -> RunPodSandboxResponse {

        println!("Got a request: {:?}", request);
        let reply = RunPodSandboxResponse {
            pod_sandbox_id:"pod_sandbox_id".to_string()
        };
        return reply;
}