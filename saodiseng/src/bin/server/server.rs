// #[tokio::main]
// async fn main() {
//     println!("hi");
// }



// #![cfg_attr(not(unix), allow(unused_imports))]

// #[macro_use]
extern crate log;
extern crate env_logger;
use env_logger::Env;
use chrono::Local;
use std::io::Write;
// use std::path::Path;
pub mod v1alpha2;
pub mod v1;
pub mod impl_pull_image_v1alpha2;
pub mod impl_pull_image_v1;
pub mod impl_list_images_v1alpha2;
pub mod impl_list_images_v1;
pub mod impl_remove_image_v1alpha2;
pub mod impl_remove_image_v1;
pub mod impl_image_status_v1alpha2;
pub mod impl_image_status_v1;

use futures::TryFutureExt;
#[cfg(unix)]
use tokio::net::UnixListener;
use tonic::{transport::Server, Request, Response, Status};

use v1alpha2::impl_struct::{ MyK8sRuntimeV1alpha2,MyK8sImageV1alpha2 };
use v1alpha2::rpc_version::version as version_v1alpha2;
use v1alpha2::rpc_run_pod_sandbox::run_pod_sandbox as run_pod_sandbox_v1alpha2;
use v1alpha2::rpc_stop_pod_sandbox::stop_pod_sandbox as stop_pod_sandbox_v1alpha2;
use v1alpha2::rpc_remove_pod_sandbox::remove_pod_sandbox as remove_pod_sandbox_v1alpha2;
use v1alpha2::rpc_pod_sandbox_status::pod_sandbox_status as pod_sandbox_status_v1alpha2;
use v1alpha2::rpc_list_pod_sandbox::list_pod_sandbox as list_pod_sandbox_v1alpha2;
use v1alpha2::rpc_create_container::create_container as create_container_v1alpha2;
use v1alpha2::rpc_start_container::start_container as start_container_v1alpha2;
use v1alpha2::rpc_stop_container::stop_container as stop_container_v1alpha2;
use v1alpha2::rpc_remove_container::remove_container as remove_container_v1alpha2;
use v1alpha2::rpc_list_containers::list_containers as list_containers_v1alpha2;
use v1alpha2::rpc_container_status::container_status as container_status_v1alpha2;
use v1alpha2::rpc_update_container_resources::update_container_resources as update_container_resources_v1alpha2;
use v1alpha2::rpc_reopen_container_log::reopen_container_log as reopen_container_log_v1alpha2;
use v1alpha2::rpc_exec_sync::exec_sync as exec_sync_v1alpha2;
use v1alpha2::rpc_exec::exec as exec_v1alpha2;
use v1alpha2::rpc_attach::attach as attach_v1alpha2;
use v1alpha2::rpc_port_forward::port_forward as port_forward_v1alpha2;
use v1alpha2::rpc_container_stats::container_stats as container_stats_v1alpha2;
use v1alpha2::rpc_list_container_stats::list_container_stats as list_container_stats_v1alpha2;
use v1alpha2::rpc_update_runtime_config::update_runtime_config as update_runtime_config_v1alpha2;
use v1alpha2::rpc_status::status as status_v1alpha2;
use v1alpha2::rpc_list_images::list_images as list_images_v1alpha2;
use v1alpha2::rpc_image_status::image_status as image_status_v1alpha2;
use v1alpha2::rpc_pull_image::pull_image as pull_image_v1alpha2;
use v1alpha2::rpc_remove_image::remove_image as remove_image_v1alpha2;
use v1alpha2::rpc_image_fs_info::image_fs_info as image_fs_info_v1alpha2;




