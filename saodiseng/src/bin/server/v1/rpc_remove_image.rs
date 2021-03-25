use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ RemoveImageRequest,RemoveImageResponse };

pub fn remove_image(request:Request<RemoveImageRequest>) -> RemoveImageResponse {

        println!("Got a request: {:?}", request);
        let reply = RemoveImageResponse {
        };
        return reply;
}