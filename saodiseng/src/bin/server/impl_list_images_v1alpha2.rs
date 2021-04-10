use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ ListImagesRequest,ListImagesResponse };
use k8s_v1alpha2::{Image,Int64Value,ImageSpec};
use ant_king_image::get_image_list::read_image_list_repositories;


pub async fn list_images_impl_v1alpha2(request:Request<ListImagesRequest>) -> ListImagesResponse {
        let list_images_request = request.into_inner();

        let filter = list_images_request.clone().filter;


        let image_filter = match filter {
                Some(res) => res,
                None => {
                        let reply = ListImagesResponse {
                                images: Vec::new()
                        };
                        return reply
                }
        };


        let image_spec = match image_filter.image {
                Some(res) => res,
                None => {
                        let reply = ListImagesResponse {
                                images: Vec::new()
                        };
                        return reply
                }
        };


        let image = format!("{}",image_spec.image);
        let _annotations = format!("{:?}",image_spec.annotations);

        if image.clone() == "".to_string() {
                let reply_tmp = read_image_list_repositories("".to_string(),"".to_string(),"".to_string()) .await;
                let mut reply_vec:Vec<Image> = Vec::new();

                for i in 0..reply_tmp.len(){
                        let image_tmp = Image {
                                id:reply_tmp[i].id.clone(),
                                repo_tags:reply_tmp[i].repo_tags.clone(),
                                // repo_tags: vec!["nginx:latest".to_string()],
                                repo_digests: reply_tmp[i].repo_digests.clone(),
                                size: reply_tmp[i].size,
                                uid: Some(Int64Value{value:reply_tmp[i].uid}),
                                username: reply_tmp[i].username.clone(),
                                spec: Some(ImageSpec{
                                        image:reply_tmp[i].spec.image_digest.clone(),
                                        annotations:reply_tmp[i].spec.annotations.clone()
                                })
                        };
                        reply_vec.push(image_tmp);
                }

                let reply = ListImagesResponse {
                        images: reply_vec
                };
                return reply
        } else {
                let tmp_1 = image.split(":");
                let tmp_2:Vec<&str> = tmp_1.collect();
                let prefix = format!("{}",tmp_2[0]);
                return if prefix.clone() == "sha256".to_string() {
                        let reply_tmp = read_image_list_repositories("".to_string(), "".to_string(), image.clone()).await;
                        let mut reply_vec: Vec<Image> = Vec::new();

                        for i in 0..reply_tmp.len() {
                                let image_tmp = Image {
                                        id: reply_tmp[i].id.clone(),
                                        repo_tags: reply_tmp[i].repo_tags.clone(),
                                        repo_digests: reply_tmp[i].repo_digests.clone(),
                                        size: reply_tmp[i].size,
                                        uid: Some(Int64Value { value: reply_tmp[i].uid }),
                                        username: reply_tmp[i].username.clone(),
                                        spec: Some(ImageSpec {
                                                image: reply_tmp[i].spec.image_digest.clone(),
                                                annotations: reply_tmp[i].spec.annotations.clone()
                                        })
                                };
                                reply_vec.push(image_tmp);
                        }

                        let reply = ListImagesResponse {
                                images: reply_vec
                        };
                        reply
                } else if prefix == "docker"{
                        let image_name = tmp_2[1];
                        let image_version = tmp_2[tmp_2.len()-1];

                        let tmp1 =  image_name.clone().split("/");
                        let tmp2:Vec<&str> = tmp1.collect();

                        let image_completed_name:String;
                        if tmp2.len() > 1 {
                                image_completed_name = image_name.clone().parse().unwrap();
                        }else {
                                image_completed_name = format!("library/{}", image_name.clone());
                        }

                        let reply_tmp = read_image_list_repositories(image_completed_name.clone(), image_version.clone().parse().unwrap(), "".to_string()).await;
                        let mut reply_vec: Vec<Image> = Vec::new();

                        for i in 0..reply_tmp.len() {
                                let image_tmp = Image {
                                        id: reply_tmp[i].id.clone(),
                                        repo_tags: reply_tmp[i].repo_tags.clone(),
                                        repo_digests: reply_tmp[i].repo_digests.clone(),
                                        size: reply_tmp[i].size,
                                        uid: Some(Int64Value { value: reply_tmp[i].uid }),
                                        username: reply_tmp[i].username.clone(),
                                        spec: Some(ImageSpec {
                                                image: reply_tmp[i].spec.image_digest.clone(),
                                                annotations: reply_tmp[i].spec.annotations.clone()
                                        })
                                };
                                reply_vec.push(image_tmp);
                        }

                        let reply = ListImagesResponse {
                                images: reply_vec
                        };
                        reply
                }else {
                        let image_version = tmp_2[tmp_2.len()-1];
                        let tmp1 = tmp_2[2].split("/");
                        let tmp2:Vec<&str> = tmp1.collect();

                        let mut image_name = "".to_string();
                        for l in 1..tmp2.len() {
                                image_name += &*("/".to_string() + tmp2[l]);
                        }

                        let reply_tmp = read_image_list_repositories(image_name.clone(), image_version.clone().parse().unwrap(), "".to_string()).await;
                        let mut reply_vec: Vec<Image> = Vec::new();

                        for i in 0..reply_tmp.len() {
                                let image_tmp = Image {
                                        id: reply_tmp[i].id.clone(),
                                        repo_tags: reply_tmp[i].repo_tags.clone(),
                                        repo_digests: reply_tmp[i].repo_digests.clone(),
                                        size: reply_tmp[i].size,
                                        uid: Some(Int64Value { value: reply_tmp[i].uid }),
                                        username: reply_tmp[i].username.clone(),
                                        spec: Some(ImageSpec {
                                                image: reply_tmp[i].spec.image_digest.clone(),
                                                annotations: reply_tmp[i].spec.annotations.clone()
                                        })
                                };
                                reply_vec.push(image_tmp);
                        }

                        let reply = ListImagesResponse {
                                images: reply_vec
                        };
                        reply
                }
        }
}