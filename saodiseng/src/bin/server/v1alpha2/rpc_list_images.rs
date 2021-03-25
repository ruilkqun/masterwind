use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ ListImagesRequest,ListImagesResponse };

pub fn list_images(request:Request<ListImagesRequest>) -> ListImagesResponse {

        println!("Got a request: {:?}", request);
        let mut tmp_items = Vec::new();
        let reply = ListImagesResponse {
            images: tmp_items
        };
        return reply;
}