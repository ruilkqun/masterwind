use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ RunPodSandboxRequest,RunPodSandboxResponse };
use std::collections::HashMap;
use std::iter::Enumerate;


pub async fn run_pod_sandbox_impl_v1alpha2(request:Request<RunPodSandboxRequest>) -> RunPodSandboxResponse {
        let run_pod_sandbox_request = request.into_inner();
        let pod_sandbox_config_1 = run_pod_sandbox_request.config;
        let pod_sandbox_config = match pod_sandbox_config_1 {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        let runtime_handler = run_pod_sandbox_request.runtime_handler;

        // 获取config
        let pod_sandbox_config = match run_pod_sandbox_request.config {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // 获取config下的metadata
        let pod_sandbox_config_metadata = match pod_sandbox_config.metadata {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        let pod_sandbox_config_metadata_name = pod_sandbox_config_metadata.name;
        let pod_sandbox_config_metadata_uid = pod_sandbox_config_metadata.uid;
        let pod_sandbox_config_metadata_namespace = pod_sandbox_config_metadata.namespace;
        let pod_sandbox_config_metadata_attempt = pod_sandbox_config_metadata.attempt;

        // 获取config下的hostname
        let pod_sandbox_config_hostname = pod_sandbox_config.hostname;

        // 获取config下的log_directory
        let pod_sandbox_config_log_directory = pod_sandbox_config.log_directory;

        // 获取config下的dns_config
        let pod_sandbox_config_dns_config = match pod_sandbox_config.dns_config {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // 集群的DNS服务器列表(Vec)
        let pod_sandbox_config_dns_config_servers:Vec<String> = pod_sandbox_config_dns_config.servers;
        // 集群的DNS搜索阈列表(Vec)
        let pod_sandbox_config_dns_config_searches:Vec<String> = pod_sandbox_config_dns_config.searches;
        // DNS选项列表，具体所有选项参考https://linux.die.net/man/5/resolv.conf(Vec)
        let pod_sandbox_config_dns_config_options:Vec<String> = pod_sandbox_config_dns_config.options;


        // 获取config下的port_mappings(Vec)
        #[derive(Debug)]
        pub enum Protocol{
                TCP,
                UDP,
                SCTP
        }

        #[derive(Debug)]
        pub struct PortMapping{
                // 端口映射协议
                protocol:Protocol,
                // 容器内的端口号。默认值为0(未指定情况下)
                container_port:i32,
                // 主机上的端口号。默认值为0(未指定情况下)
                host_port:i32,
                // 主机IP
                host_ip:String
        }
        let pod_sandbox_config_port_mappings:Vec<Option<PortMapping>> = pod_sandbox_config.port_mappings;

        // 获取config下的labels
        let pod_sandbox_config_labels:HashMap<String,String> = pod_sandbox_config.labels;
        // 获取config下的annotations
        let pod_sandbox_config_annotations = pod_sandbox_config.annotations;

        // 获取config下linux
        let pod_sandbox_config_linux = match pod_sandbox_config.linux {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // PodSandbox的父cgroup
        // 将使用cgroupfs样式的语法，如果需要，但容器运行时可以将其转换为systemd语义
        // 获取 linux下cgroup_parent
        let pod_sandbox_config_linux_cgroup_parent = pod_sandbox_config_linux.cgroup_parent;

        // LinuxSandboxSecurityContext包含沙盒安全属性
        // 获取 linux下security_context
        let pod_sandbox_config_linux_security_context = match pod_sandbox_config_linux.security_context {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // LinuxSandboxSecurityContext应用于沙盒保存linux安全配置
        // 请注意：
        // 不适应pod内的容器
        // 不适用于不包含任何运行进程的PodSandbox

        // 配置沙箱的命名空间
        // 只有当PodSandbox使用名称空间进行隔离时，才会使用此选项
        // 获取 linux下security_context下的namespace_options
        let pod_sandbox_config_linux_security_context_namespace_options = match pod_sandbox_config_linux_security_context.namespace_options {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // NamespaceMode描述了每个名称空间选项中的名称空间（网络、PID、IPC）
        // 运行时应该根据运行时底层的技术映射这些模式
        #[derive(Debug)]
        pub enum NamespaceMode {
            // POD命名空间对于POD中的所有容器都是公用的
            POD,
            // 容器命名空间仅限于单个容器
            CONTAINER,
            // 节点命名空间是Kubernetes节点的命名空间
            NODE,
            // TARGET指向另一个容器的命名空间
            TARGET
        }


        // 容器/沙箱的net命名空间
        // 注意：目前无法在kubernetes api中设置容器范围的net
        // 当前由kubelet负责的命名空间：POD，NODE
        // 获取 linux下security_context下的namespace_options下的network
        let pod_sandbox_config_linux_security_context_namespace_options_network:NamespaceMode = match pod_sandbox_config_linux_security_context_namespace_options.network {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // 容器/沙箱的pid命名空间
        // 注意：CRI的默认值是POD，但是v1.PodSpec的默认值是CONTAINER
        // kubelet的运行时管理器将为v1 pods显式地将其设置为CONTAINER
        // kubelet当前设置的名称空间：POD、CONTAINER、NODE、TARGET
        // 获取 linux下security_context下的namespace_options下的network
        let pod_sandbox_config_linux_security_context_namespace_options_pid:NamespaceMode = match pod_sandbox_config_linux_security_context_namespace_options.pid {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };


        // 容器/沙箱的ipc命名空间
        // 注意：目前无法在kubernetes api中设置容器范围的ipc
        // 当前由kubelet负责的命名空间：POD，NODE
        let pod_sandbox_config_linux_security_context_namespace_options_ipc:NamespaceMode = match pod_sandbox_config_linux_security_context_namespace_options.ipc {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // TARGET的命名空间模式(目标容器ID)
        // 这个容器一定是以前是在同一个pod里创建的。无法指定不同的目标对于每个命名空间
        let pod_sandbox_config_linux_security_context_namespace_options_target_id:NamespaceMode = match pod_sandbox_config_linux_security_context_namespace_options.target_id {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // 要应用的可选SELinux上下文
        // 获取 linux下security_context下的selinux_options
        let  pod_sandbox_config_linux_security_context_selinux_options = match  pod_sandbox_config_linux_security_context.selinux_options{
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // SELinuxOption are the labels to be applied to the container.

        let pod_sandbox_config_linux_security_context_selinux_options_user:String = pod_sandbox_config_linux_security_context_selinux_options.user;
        let pod_sandbox_config_linux_security_context_selinux_options_role:String = pod_sandbox_config_linux_security_context_selinux_options.role;
        let pod_sandbox_config_linux_security_context_selinux_options_type:String = pod_sandbox_config_linux_security_context_selinux_options["type"];
        let pod_sandbox_config_linux_security_context_selinux_options_level:String = pod_sandbox_config_linux_security_context_selinux_options.level;

        // UID to run the container process as. Only one of run_as_user and
        // run_as_username can be specified at a time.
        let pod_sandbox_config_linux_security_context_run_as_user = match pod_sandbox_config_linux_security_context.run_as_user {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        let pod_sandbox_config_linux_security_context_run_as_user_value =  pod_sandbox_config_linux_security_context_run_as_user.value;


        // 当适应的时候，运行沙盒进程的GID
        // 当run_as_user被确定时，run_as_group被指定
        // 否则，运行时必须出错
        let pod_sandbox_config_linux_security_context_run_as_group = match pod_sandbox_config_linux_security_context.run_as_group {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        let pod_sandbox_config_linux_security_context_run_as_group_value =  pod_sandbox_config_linux_security_context_run_as_group.value;


        // 如果设置，沙箱的根文件系统只读
        let pod_sandbox_config_linux_security_context_readonly_rootfs:bool = pod_sandbox_config_linux_security_context.readonly_rootfs;


        // 应用于沙盒中第一个进程运行的组的列表,除了沙箱的主GID
        let pod_sandbox_config_linux_security_context_supplemental_groups:Vec<i64> = pod_sandbox_config_linux_security_context.supplemental_groups;

        //指示是否要求沙盒运行特权容器。如果要在其中执行特权容器，则一定为true
        //这允许沙盒在没有安全措施的情况下采取额外的安全预防措施运行特权容器
        let pod_sandbox_config_linux_security_context_privileged:bool = pod_sandbox_config_linux_security_context.privileged;


        #[derive(Debug)]
        pub enum ProfileType {
                // 容器运行时应使用默认配置文件
                RuntimeDefault,
                // 禁用沙箱或容器的功能
                Unconfined,
                // 应使用节点上的预定义配置文件
                Localhost
        };

        // 沙盒的Seccomp配置文件
        let pod_sandbox_config_linux_security_context_seccomp = match pod_sandbox_config_linux_security_context.seccomp {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // 指示哪种ProfileType应该被应用
        let pod_sandbox_config_linux_security_context_seccomp_profile_type = match pod_sandbox_config_linux_security_context_seccomp.profile_type {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // Indicates that a pre-defined profile on the node should be used.
        // Must only be set if `ProfileType` is `Localhost`.
        // For seccomp, it must be an absolute path to the seccomp profile.
        // For AppArmor, this field is the AppArmor `<profile name>/`
        let pod_sandbox_config_linux_security_context_seccomp_localhost_ref = pod_sandbox_config_linux_security_context_seccomp.localhost_ref;


        // 沙盒的AppArmor配置文件
        let pod_sandbox_config_linux_security_context_apparmor = match pod_sandbox_config_linux_security_context.apparmor {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };

        // 指示哪种ProfileType应该被应用
        let pod_sandbox_config_linux_security_context_apparmor_profile_type = match pod_sandbox_config_linux_security_context_apparmor.profile_type {
                Some(res) => res,
                None => {
                        let reply = RunPodSandboxResponse{
                                pod_sandbox_id: "".to_string()
                        };
                        return reply
                }
        };


        // Indicates that a pre-defined profile on the node should be used.
        // Must only be set if `ProfileType` is `Localhost`.
        // For seccomp, it must be an absolute path to the seccomp profile.
        // For AppArmor, this field is the AppArmor `<profile name>/`
        let pod_sandbox_config_linux_security_context_apparmor_localhost_ref = pod_sandbox_config_linux_security_context_apparmor.localhost_ref;


        // Sysctls为沙箱保存linux Sysctls配置
        // 获取 linux下sysctls
        let pod_sandbox_config_linux_sysctls:HashMap<String,String> = pod_sandbox_config_linux.sysctls;


        let reply = RunPodSandboxResponse{
                pod_sandbox_id: "".to_string()
        };
        return reply
}