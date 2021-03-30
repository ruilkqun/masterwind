use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ PullImageRequest,PullImageResponse };

pub fn pull_image(request:Request<PullImageRequest>) -> PullImageResponse {

        println!("Got a request: {:?}", request);
        let reply = PullImageResponse {
            image_ref:"pull image_ref".to_string(),
        };
        return reply;
}