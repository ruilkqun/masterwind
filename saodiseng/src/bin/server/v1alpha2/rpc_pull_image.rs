use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ PullImageRequest,PullImageResponse };

use crate::impl_pull_image_v1alpha2::pull_image_impl_v1alpha2;

pub async fn pull_image(request:Request<PullImageRequest>) -> PullImageResponse {

        println!("Got a request: {:?}", request);
        // let reply = PullImageResponse {
        //     image_ref:"pull image_ref".to_string(),
        // };

        let reply = pull_image_impl_v1alpha2(request).await;

        println!("reply:{:?}",reply);

        return reply;
}