use v1::impl_struct::{ MyK8sRuntimeV1,MyK8sImageV1 };
use v1::rpc_version::version as version_v1;
use v1::rpc_run_pod_sandbox::run_pod_sandbox as run_pod_sandbox_v1;
use v1::rpc_stop_pod_sandbox::stop_pod_sandbox as stop_pod_sandbox_v1;
use v1::rpc_remove_pod_sandbox::remove_pod_sandbox as remove_pod_sandbox_v1;
use v1::rpc_pod_sandbox_status::pod_sandbox_status as pod_sandbox_status_v1;
use v1::rpc_list_pod_sandbox::list_pod_sandbox as list_pod_sandbox_v1;
use v1::rpc_create_container::create_container as create_container_v1;
use v1::rpc_start_container::start_container as start_container_v1;
use v1::rpc_stop_container::stop_container as stop_container_v1;
use v1::rpc_remove_container::remove_container as remove_container_v1;
use v1::rpc_list_containers::list_containers as list_containers_v1;
use v1::rpc_container_status::container_status as container_status_v1;
use v1::rpc_update_container_resources::update_container_resources as update_container_resources_v1;
use v1::rpc_reopen_container_log::reopen_container_log as reopen_container_log_v1;
use v1::rpc_exec_sync::exec_sync as exec_sync_v1;
use v1::rpc_exec::exec as exec_v1;
use v1::rpc_attach::attach as attach_v1;
use v1::rpc_port_forward::port_forward as port_forward_v1;
use v1::rpc_container_stats::container_stats as container_stats_v1;
use v1::rpc_list_container_stats::list_container_stats as list_container_stats_v1;
use v1::rpc_update_runtime_config::update_runtime_config as update_runtime_config_v1;
use v1::rpc_status::status as status_v1;
use v1::rpc_list_images::list_images as list_images_v1;
use v1::rpc_image_status::image_status as image_status_v1;
use v1::rpc_pull_image::pull_image as pull_image_v1;
use v1::rpc_remove_image::remove_image as remove_image_v1;
use v1::rpc_image_fs_info::image_fs_info as image_fs_info_v1;




use v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::runtime_service_server::{ RuntimeService as RuntimeServiceV1alpha2,RuntimeServiceServer as RuntimeServiceServerV1alpha2 };
use k8s_v1alpha2::{ VersionRequest as VersionRequestV1alpha2,VersionResponse as VersionResponseV1alpha2 };
use k8s_v1alpha2::{ RunPodSandboxRequest as RunPodSandboxRequestV1alpha2,RunPodSandboxResponse as RunPodSandboxResponseV1alpha2 };
use k8s_v1alpha2::{ StopPodSandboxRequest as StopPodSandboxRequestV1alpha2,StopPodSandboxResponse as StopPodSandboxResponseV1alpha2 };
use k8s_v1alpha2::{ RemovePodSandboxRequest as RemovePodSandboxRequestV1alpha2,RemovePodSandboxResponse as RemovePodSandboxResponseV1alpha2 };
use k8s_v1alpha2::{ PodSandboxStatusRequest as PodSandboxStatusRequestV1alpha2,PodSandboxStatusResponse as PodSandboxStatusResponseV1alpha2 };
use k8s_v1alpha2::{ ListPodSandboxRequest as ListPodSandboxRequestV1alpha2,ListPodSandboxResponse as ListPodSandboxResponseV1alpha2 };
use k8s_v1alpha2::{ CreateContainerRequest as CreateContainerRequestV1alpha2,CreateContainerResponse as CreateContainerResponseV1alpha2 };
use k8s_v1alpha2::{ StartContainerRequest as StartContainerRequestV1alpha2,StartContainerResponse as StartContainerResponseV1alpha2 };
use k8s_v1alpha2::{ StopContainerRequest as StopContainerRequestV1alpha2,StopContainerResponse as StopContainerResponseV1alpha2 };
use k8s_v1alpha2::{ RemoveContainerRequest as RemoveContainerRequestV1alpha2,RemoveContainerResponse as RemoveContainerResponseV1alpha2 };
use k8s_v1alpha2::{ ListContainersRequest as ListContainersRequestV1alpha2,ListContainersResponse as ListContainersResponseV1alpha2 };
use k8s_v1alpha2::{ ContainerStatusRequest as ContainerStatusRequestV1alpha2,ContainerStatusResponse as ContainerStatusResponseV1alpha2 };
use k8s_v1alpha2::{ UpdateContainerResourcesRequest as UpdateContainerResourcesRequestV1alpha2,UpdateContainerResourcesResponse as UpdateContainerResourcesResponseV1alpha2 };
use k8s_v1alpha2::{ ReopenContainerLogRequest as ReopenContainerLogRequestV1alpha2,ReopenContainerLogResponse as ReopenContainerLogResponseV1alpha2 };
use k8s_v1alpha2::{ ExecSyncRequest as ExecSyncRequestV1alpha2,ExecSyncResponse as ExecSyncResponseV1alpha2 };
use k8s_v1alpha2::{ ExecRequest as ExecRequestV1alpha2,ExecResponse as ExecResponseV1alpha2 };
use k8s_v1alpha2::{ AttachRequest as AttachRequestV1alpha2,AttachResponse as AttachResponseV1alpha2 };
use k8s_v1alpha2::{ PortForwardRequest as PortForwardRequestV1alpha2,PortForwardResponse as PortForwardResponseV1alpha2 };
use k8s_v1alpha2::{ ContainerStatsRequest as ContainerStatsRequestV1alpha2,ContainerStatsResponse as ContainerStatsResponseV1alpha2 };
use k8s_v1alpha2::{ ListContainerStatsRequest as ListContainerStatsRequestV1alpha2,ListContainerStatsResponse as ListContainerStatsResponseV1alpha2 };
use k8s_v1alpha2::{ UpdateRuntimeConfigRequest as UpdateRuntimeConfigRequestV1alpha2,UpdateRuntimeConfigResponse as UpdateRuntimeConfigResponseV1alpha2 };
use k8s_v1alpha2::{ StatusRequest as StatusRequestV1alpha2,StatusResponse as StatusResponseV1alpha2 };

