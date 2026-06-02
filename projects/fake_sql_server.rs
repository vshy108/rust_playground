// example must have main function
// Goal: Protocol engineering

// Build:

// ```bash
// cargo run --bin fake_sql_server
// psql -h 127.0.0.1 -p 5433 -U test testdb
// ```

// Learn:

// - protocol — the PostgreSQL wire protocol starts with a Startup message (version + params),
//   then an Authentication request/response, followed by ReadyForQuery; each message has a
//   1-byte type tag and a 4-byte big-endian length prefix
// - binary parsing — read fixed-width fields with `u32::from_be_bytes`; read variable-length
//   strings as null-terminated byte sequences; use a cursor over the byte buffer

// Progress:

// Extra:

// - [ ] query execution — parse simple SELECT statements and return mock rows

fn main() {
    println!("fake_sql_server: not yet implemented");
}
