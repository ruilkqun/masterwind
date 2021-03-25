use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ UpdateContainerResourcesRequest,UpdateContainerResourcesResponse };

pub fn update_container_resources(request:Request<UpdateContainerResourcesRequest>) -> UpdateContainerResourcesResponse {

        println!("Got a request: {:?}", request);
        let reply = UpdateContainerResourcesResponse {
        };
        return reply;
}