use k8s_v1alpha2::image_service_server::{ ImageService as ImageServiceV1alpha2,ImageServiceServer as ImageServiceServerV1alpha2 };
use k8s_v1alpha2::{ ListImagesRequest as ListImagesRequestV1alpha2,ListImagesResponse as ListImagesResponseV1alpha2 };
use k8s_v1alpha2::{ ImageStatusRequest as ImageStatusRequestV1alpha2,ImageStatusResponse as ImageStatusResponseV1alpha2 };
use k8s_v1alpha2::{ PullImageRequest as PullImageRequestV1alpha2,PullImageResponse as PullImageResponseV1alpha2 };
use k8s_v1alpha2::{ RemoveImageRequest as RemoveImageRequestV1alpha2,RemoveImageResponse as RemoveImageResponseV1alpha2 };
use k8s_v1alpha2::{ ImageFsInfoRequest as ImageFsInfoRequestV1alpha2,ImageFsInfoResponse as ImageFsInfoResponseV1alpha2 };



use v1::impl_struct::k8s_v1;
use k8s_v1::runtime_service_server::{ RuntimeService as RuntimeServiceV1,RuntimeServiceServer as RuntimeServiceServerV1 };
use k8s_v1::{ VersionRequest as VersionRequestV1,VersionResponse as VersionResponseV1 };
use k8s_v1::{ RunPodSandboxRequest as RunPodSandboxRequestV1,RunPodSandboxResponse as RunPodSandboxResponseV1 };
use k8s_v1::{ StopPodSandboxRequest as StopPodSandboxRequestV1,StopPodSandboxResponse as StopPodSandboxResponseV1 };
use k8s_v1::{ RemovePodSandboxRequest as RemovePodSandboxRequestV1,RemovePodSandboxResponse as RemovePodSandboxResponseV1 };
use k8s_v1::{ PodSandboxStatusRequest as PodSandboxStatusRequestV1,PodSandboxStatusResponse as PodSandboxStatusResponseV1 };
use k8s_v1::{ ListPodSandboxRequest as ListPodSandboxRequestV1,ListPodSandboxResponse as ListPodSandboxResponseV1 };
use k8s_v1::{ CreateContainerRequest as CreateContainerRequestV1,CreateContainerResponse as CreateContainerResponseV1 };
use k8s_v1::{ StartContainerRequest as StartContainerRequestV1,StartContainerResponse as StartContainerResponseV1 };
use k8s_v1::{ StopContainerRequest as StopContainerRequestV1,StopContainerResponse as StopContainerResponseV1 };
use k8s_v1::{ RemoveContainerRequest as RemoveContainerRequestV1,RemoveContainerResponse as RemoveContainerResponseV1 };
use k8s_v1::{ ListContainersRequest as ListContainersRequestV1,ListContainersResponse as ListContainersResponseV1 };
use k8s_v1::{ ContainerStatusRequest as ContainerStatusRequestV1,ContainerStatusResponse as ContainerStatusResponseV1 };
use k8s_v1::{ UpdateContainerResourcesRequest as UpdateContainerResourcesRequestV1,UpdateContainerResourcesResponse as UpdateContainerResourcesResponseV1 };
use k8s_v1::{ ReopenContainerLogRequest as ReopenContainerLogRequestV1,ReopenContainerLogResponse as ReopenContainerLogResponseV1 };
use k8s_v1::{ ExecSyncRequest as ExecSyncRequestV1,ExecSyncResponse as ExecSyncResponseV1 };
use k8s_v1::{ ExecRequest as ExecRequestV1,ExecResponse as ExecResponseV1 };
use k8s_v1::{ AttachRequest as AttachRequestV1,AttachResponse as AttachResponseV1 };
use k8s_v1::{ PortForwardRequest as PortForwardRequestV1,PortForwardResponse as PortForwardResponseV1 };
use k8s_v1::{ ContainerStatsRequest as ContainerStatsRequestV1,ContainerStatsResponse as ContainerStatsResponseV1 };
use k8s_v1::{ ListContainerStatsRequest as ListContainerStatsRequestV1,ListContainerStatsResponse as ListContainerStatsResponseV1 };
use k8s_v1::{ UpdateRuntimeConfigRequest as UpdateRuntimeConfigRequestV1,UpdateRuntimeConfigResponse as UpdateRuntimeConfigResponseV1 };
use k8s_v1::{ StatusRequest as StatusRequestV1,StatusResponse as StatusResponseV1 };

