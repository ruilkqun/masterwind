use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ RemoveImageRequest,RemoveImageResponse };

use crate::impl_remove_image_v1::remove_image_impl_v1;

pub async fn remove_image(request:Request<RemoveImageRequest>) -> RemoveImageResponse {

        println!("Got a request: {:?}", request);
        // let reply = RemoveImageResponse {
        // };
        // return reply;
        let reply = remove_image_impl_v1(request).await;

        println!("reply:{:?}",reply);

        return reply;
}