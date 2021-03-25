use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ RemoveContainerRequest,RemoveContainerResponse };

pub fn remove_container(request:Request<RemoveContainerRequest>) -> RemoveContainerResponse {

        println!("Got a request: {:?}", request);
        let reply = RemoveContainerResponse {
        };
        return reply;
}