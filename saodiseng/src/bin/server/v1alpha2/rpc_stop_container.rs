use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ StopContainerRequest,StopContainerResponse };

pub fn stop_container(request:Request<StopContainerRequest>) -> StopContainerResponse {

        println!("Got a request: {:?}", request);
        let reply = StopContainerResponse {
        };
        return reply;
}