use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ ContainerStatusRequest,ContainerStatusResponse };
use std::collections::HashMap;

pub fn container_status(request:Request<ContainerStatusRequest>) -> ContainerStatusResponse {

        println!("Got a request: {:?}", request);
        let mut info_tmp = HashMap::<String,String>::new();
        info_tmp.insert("ContainerStatusResponse".to_string(),"container_status".to_string());
        let reply = ContainerStatusResponse {
            status:None,
            info:info_tmp,
        };
        return reply;
}