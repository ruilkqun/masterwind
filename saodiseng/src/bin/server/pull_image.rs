use tonic::{Request};

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ PullImageRequest,PullImageResponse };
use ant_king_image::cri_server_image_pull::cri_pull_image;

use std::error::Error;
use futures::future::Future;
use futures::future::ok;
use tokio_core::reactor::Core;

pub async fn pull_image_impl(request:Request<PullImageRequest>) -> PullImageResponse {
        // println!("Got a request: {:?}", request);
        let mut reactor = Core::new().unwrap();
        // docker:nginx:latest
        // 192.168.1.118:8899/saodiseng/nginx:latest
        let pull_image_request = request.into_inner();
        let image_tmp1 = pull_image_request.clone().image;
        let auth = pull_image_request.clone().auth;
        let sandbox_config = pull_image_request.clone().sandbox_config;

        let image_tmp2 = match image_tmp1 {
                Some(res) => res,
                None => {
                        let reply = PullImageResponse {
                                image_ref: "".to_string()
                        };
                        return reply
                }
        };

        let image_analysis1 = image_tmp2.image.split(";");
        let image_analysis2:Vec<&str> = image_analysis1.collect();
        let docker = image_analysis2[0];

        return if docker == "docker" {
                let image_name = image_analysis2[1];
                let image_version = image_analysis2[2];

                // let image_digest = reactor.run(cri_pull_image("".to_string(), "".to_string(), "".to_string(), image_name.parse().unwrap(), image_version.parse().unwrap(), true)).unwrap();

                let image_digest_future = cri_pull_image("".to_string(), "".to_string(), "".to_string(), image_name.parse().unwrap(), image_version.parse().unwrap(), true).await;
                let image_digest = reactor.run(image_digest_future).unwrap();

                let reply = PullImageResponse {
                        image_ref: image_digest
                };
                reply
        } else {

                let image_auth = match auth {
                        Some(res) => res,
                        None => {
                                let reply = PullImageResponse {
                                        image_ref: "".to_string()
                                };
                                return reply
                        }
                };
                let username = image_auth.username;
                let password = image_auth.password;
                let image_version = image_analysis2[2];
                let tmp1 = image_analysis2[1].split("/");
                let tmp2:Vec<&str> = tmp1.collect();
                let tmp3 = format!("{}:{}",image_analysis2[0],tmp2[0]);
                let mut tmp4 = "".to_string();
                for l in 1..tmp2.len() {
                        tmp4 += &*("/".to_string() + tmp2[l]);
                }
                let image_name = format!("{}",tmp4);

                // let image_digest = reactor.run(cri_pull_image(tmp3, username, password, image_name.parse().unwrap(), image_version.parse().unwrap(), true)).unwrap();

                let image_digest_future = cri_pull_image(tmp3, username, password, image_name.parse().unwrap(), image_version.parse().unwrap(), true).await;
                let image_digest = reactor.run(image_digest_future).unwrap();

                let reply = PullImageResponse {
                        image_ref: image_digest
                };
                reply
        }
}