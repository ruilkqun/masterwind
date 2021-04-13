use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ ImageStatusRequest,ImageStatusResponse };
use k8s_v1::{ Image,ImageSpec,Int64Value };
use ant_king_image::get_image_status::read_image_status_repositories;
use std::collections::HashMap;



pub async fn images_status_impl_v1(request:Request<ImageStatusRequest>) -> ImageStatusResponse {
        let images_status_request = request.into_inner();
        let image_spec_1 = images_status_request.clone().image;
        let _verbose = images_status_request.clone().verbose;

        let image_spec = match image_spec_1 {
                Some(res) => res,
                None => {
                        // let image_item = Image {
                        //     id:"".to_string(),
                        //     repo_tags:Vec::new(),
                        //     repo_digests: Vec::new(),
                        //     size: 0,
                        //     uid: Some(Int64Value{value:0}),
                        //     username: "".to_string(),
                        //     spec: Some(ImageSpec{
                        //             image:"".to_string(),
                        //             annotations:HashMap::new()
                        //     })
                        // };
                        println!("1");
                        let reply = ImageStatusResponse {
                                image:None,
                                info: HashMap::new()
                        };
                        return reply
                }
        };



        let image = format!("{}",image_spec.image);
        let _annotations = format!("{:?}",image_spec.annotations);


        let reply_tmp = read_image_status_repositories(image.clone()).await;
        let image_item = Image {
            id:reply_tmp.id.clone(),
            repo_tags:reply_tmp.repo_tags.clone(),
            repo_digests: reply_tmp.repo_digests.clone(),
            size: reply_tmp.size,
            uid: Some(Int64Value{value:reply_tmp.uid}),
            username: reply_tmp.username.clone(),
            spec: Some(ImageSpec{
                    image:reply_tmp.spec.image_digest.clone(),
                    annotations:reply_tmp.spec.annotations.clone()
            })
        };
        let reply = ImageStatusResponse {
                image:Some(image_item),
                info: HashMap::new()
        };
        return reply
}