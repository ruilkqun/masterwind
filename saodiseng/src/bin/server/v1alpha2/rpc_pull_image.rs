use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ PullImageRequest,PullImageResponse };
use crate::pull_image::pull_image_impl;

use std::error::Error;
use futures::future::Future;
use futures::future::ok;
use tokio_core::reactor::Core;


// pub async fn pull_image(request:Request<PullImageRequest>) -> impl Future<Item=PullImageResponse,Error=Box<dyn Error + 'static>> {
//         //
//         // println!("Got a request: {:?}", request);
//         // let reply = PullImageResponse {
//         //     image_ref:"pull image_ref".to_string(),
//         // };
//         let mut reactor = Core::new().unwrap();
//         let reply_future = pull_image_impl(request).await;
//
//         let reply = reactor.run(reply_future).unwrap();
//         return ok(reply);
// }


pub fn pull_image(request:Request<PullImageRequest>) -> PullImageResponse {
        //
        println!("Got a request: {:?}", request);
        let reply = PullImageResponse {
            image_ref:"pull image_ref".to_string(),
        };
        // let mut reactor = Core::new().unwrap();
        // let reply = pull_image_impl(request).await;

        // let reply = reactor.run(reply_future).unwrap();
        return reply;
}