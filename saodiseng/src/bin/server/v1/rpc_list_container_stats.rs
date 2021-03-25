use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ ListContainerStatsRequest,ListContainerStatsResponse };

pub fn list_container_stats(request:Request<ListContainerStatsRequest>) -> ListContainerStatsResponse {

        println!("Got a request: {:?}", request);
        let mut tmp_items = Vec::new();
        let reply = ListContainerStatsResponse {
            stats: tmp_items
        };
        return reply;
}