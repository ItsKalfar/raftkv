use crate::raft;
use pb::raft_service_server::RaftService;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub mod pb {
    tonic::include_proto!("raft"); // package name from raft.proto
}

pub struct RaftRpc {
    pub node: Mutex<raft::Node>,
}

#[tonic::async_trait]
impl RaftService for RaftRpc {
    async fn append_entries(
        &self,
        req: Request<pb::AppendEntriesRequest>,
    ) -> Result<Response<pb::AppendEntriesResponse>, Status> {
        todo!()
    }

    async fn commit_entries(
        &self,
        req: Request<pb::CommitEntriesRequest>,
    ) -> Result<Response<pb::CommitEntriesResponse>, Status> {
        todo!()
    }
}
