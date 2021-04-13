use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ ImageStatusRequest,ImageStatusResponse };
use crate::impl_image_status_v1::images_status_impl_v1;


pub async fn image_status(request:Request<ImageStatusRequest>) -> ImageStatusResponse {

        println!("Got a request: {:?}", request);
        // let mut info_tmp = HashMap::<String,String>::new();
        // info_tmp.insert("ImageStatusResponse".to_string(),"image_status".to_string());
        // let reply = ImageStatusResponse {
        //     image:None,
        //     info:info_tmp,
        // };
        // return reply;
        let reply = images_status_impl_v1(request).await;

        println!("reply:{:?}",reply);

        return reply;
}