use tonic::Request;

use crate::v1::impl_struct::k8s_v1;
use k8s_v1::{ PullImageRequest,PullImageResponse };

use ant_king_image::cri_server_image_pull::cri_pull_image;
use ant_king_image::local_repositories::get_image_digest_local;

pub async fn pull_image_impl_v1(request:Request<PullImageRequest>) -> PullImageResponse {
        // docker:nginx:latest
        // 192.168.1.118:8899/saodiseng/nginx:latest
        let pull_image_request = request.into_inner();
        let image_tmp1 = pull_image_request.clone().image;
        let auth = pull_image_request.clone().auth;
        let _sandbox_config = pull_image_request.clone().sandbox_config;

        let image_tmp2 = match image_tmp1 {
                Some(res) => res,
                None => {
                        let reply = PullImageResponse {
                                image_ref: "".to_string()
                        };
                        return reply
                }
        };

        let image_analysis1 = image_tmp2.image.split(":");
        let image_analysis2:Vec<&str> = image_analysis1.collect();
        let docker = image_analysis2[0];

        // println!("1");
        // println!("docker:{}",docker);
        return if docker == "docker" {
                let image_name = image_analysis2[1];
                let image_version = image_analysis2[2];

                cri_pull_image("".to_string(), "".to_string(), "".to_string(), image_name.clone().parse().unwrap(), image_version.clone().parse().unwrap(), true).await;
                let image_digest_1 = get_image_digest_local(image_name.clone().parse().unwrap(), image_version.clone().parse().unwrap()).await.unwrap();
                let image_digest = format!("{}@{}",image_name.clone(),image_digest_1.clone());

                let reply = PullImageResponse {
                        image_ref: image_digest.clone()
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

                cri_pull_image(tmp3, username, password, image_name.parse().unwrap(), image_version.parse().unwrap(), true).await;
                let image_digest_1 = get_image_digest_local(image_name.clone().parse().unwrap(), image_version.clone().parse().unwrap()).await.unwrap();
                let image_digest = format!("{}@{}",image_name.clone(),image_digest_1.clone());

                let reply = PullImageResponse {
                        image_ref: image_digest.clone()
                };
                reply
        }
}