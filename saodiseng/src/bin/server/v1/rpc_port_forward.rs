use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ PortForwardRequest,PortForwardResponse };

pub fn port_forward(request:Request<PortForwardRequest>) -> PortForwardResponse {

        println!("Got a request: {:?}", request);
        let reply = PortForwardResponse {
            url:"port forward url".to_string(),
        };
        return reply;
}