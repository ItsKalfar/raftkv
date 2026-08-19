// Build order - WAL -> Election -> Replication -> State machine

mod raft;
mod wal;

use wal::Wal;

fn main() -> std::io::Result<()> {
    let mut wal = Wal::new("app.log")?;

    wal.write_log("helloo")?;
    wal.write_log("its")?;
    wal.write_log("me")?;
    wal.write_log("kalfar")?;

    Ok(())
}
