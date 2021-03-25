use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ PortForwardRequest,PortForwardResponse };

pub fn port_forward(request:Request<PortForwardRequest>) -> PortForwardResponse {

        println!("Got a request: {:?}", request);
        let reply = PortForwardResponse {
            url:"port forward url".to_string(),
        };
        return reply;
}