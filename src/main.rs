// Build order - WAL -> Election -> Replication -> State machine

mod wal;

use wal::Wal;

fn main() {
    let mut wal = Wal::new("app.log").unwrap();

    wal.write_log("helloo").expect("Something went wrong");
    wal.write_log("its").expect("Something went wrong");
    wal.write_log("me").expect("Something went wrong");
    wal.write_log("kalfar").expect("Something went wrong");

    println!("Hello, world!");
}