use k8s_v1::image_service_server::{ ImageService as ImageServiceV1,ImageServiceServer as ImageServiceServerV1 };
use k8s_v1::{ ListImagesRequest as ListImagesRequestV1,ListImagesResponse as ListImagesResponseV1 };
use k8s_v1::{ ImageStatusRequest as ImageStatusRequestV1,ImageStatusResponse as ImageStatusResponseV1 };
use k8s_v1::{ PullImageRequest as PullImageRequestV1,PullImageResponse as PullImageResponseV1 };
use k8s_v1::{ RemoveImageRequest as RemoveImageRequestV1,RemoveImageResponse as RemoveImageResponseV1 };
use k8s_v1::{ ImageFsInfoRequest as ImageFsInfoRequestV1,ImageFsInfoResponse as ImageFsInfoResponseV1 };



#[tonic::async_trait]
impl RuntimeServiceV1alpha2 for MyK8sRuntimeV1alpha2 {
    async fn version(&self,request:Request<VersionRequestV1alpha2>) -> Result<Response<VersionResponseV1alpha2>, Status> {
        Ok(Response::new(version_v1alpha2(request)))
    }

    async fn run_pod_sandbox(&self,request:Request<RunPodSandboxRequestV1alpha2>) -> Result<Response<RunPodSandboxResponseV1alpha2>, Status> {
        Ok(Response::new(run_pod_sandbox_v1alpha2(request)))
    }

    async fn stop_pod_sandbox(&self,request:Request<StopPodSandboxRequestV1alpha2>) -> Result<Response<StopPodSandboxResponseV1alpha2>, Status> {
        Ok(Response::new(stop_pod_sandbox_v1alpha2(request)))
    }

    async fn remove_pod_sandbox(&self,request:Request<RemovePodSandboxRequestV1alpha2>) -> Result<Response<RemovePodSandboxResponseV1alpha2>, Status> {
        Ok(Response::new(remove_pod_sandbox_v1alpha2(request)))
    }

    async fn pod_sandbox_status(&self,request:Request<PodSandboxStatusRequestV1alpha2>) -> Result<Response<PodSandboxStatusResponseV1alpha2>, Status> {
        Ok(Response::new(pod_sandbox_status_v1alpha2(request)))
    }

    async fn list_pod_sandbox(&self,request:Request<ListPodSandboxRequestV1alpha2>) -> Result<Response<ListPodSandboxResponseV1alpha2>, Status> {
        Ok(Response::new(list_pod_sandbox_v1alpha2(request)))
    }

    async fn create_container(&self,request:Request<CreateContainerRequestV1alpha2>) -> Result<Response<CreateContainerResponseV1alpha2>, Status> {
        Ok(Response::new(create_container_v1alpha2(request)))
    }

