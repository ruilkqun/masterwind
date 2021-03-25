use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ ImageFsInfoRequest,ImageFsInfoResponse };

pub fn image_fs_info(request:Request<ImageFsInfoRequest>) -> ImageFsInfoResponse {

        println!("Got a request: {:?}", request);
        let mut tmp_items = Vec::new();
        let reply = ImageFsInfoResponse {
            image_filesystems:tmp_items,
        };
        return reply;
}