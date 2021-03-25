use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ ReopenContainerLogRequest,ReopenContainerLogResponse };

pub fn reopen_container_log(request:Request<ReopenContainerLogRequest>) -> ReopenContainerLogResponse {

        println!("Got a request: {:?}", request);
        let reply = ReopenContainerLogResponse {
        };
        return reply;
}