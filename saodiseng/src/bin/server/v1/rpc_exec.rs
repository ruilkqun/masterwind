use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ ExecRequest,ExecResponse };

pub fn exec(request:Request<ExecRequest>) -> ExecResponse {

        println!("Got a request: {:?}", request);
        let reply = ExecResponse {
            url:"exec url".to_string(),
        };
        return reply;
}