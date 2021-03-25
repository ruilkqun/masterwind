use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ StatusRequest,StatusResponse };
use std::collections::HashMap;

pub fn status(request:Request<StatusRequest>) -> StatusResponse {

        println!("Got a request: {:?}", request);
        let mut info_tmp = HashMap::<String,String>::new();
        info_tmp.insert("StatusResponse".to_string(),"status".to_string());
        let reply = StatusResponse {
                status:None,
                info:info_tmp,
        };
        return reply;
}