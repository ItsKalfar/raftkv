// Election and Replication

// Data Flow -> Leader get's the query -> appends the entry into it's log (fsync in log file as well) and then send it to the follower ->
// Follower receives the signal -> appends the entry into it's log (fsync in the log file as well) and return the ack to leader ->
// Once enough nodes ack, leader then commits the query i.e. updates the commit index (so the system knows that this much data is okay to go in state machine) ->
// Leader then sends the signal to followers to commit as well -> followers follow the same process and commit the query (commit_index++) ->
// State machine then checks the commit index and applies the chnages to database and updates the last applied

// Election -> For Some case (leader timeout or crashes) -> follower node becomes candidate -> starts election -> requests for vote -> everybody checks for node with latest and longest log ->
// Vote accordinly -> Candidate becomes the leader and starts accepting requests

use std::fs::File;

enum NodeType {
    Candidate,
    Leader,
    Follower,
}

struct LogEntry {
    index: u64,
    message: String,
}

pub struct Node {
    wal_file: File,
    log: Vec<LogEntry>, // a copy of actual fsync file for faster reads -> writes speed is same
    commit_index: u64,  // Till what the index is commited -> Safe to apply in state machine
    last_applied: u64,
    current_term: u64,
    timeout: u64, // random milliseconds
    node_type: NodeType,
    other_nodes: Vec<Node>,
}

impl Node {
    // Create new node
    pub fn new() {}
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

    fn get_all_pending_logs() {
        // helper to get the remaining logs for delayed or late node with unaligned prev index
    }

    // Election
    pub fn check_leader_connection() {
        // Reset the timeout to it's original state on every request -> if timeout reaches to 0 -> start an election
        // start-election()
        // Check in each function call of append and commit for follower node
    }

    pub fn start_election() {
        // Only for follower -> Leader can't start election
        // Update the Term Count -> Update the node type to Candidate -> Send vote request to other nodes
    }

    pub fn vote_in_election() {
        // Receive voting request -> Update the term count -> Check the node for latest and longest log file -> Vote for that node
    }

    // Election helpers

    fn get_node_with_latest_logs() {
        // Check which node has latest and longest log -> useful in voting function
    }

    // State machine -> Will operate on last_applied, commit index, and log file on it's own
}
