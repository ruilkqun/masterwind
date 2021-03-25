use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ PodSandboxStatusRequest,PodSandboxStatusResponse };
use std::collections::HashMap;

pub fn pod_sandbox_status(request:Request<PodSandboxStatusRequest>) -> PodSandboxStatusResponse {

        println!("Got a request: {:?}", request);
        let mut info_tmp = HashMap::<String,String>::new();
        info_tmp.insert("PodSandboxStatusResponse".to_string(),"pod_sandbox_status".to_string());
        let reply = PodSandboxStatusResponse {
            status:None,
            info: info_tmp,
        };
        return reply;
}