use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ RemoveImageRequest,RemoveImageResponse };

use crate::impl_remove_image_v1alpha2::remove_image_impl_v1alpha2;

pub async fn remove_image(request:Request<RemoveImageRequest>) -> RemoveImageResponse {

        println!("Got a request: {:?}", request);
        //
        // let reply = RemoveImageResponse {
        // };
        // return reply;
        let reply = remove_image_impl_v1alpha2(request).await;

        println!("reply:{:?}",reply);

        return reply;
}