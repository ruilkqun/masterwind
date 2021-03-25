use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ StopContainerRequest,StopContainerResponse };

pub fn stop_container(request:Request<StopContainerRequest>) -> StopContainerResponse {

        println!("Got a request: {:?}", request);
        let reply = StopContainerResponse {
        };
        return reply;
}