use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ ListContainerStatsRequest,ListContainerStatsResponse };

pub fn list_container_stats(request:Request<ListContainerStatsRequest>) -> ListContainerStatsResponse {

        println!("Got a request: {:?}", request);
        let mut tmp_items = Vec::new();
        let reply = ListContainerStatsResponse {
            stats: tmp_items
        };
        return reply;
}