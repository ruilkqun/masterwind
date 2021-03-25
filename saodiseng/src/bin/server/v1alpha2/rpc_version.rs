use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ VersionRequest,VersionResponse };


pub fn version(request:Request<VersionRequest>) -> VersionResponse {

        println!("Got a request: {:?}", request);
        let reply = VersionResponse {
            version:"0.1.0".to_string(),
            runtime_name:"dragonforce".to_string(),
            runtime_version:"1.20.0".to_string(),
            runtime_api_version:"v1".to_string()
        };
        return reply;
}