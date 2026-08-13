use std::fs::{File, OpenOptions};
use std::io::Write; // brings the `Write` trait into scope. `File` only gets `write_all`/`write_fmt`
// (what `writeln!` uses under the hood) because this trait is imported here.

pub struct Wal {
    file: File, // owns the underlying OS file descriptor
    index: u64, // last index written; recovered from disk on startup, then just incremented in memory
}

impl Wal {
    pub fn new(path: &str) -> std::io::Result<Self> {
        // OpenOptions is a builder — nothing touches the OS until `.open()` runs the actual
        // open() syscall. append(true) = O_APPEND (every write atomically seeks-to-end-then-writes,
        // so concurrent writers can't clobber each other's offset). create(true) = O_CREAT
        // (create the file if it doesn't exist; open as-is if it does — no truncation).
        let file = OpenOptions::new().append(true).create(true).open(path)?;
        let mut index = 0;

        // Reads the ENTIRE file into memory as one String. Fine at this size; would need a
        // different strategy (streaming, or seeking from the end) once the log gets large —
        // a deliberate simplification for now, not an oversight.
        let content = std::fs::read_to_string(path)?;

        if content.len() != 0 {
            // `writeln!` always appends '\n' as the LAST byte of a completed write. So "does the
            // raw file end in '\n'" is our signal that the most recent write finished before any
            // crash — this is the actual durability check, not a string-formatting detail.
            if content.ends_with('\n') {
                // `.lines()` splits on '\n' / '\r\n' and DROPS the terminator from every yielded
                // &str — including for a string that has a trailing newline, it does not produce
                // a trailing empty line. That's exactly why the check above has to run on `content`
                // itself, before `.lines()` — by the time you have a `line`, the terminator info is gone.
                // `.last()` walks the whole iterator and returns Option<&str> (None only if the
                // string were empty, which can't happen here — we're inside the `content.len() != 0`
                // and `ends_with('\n')` guards).
                let line = content.lines().last().ok_or_else(|| {
                    // `ok_or_else` converts `Option<T>` -> `Result<T, E>`. The closure only runs
                    // (and only builds the error) on the `None` path — lazy, no cost on `Some`.
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "No last line")
                })?; // `?` now works because ok_or_else already turned this into a Result

                index = line
                    .split(".") // splits on EVERY '.' in the line, not just the first
                    .next() // first token = the index prefix. Safe to take just [0] regardless of
                    // how many more '.'s appear later in the message, since the index is always
                    // the substring before the very first '.' on the line.
                    .ok_or_else(|| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "malformed line")
                    })?
                    .parse() // &str -> Result<u64, ParseIntError>; target type inferred from `index: u64`
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                // `map_err` transforms only the `Err` variant, leaves `Ok` untouched. Needed
                // because `ParseIntError` has no automatic conversion into `io::Error` — `?`
                // alone can't bridge two unrelated error types.
            } else {
                // File does NOT end in '\n' -> the last write was torn (process/power died
                // mid-write, before this entry's fsync). `rsplit_once` finds the LAST '\n' in
                // the whole file and splits there, returning `Option<(&str, &str)>`: everything
                // before that '\n', and everything after it. The '\n' itself is consumed by
                // rsplit_once — it appears in neither half.
                if let Some((trimmed, _torn_tail)) = content.rsplit_once('\n') {
                    // `trimmed` = every entry up to and including the last COMPLETE one (minus
                    // its trailing '\n', which rsplit_once ate). Because we split at the LAST
                    // '\n' in the raw file, every line inside `trimmed` was already correctly
                    // terminated when it was originally written — no need to re-check or recurse
                    // into it. This can only ever strip ONE trailing fragment per crash, because
                    // `write_log` does exactly one write + one sync_data before returning, so at
                    // most one entry can ever be "in flight" at the moment of a crash.
                    let line = trimmed.lines().last().ok_or_else(|| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "No last line")
                    })?;

                    index = line
                        .split(".")
                        .next()
                        .ok_or_else(|| {
                            std::io::Error::new(std::io::ErrorKind::InvalidData, "malformed line")
                        })?
                        .parse()
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

                    // `set_len` is a raw truncate syscall — it just moves the OS's end-of-file
                    // marker, no full-file rewrite. The `+ 1` accounts for the '\n' that
                    // rsplit_once stripped out of `trimmed`: without it, we'd truncate one byte
                    // short of where the real, valid data ends, and the next `write_log` call
                    // (append mode) would land directly on top with no separating '\n'.
                    file.set_len((trimmed.len() + 1) as u64)?;
                } else {
                    // rsplit_once returns None when '\n' doesn't occur in content at all.
                    // That means the whole file is a single unterminated entry (crash hit
                    // before the very first write completed) — no valid line exists to
                    // recover, so truncate the file to 0 bytes.
                    file.set_len(0 as u64)?;
                }
            }
        }

        Ok(Wal { file, index })
    }

    pub fn write_log(&mut self, message: &str) -> std::io::Result<()> {
        self.index += 1;
        // `writeln!` formats "{index}.{message}\n" and calls `write_fmt` on `self.file`, which
        // internally loops `write()` syscalls (same guarantee as `write_all`) until every byte
        // has been handed to the OS. At the point this returns, the bytes are sitting in the
        // kernel's page cache — NOT yet guaranteed to survive a power loss.
        writeln!(self.file, "{}.{}", self.index, message)?;

        // Forces the kernel to flush this file's dirty pages to physical storage before
        // returning. This is the actual durability boundary: only once this call returns `Ok`
        // is the entry guaranteed to survive a crash.
        self.file.sync_data()?;

        Ok(())
    }
}
