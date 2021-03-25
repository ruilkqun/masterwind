use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ ListContainersRequest,ListContainersResponse };

pub fn list_containers(request:Request<ListContainersRequest>) -> ListContainersResponse {

        println!("Got a request: {:?}", request);
        let mut tmp_items = Vec::new();
        let reply = ListContainersResponse {
            containers: tmp_items
        };
        return reply;
}