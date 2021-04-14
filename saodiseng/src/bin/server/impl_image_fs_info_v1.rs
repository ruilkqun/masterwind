use tonic::Request;
use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ ImageFsInfoRequest,ImageFsInfoResponse };
use k8s_v::{ FilesystemUsage };
use k8s_v1::{ FilesystemIdentifier,UInt64Value };
use ant_king_image::cri_server_image_fs_info::cri_image_fs;



pub async fn images_fs_info_impl_v1(request:Request<ImageFsInfoRequest>) -> ImageFsInfoResponse {
        let computer_result = cri_image_fs().await;
        let timestamp = computer_result.0;

        let fs_id = FilesystemIdentifier{
            mountpoint:computer_result.1
        };

        let used_bytes = UInt64Value {
            value: computer_result.2
        };

        let inodes_used = UInt64Value {
            value: computer_result.3
        };

        let reply_tmp = ImageFsInfoResponse {
            timestamp,
            mountpoint,
            used_bytes,
            inodes_used,
        };

        return reply
}