use tonic::Request;

use crate::v1alpha2::impl_struct::k8s_v1alpha2;
use k8s_v1alpha2::{ ExecSyncRequest,ExecSyncResponse };

pub fn exec_sync(request:Request<ExecSyncRequest>) -> ExecSyncResponse {

        println!("Got a request: {:?}", request);
        let reply = ExecSyncResponse {
            stdout:"exec_sync stdout".as_bytes().to_vec(),
            stderr:"exec_sync stderr".as_bytes().to_vec(),
            exit_code:0
        };
        return reply;
}