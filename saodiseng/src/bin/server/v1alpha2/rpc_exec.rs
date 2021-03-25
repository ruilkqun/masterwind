use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ ExecRequest,ExecResponse };

pub fn exec(request:Request<ExecRequest>) -> ExecResponse {

        println!("Got a request: {:?}", request);
        let reply = ExecResponse {
            url:"exec url".to_string(),
        };
        return reply;
}