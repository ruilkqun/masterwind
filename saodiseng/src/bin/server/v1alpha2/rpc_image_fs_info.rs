use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ ImageFsInfoRequest,ImageFsInfoResponse };

pub fn image_fs_info(request:Request<ImageFsInfoRequest>) -> ImageFsInfoResponse {

        println!("Got a request: {:?}", request);
        let mut tmp_items = Vec::new();
        let reply = ImageFsInfoResponse {
            image_filesystems:tmp_items,
        };

        println!("reply:{:?}",reply);
        return reply;
}