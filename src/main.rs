// Build order - WAL -> Election -> Replication -> State machine

mod raft;
mod wal;

fn main() -> std::io::Result<()> {
    // Will start grpc server for this node
    // get the address and initialize clients for remaining nodes
    // Check for exisiting log file
    // Start a node
    Ok(())
}
