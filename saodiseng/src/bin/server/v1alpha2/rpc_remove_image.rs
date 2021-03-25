use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ RemoveImageRequest,RemoveImageResponse };

pub fn remove_image(request:Request<RemoveImageRequest>) -> RemoveImageResponse {

        println!("Got a request: {:?}", request);
        let reply = RemoveImageResponse {
        };
        return reply;
}