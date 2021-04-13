use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ RemoveImageRequest,RemoveImageResponse };

use ant_king_image::cri_server_image_remove::cri_remove_image;
// use futures::{FutureExt, TryFutureExt};

pub async fn remove_image_impl_v1alpha2(request:Request<RemoveImageRequest>) -> RemoveImageResponse {
        //6084105296a9
        let pull_image_request = request.into_inner();
        let image_spec_1 = pull_image_request.clone().image;
        let image_spec = match image_spec_1 {
                Some(res) => res,
                None => {
                        let reply = RemoveImageResponse{};
                        return reply
                }
        };

        let image = format!("{}",image_spec.image);
        let _annotations = format!("{:?}",image_spec.annotations);
        cri_remove_image(image.clone()).await;
        let reply = RemoveImageResponse{};
        return reply
}