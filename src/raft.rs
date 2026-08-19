// Election and replication
// We will have leader node and follower nodes.
// Leader node will get query, commit it and send it to follower nodes with index, prev index and term.
// Follower nodes accepts it, commits it and acknolegdes back to leader, then on next query, leader writes that commit to disk and sends the next query to followers
// Followers then commit the new query as well and write the previous one commited to disk
// Each node will have a timestamp or timelimit in which the leader should ping the follower, if it fails, it starts the electiom
// The node becomes a candidate node and each node elects the new leader based on which was has the latest log file.
