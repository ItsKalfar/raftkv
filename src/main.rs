// Build order - WAL -> Election -> Replication -> State machine

mod raft;
mod wal;

fn main() -> std::io::Result<()> {
    // Start grpc server for this node
    // Create channel and client for rest of the nodes -> Get node address from config file or something like that

    // If this node is leader -> Receive queries from client
    // Then complete a data lifecyle here -> append -> commit -> on each request

    // If this node is follower -> recive data from leader
    // perform operation of follower on each request

    Ok(())
}
