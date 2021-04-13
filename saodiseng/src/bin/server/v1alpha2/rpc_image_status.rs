use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ ImageStatusRequest,ImageStatusResponse };
use crate::impl_image_status_v1alpha2::images_status_impl_v1alpha2;



pub async fn image_status(request:Request<ImageStatusRequest>) -> ImageStatusResponse {

        println!("Got a request: {:?}", request);
        // let mut info_tmp = HashMap::<String,String>::new();
        // info_tmp.insert("ImageStatusResponse".to_string(),"image_status".to_string());
        // let reply = ImageStatusResponse {
        //     image:None,
        //     info:info_tmp,
        // };
        // return reply;
        let reply = images_status_impl_v1alpha2(request).await;

        println!("reply:{:?}",reply);

        return reply;
}