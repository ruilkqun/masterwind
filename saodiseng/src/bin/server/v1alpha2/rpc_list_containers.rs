use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ ListContainersRequest,ListContainersResponse };

pub fn list_containers(request:Request<ListContainersRequest>) -> ListContainersResponse {

        println!("Got a request: {:?}", request);
        let mut tmp_items = Vec::new();
        let reply = ListContainersResponse {
            containers: tmp_items
        };
        return reply;
}