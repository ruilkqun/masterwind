use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ ImageFsInfoRequest,ImageFsInfoResponse };
use crate::impl_image_fs_info_v1::images_fs_info_impl_v1;

pub async fn image_fs_info(request:Request<ImageFsInfoRequest>) -> ImageFsInfoResponse {

        println!("Got a request: {:?}", request);
        // let mut tmp_items = Vec::new();
        // let reply = ImageFsInfoResponse {
        //     image_filesystems:tmp_items,
        // };
        //
        // println!("reply:{:?}",reply);
        // return reply;

        let reply = images_fs_info_impl_v1(request).await;

        println!("reply:{:?}",reply);

        return reply;
}