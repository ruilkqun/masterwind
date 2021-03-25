use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ CreateContainerRequest,CreateContainerResponse };

pub fn create_container(request:Request<CreateContainerRequest>) -> CreateContainerResponse {

        println!("Got a request: {:?}", request);
        let reply = CreateContainerResponse {
            container_id:"container_id".to_string()
        };
        return reply;
}