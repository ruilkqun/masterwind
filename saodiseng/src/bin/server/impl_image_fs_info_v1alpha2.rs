use tonic::Request;
use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ ImageFsInfoRequest,ImageFsInfoResponse };
use k8s_v1alpha2::{ FilesystemUsage };
use k8s_v1alpha2::{ FilesystemIdentifier,UInt64Value };
use ant_king_image::cri_server_image_fs_info::cri_image_fs;



pub async fn images_fs_info_impl_v1alpha2(request:Request<ImageFsInfoRequest>) -> ImageFsInfoResponse {
        let mut result = Vec::new();
        let computer_result = cri_image_fs().await;
        let timestamp = computer_result.0;

        let fs_id = Some(FilesystemIdentifier{
            mountpoint:computer_result.1
        });

        let used_bytes = Some(UInt64Value {
            value: computer_result.2
        });

        let inodes_used = Some(UInt64Value {
            value: computer_result.3
        });

        let reply_item = FilesystemUsage {
            timestamp,
            fs_id,
            used_bytes,
            inodes_used,
        };

        result.push(reply_item);
        let reply =  ImageFsInfoResponse {
            image_filesystems: result
        };
        return reply
}