    async fn start_container(&self,request:Request<StartContainerRequestV1alpha2>) -> Result<Response<StartContainerResponseV1alpha2>, Status> {
        Ok(Response::new(start_container_v1alpha2(request)))
    }

    async fn stop_container(&self,request:Request<StopContainerRequestV1alpha2>) -> Result<Response<StopContainerResponseV1alpha2>, Status> {
        Ok(Response::new(stop_container_v1alpha2(request)))
    }

    async fn remove_container(&self,request:Request<RemoveContainerRequestV1alpha2>) -> Result<Response<RemoveContainerResponseV1alpha2>, Status> {
        Ok(Response::new(remove_container_v1alpha2(request)))
    }

    async fn list_containers(&self,request:Request<ListContainersRequestV1alpha2>) -> Result<Response<ListContainersResponseV1alpha2>, Status> {
        Ok(Response::new(list_containers_v1alpha2(request)))
    }

    async fn container_status(&self,request:Request<ContainerStatusRequestV1alpha2>) -> Result<Response<ContainerStatusResponseV1alpha2>, Status> {
        Ok(Response::new(container_status_v1alpha2(request)))
    }

    async fn update_container_resources(&self,request:Request<UpdateContainerResourcesRequestV1alpha2>) -> Result<Response<UpdateContainerResourcesResponseV1alpha2>, Status> {
        Ok(Response::new(update_container_resources_v1alpha2(request)))
    }

    async fn reopen_container_log(&self,request:Request<ReopenContainerLogRequestV1alpha2>) -> Result<Response<ReopenContainerLogResponseV1alpha2>, Status> {
        Ok(Response::new(reopen_container_log_v1alpha2(request)))
    }

    async fn exec_sync(&self,request:Request<ExecSyncRequestV1alpha2>) -> Result<Response<ExecSyncResponseV1alpha2>, Status> {
        Ok(Response::new(exec_sync_v1alpha2(request)))
    }

    async fn exec(&self,request:Request<ExecRequestV1alpha2>) -> Result<Response<ExecResponseV1alpha2>, Status> {
        Ok(Response::new(exec_v1alpha2(request)))
    }

    async fn attach(&self,request:Request<AttachRequestV1alpha2>) -> Result<Response<AttachResponseV1alpha2>, Status> {
        Ok(Response::new(attach_v1alpha2(request)))
    }

    async fn port_forward(&self,request:Request<PortForwardRequestV1alpha2>) -> Result<Response<PortForwardResponseV1alpha2>, Status> {
        Ok(Response::new(port_forward_v1alpha2(request)))
    }

    async fn container_stats(&self,request:Request<ContainerStatsRequestV1alpha2>) -> Result<Response<ContainerStatsResponseV1alpha2>, Status> {
        Ok(Response::new(container_stats_v1alpha2(request)))
    }

    async fn list_container_stats(&self,request:Request<ListContainerStatsRequestV1alpha2>) -> Result<Response<ListContainerStatsResponseV1alpha2>, Status> {
        Ok(Response::new(list_container_stats_v1alpha2(request)))
    }

    async fn update_runtime_config(&self,request:Request<UpdateRuntimeConfigRequestV1alpha2>) -> Result<Response<UpdateRuntimeConfigResponseV1alpha2>, Status> {
        Ok(Response::new(update_runtime_config_v1alpha2(request)))
    }

    async fn status(&self,request:Request<StatusRequestV1alpha2>) -> Result<Response<StatusResponseV1alpha2>, Status> {
        Ok(Response::new(status_v1alpha2(request)))
    }
}

#[tonic::async_trait]
impl ImageServiceV1alpha2 for MyK8sImageV1alpha2 {
    async fn list_images(&self,request:Request<ListImagesRequestV1alpha2>) -> Result<Response<ListImagesResponseV1alpha2>, Status> {
        Ok(Response::new(list_images_v1alpha2(request).await))
    }

    async fn image_status(&self,request:Request<ImageStatusRequestV1alpha2>) -> Result<Response<ImageStatusResponseV1alpha2>, Status> {
        Ok(Response::new(image_status_v1alpha2(request).await))
    }

