use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ ContainerStatsRequest,ContainerStatsResponse };

pub fn container_stats(request:Request<ContainerStatsRequest>) -> ContainerStatsResponse {

        println!("Got a request: {:?}", request);
        let reply = ContainerStatsResponse {
            stats:None,
        };
        return reply;
}