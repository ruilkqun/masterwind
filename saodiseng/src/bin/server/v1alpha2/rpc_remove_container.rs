use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ RemoveContainerRequest,RemoveContainerResponse };

pub fn remove_container(request:Request<RemoveContainerRequest>) -> RemoveContainerResponse {

        println!("Got a request: {:?}", request);
        let reply = RemoveContainerResponse {
        };
        return reply;
}