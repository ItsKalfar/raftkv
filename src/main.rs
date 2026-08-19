use grpc::{RaftRpc, pb::raft_service_server::RaftServiceServer};

mod grpc;
mod raft;
mod wal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic::transport::Server::builder()
        .add_service(RaftServiceServer::new(RaftRpc { node: todo!() }))
        .serve("127.0.0.1:50051".parse()?)
        .await?;
    Ok(())
}
