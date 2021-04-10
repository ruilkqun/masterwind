use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ PullImageRequest,PullImageResponse };

use crate::impl_pull_image_v1::pull_image_impl_v1;

pub async fn pull_image(request:Request<PullImageRequest>) -> PullImageResponse {

        println!("Got a request: {:?}", request);
        // let reply = PullImageResponse {
        //     image_ref:"pull image_ref".to_string(),
        // };
        // return reply;

        let reply = pull_image_impl_v1(request).await;

        println!("reply:{:?}",reply);

        return reply;
}