use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ ListPodSandboxRequest,ListPodSandboxResponse };

pub fn list_pod_sandbox(request:Request<ListPodSandboxRequest>) -> ListPodSandboxResponse {

        println!("Got a request: {:?}", request);
        let mut tmp_items = Vec::new();
        let reply = ListPodSandboxResponse {
            items: tmp_items
        };
        return reply;
}