    async fn pull_image(&self,request:Request<PullImageRequestV1alpha2>) -> Result<Response<PullImageResponseV1alpha2>, Status> {
        Ok(Response::new(pull_image_v1alpha2(request).await))
    }

    async fn remove_image(&self,request:Request<RemoveImageRequestV1alpha2>) -> Result<Response<RemoveImageResponseV1alpha2>, Status> {
        Ok(Response::new(remove_image_v1alpha2(request).await))
    }

    async fn image_fs_info(&self,request:Request<ImageFsInfoRequestV1alpha2>) -> Result<Response<ImageFsInfoResponseV1alpha2>, Status> {
        Ok(Response::new(image_fs_info_v1alpha2(request)))
    }
}




#[tonic::async_trait]
impl RuntimeServiceV1 for MyK8sRuntimeV1 {
    async fn version(&self,request:Request<VersionRequestV1>) -> Result<Response<VersionResponseV1>, Status> {
        Ok(Response::new(version_v1(request)))
    }

    async fn run_pod_sandbox(&self,request:Request<RunPodSandboxRequestV1>) -> Result<Response<RunPodSandboxResponseV1>, Status> {
        Ok(Response::new(run_pod_sandbox_v1(request)))
    }

    async fn stop_pod_sandbox(&self,request:Request<StopPodSandboxRequestV1>) -> Result<Response<StopPodSandboxResponseV1>, Status> {
        Ok(Response::new(stop_pod_sandbox_v1(request)))
    }

    async fn remove_pod_sandbox(&self,request:Request<RemovePodSandboxRequestV1>) -> Result<Response<RemovePodSandboxResponseV1>, Status> {
        Ok(Response::new(remove_pod_sandbox_v1(request)))
    }

    async fn pod_sandbox_status(&self,request:Request<PodSandboxStatusRequestV1>) -> Result<Response<PodSandboxStatusResponseV1>, Status> {
        Ok(Response::new(pod_sandbox_status_v1(request)))
    }

    async fn list_pod_sandbox(&self,request:Request<ListPodSandboxRequestV1>) -> Result<Response<ListPodSandboxResponseV1>, Status> {
        Ok(Response::new(list_pod_sandbox_v1(request)))
    }

    async fn create_container(&self,request:Request<CreateContainerRequestV1>) -> Result<Response<CreateContainerResponseV1>, Status> {
        Ok(Response::new(create_container_v1(request)))
    }

    async fn start_container(&self,request:Request<StartContainerRequestV1>) -> Result<Response<StartContainerResponseV1>, Status> {
        Ok(Response::new(start_container_v1(request)))
    }

    async fn stop_container(&self,request:Request<StopContainerRequestV1>) -> Result<Response<StopContainerResponseV1>, Status> {
        Ok(Response::new(stop_container_v1(request)))
    }

    async fn remove_container(&self,request:Request<RemoveContainerRequestV1>) -> Result<Response<RemoveContainerResponseV1>, Status> {
        Ok(Response::new(remove_container_v1(request)))
    }

    async fn list_containers(&self,request:Request<ListContainersRequestV1>) -> Result<Response<ListContainersResponseV1>, Status> {
        Ok(Response::new(list_containers_v1(request)))
    }

    async fn container_status(&self,request:Request<ContainerStatusRequestV1>) -> Result<Response<ContainerStatusResponseV1>, Status> {
        Ok(Response::new(container_status_v1(request)))
    }

    async fn update_container_resources(&self,request:Request<UpdateContainerResourcesRequestV1>) -> Result<Response<UpdateContainerResourcesResponseV1>, Status> {
        Ok(Response::new(update_container_resources_v1(request)))
    }

    async fn reopen_container_log(&self,request:Request<ReopenContainerLogRequestV1>) -> Result<Response<ReopenContainerLogResponseV1>, Status> {
        Ok(Response::new(reopen_container_log_v1(request)))
    }

    async fn exec_sync(&self,request:Request<ExecSyncRequestV1>) -> Result<Response<ExecSyncResponseV1>, Status> {
        Ok(Response::new(exec_sync_v1(request)))
    }

    async fn exec(&self,request:Request<ExecRequestV1>) -> Result<Response<ExecResponseV1>, Status> {
        Ok(Response::new(exec_v1(request)))
    }

