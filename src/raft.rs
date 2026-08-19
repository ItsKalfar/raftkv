// Election and replication
// We will have leader node and follower nodes.
// Leader node will get query, commit it and send it to follower nodes with index, prev index and term.
// Follower nodes accepts it, commits it and acknolegdes back to leader, then on next query, leader writes that commit to disk and sends the next query to followers
// Followers then commit the new query as well and write the previous one commited to disk
// Each node will have a timestamp or timelimit in which the leader should ping the follower, if it fails, it starts the electiom
// The node becomes a candidate node and each node elects the new leader based on which was has the latest log file.
use std::fs::File;

enum NodeType {
    Candidate,
    Leader,
    Follower,
}

pub struct Node {
    file: File,
    index: u64,
    term: u64,
    timeout: u64,
    node_type: NodeType,
    other_nodes: Vec<Node>,
}

impl Node {
    // WAL
    pub fn append_entires() {
        // If leader -> Append to the log and send single to followers
        // If follower -> Append to the log and return the ack to leader
    }
    pub fn commit_entires() {
        // If leader -> get ack from follower nodes and if the count is (n/2) + 1 -> commit it -> send back the single to followers to commit as well
        // If follower -> get the single from leader for commit and commit in state machine as well
    }

    // WAL Helpers

    fn take_log_snapshot() {
        // Once the log index reaches a certain limit -> Take a snapshot of log file and store it in somewhere -> Glacier, etc. -> Start with a new log file
    }

    fn consistency_check() {
        // If leader -> if the prev index does not match with sent one, get the one back and send it again - repeat till reach the actual prev index of the follower and then send the log from that index to current one
        // If follower -> receive the index, prev index and term, check against current log file and if not matched, return with relavant error
    }

    // Election
    pub fn check_leader_connection() {
        // Check if the heartbeat of leader exceeds the set timeout
        // If yes -> start-election()
        // Check in each function call of append and commit for follower node
    }

    pub fn start_election() {
        // Only if follower -> Leader can't start election
        // Update the Term Count -> Update the node type to Candidate -> Send vote request to other nodes
    }

    pub fn vote_in_election() {
        // Receive voting request -> Update the term count -> Check the node for latest and longest log file -> Vote for that node
    }

    // Election helpers

    fn get_node_with_latest_logs() {
        // Check which node has latest and longest log -> useful in voting function
    }
}
