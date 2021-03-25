use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ UpdateRuntimeConfigRequest,UpdateRuntimeConfigResponse };

pub fn update_runtime_config(request:Request<UpdateRuntimeConfigRequest>) -> UpdateRuntimeConfigResponse {

        println!("Got a request: {:?}", request);
        let reply = UpdateRuntimeConfigResponse {
        };
        return reply;
}