    async fn attach(&self,request:Request<AttachRequestV1>) -> Result<Response<AttachResponseV1>, Status> {
        Ok(Response::new(attach_v1(request)))
    }

    async fn port_forward(&self,request:Request<PortForwardRequestV1>) -> Result<Response<PortForwardResponseV1>, Status> {
        Ok(Response::new(port_forward_v1(request)))
    }

    async fn container_stats(&self,request:Request<ContainerStatsRequestV1>) -> Result<Response<ContainerStatsResponseV1>, Status> {
        Ok(Response::new(container_stats_v1(request)))
    }

    async fn list_container_stats(&self,request:Request<ListContainerStatsRequestV1>) -> Result<Response<ListContainerStatsResponseV1>, Status> {
        Ok(Response::new(list_container_stats_v1(request)))
    }

    async fn update_runtime_config(&self,request:Request<UpdateRuntimeConfigRequestV1>) -> Result<Response<UpdateRuntimeConfigResponseV1>, Status> {
        Ok(Response::new(update_runtime_config_v1(request)))
    }

    async fn status(&self,request:Request<StatusRequestV1>) -> Result<Response<StatusResponseV1>, Status> {
        Ok(Response::new(status_v1(request)))
    }
}

#[tonic::async_trait]
impl ImageServiceV1 for MyK8sImageV1 {
    async fn list_images(&self,request:Request<ListImagesRequestV1>) -> Result<Response<ListImagesResponseV1>, Status> {
        Ok(Response::new(list_images_v1(request).await))
    }

    async fn image_status(&self,request:Request<ImageStatusRequestV1>) -> Result<Response<ImageStatusResponseV1>, Status> {
        Ok(Response::new(image_status_v1(request).await))
    }

    async fn pull_image(&self,request:Request<PullImageRequestV1>) -> Result<Response<PullImageResponseV1>, Status> {
        Ok(Response::new(pull_image_v1(request).await))
    }

    async fn remove_image(&self,request:Request<RemoveImageRequestV1>) -> Result<Response<RemoveImageResponseV1>, Status> {
        Ok(Response::new(remove_image_v1(request).await))
    }

    async fn image_fs_info(&self,request:Request<ImageFsInfoRequestV1>) -> Result<Response<ImageFsInfoResponseV1>, Status> {
        Ok(Response::new(image_fs_info_v1(request)))
    }
}




#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hi");
    let env = Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{:<5} {} [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                &record.args()
            )
        })
        .init();


    // println!("hi");
    // info!("Hi,This is DragonForce!");


    let path = "/var/run/saodiseng.sock";

    // tokio::fs::create_dir_all(Path::new(path).parent().unwrap()).await?;
    std::fs::remove_file(path).unwrap();
    let my_k8s_runtime_v1alpha2 = MyK8sRuntimeV1alpha2::default();
    let my_k8s_image_v1alpha2 = MyK8sImageV1alpha2::default();
    let my_k8s_runtime_v1 = MyK8sRuntimeV1::default();
    let my_k8s_image_v1 = MyK8sImageV1::default();

    let incoming = {
        let listener = UnixListener::bind(path)?;

        async_stream::stream! {
            while let item = listener.accept().map_ok(|(st, _)| unix::UnixStream(st)).await {
                yield item;
            }
        }
    };

    Server::builder()
        .add_service(RuntimeServiceServerV1alpha2::new(my_k8s_runtime_v1alpha2))
        .add_service(ImageServiceServerV1alpha2::new(my_k8s_image_v1alpha2))
        .add_service(RuntimeServiceServerV1::new(my_k8s_runtime_v1))
        .add_service(ImageServiceServerV1::new(my_k8s_image_v1))
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}


#[cfg(unix)]
mod unix {
    use std::{
        pin::Pin,
        task::{Context, Poll},
    };

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
    use tonic::transport::server::Connected;

    #[derive(Debug)]
    pub struct UnixStream(pub tokio::net::UnixStream);

    impl Connected for UnixStream {}

    impl AsyncRead for UnixStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for UnixStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.0).poll_write(cx, buf)
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_shutdown(cx)
        }
    }
}

#[cfg(not(unix))]
fn main() {
    panic!("The `Dragonforce client` only works on unix systems!");
}