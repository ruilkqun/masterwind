use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ AttachRequest,AttachResponse };

pub fn attach(request:Request<AttachRequest>) -> AttachResponse {

        println!("Got a request: {:?}", request);
        let reply = AttachResponse {
            url:"attach url".to_string(),
        };
        return reply;
}