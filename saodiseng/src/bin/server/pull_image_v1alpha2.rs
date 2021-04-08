use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ PullImageRequest,PullImageResponse };

use ant_king_image::cri_server_image_pull::cri_pull_image;
use ant_king_image::local_repositories::get_image_digest_local;
// use futures::{FutureExt, TryFutureExt};

pub async fn pull_image_impl_v1alpha2(request:Request<PullImageRequest>) -> PullImageResponse {
        // docker:nginx:latest
        // docker:ruilkyu/nginx:latest
        // harbor:192.168.1.118:8899/saodiseng/nginx:latest
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

        return if docker == "docker" {
                let image_name = image_analysis2[1];
                let image_version = image_analysis2[2];

                let tmp1 =  image_name.clone().split("/");
                let tmp2:Vec<&str> = tmp1.collect();

                let image_completed_name:String;
                if tmp2.len() > 1 {
                        image_completed_name = image_name.clone().parse().unwrap();
                }else {
                        image_completed_name = format!("library/{}", image_name.clone());
                }

                cri_pull_image("".to_string(), "".to_string(), "".to_string(), image_completed_name.clone().parse().unwrap(), image_version.clone().parse().unwrap(), true).await;
                let image_digest_1 = get_image_digest_local(image_completed_name.clone().parse().unwrap(), image_version.clone().parse().unwrap()).await.unwrap();
                let image_digest = format!("{}@{}",image_completed_name.clone(),image_digest_1.clone());
                println!("image_digest:{}",image_digest.clone());

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
                let username = format!("{}",image_auth.username);
                let password = format!("{}",image_auth.password);

                let port_tmp0 = format!("{}",image_analysis2[image_analysis2.len()-2]);
                let port_tmp1 = port_tmp0.split("/");
                let port_tmp2:Vec<&str> = port_tmp1.collect();

                let port = port_tmp2[0];
                let repositories_url_ip = format!("{}:{}",image_analysis2[1],port);

                let mut image_name_tmp:String = format!("{}",port_tmp2[1]);
                for k in 2..port_tmp2.len(){
                        image_name_tmp += &*("/".to_owned() + port_tmp2[k])
                }
                let image_name = format!("{}",image_name_tmp);
                let image_version = format!("{}",image_analysis2[image_analysis2.len()-1]);

                cri_pull_image(repositories_url_ip.clone(), username.clone(), password.clone(), image_name.clone(), image_version.clone(), false).await;
                let image_digest_1 = get_image_digest_local(image_name.clone().parse().unwrap(), image_version.clone().parse().unwrap()).await.unwrap();
                let image_digest = format!("{}{}@{}",repositories_url_ip.clone(),image_name.clone(),image_digest_1.clone());
                println!("image_digest:{}",image_digest.clone());

                let reply = PullImageResponse {
                        image_ref: image_digest.clone()
                };
                reply
        }
}