use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ ReopenContainerLogRequest,ReopenContainerLogResponse };

pub fn reopen_container_log(request:Request<ReopenContainerLogRequest>) -> ReopenContainerLogResponse {

        println!("Got a request: {:?}", request);
        let reply = ReopenContainerLogResponse {
        };
        return reply;
}