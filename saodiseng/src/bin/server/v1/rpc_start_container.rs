use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ StartContainerRequest,StartContainerResponse };

pub fn start_container(request:Request<StartContainerRequest>) -> StartContainerResponse {

        println!("Got a request: {:?}", request);
        let reply = StartContainerResponse {
        };
        return reply;
}