use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ RunPodSandboxRequest,RunPodSandboxResponse };


pub async fn run_pod_sandbox_impl_v1alpha2(request:Request<RunPodSandboxRequest>) -> RunPodSandboxResponse {
        let run_pod_sandbox_request = request.into_inner();
        let pod_sandbox_config_1 = run_pod_sandbox_request.config;
        let pod_sandbox_config = match pod_sandbox_config_1 {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        let runtime_handler = run_pod_sandbox_request.runtime_handler;


        let reply = RunPodSandboxResponse{
                pod_sandbox_id: "".to_string()
        };
        return reply
}