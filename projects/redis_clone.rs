// example must have main function
// Goal: Network + memory

// Build:

// ```bash
// cargo run --bin redis_clone
// redis-cli -p 6380 SET foo bar
// redis-cli -p 6380 GET foo
// ```

// Learn:

// - TCP — `TcpListener::bind` accepts connections; each connection is a byte stream read with
//   `BufReader`; responses are written back with `write_all`
// - parsing — the Redis Serialization Protocol (RESP) uses `*N\r\n` (array), `$N\r\n` (bulk
//   string), `+OK\r\n` (simple string), `-ERR msg\r\n` (error) as its wire format
// - state — `Arc<Mutex<HashMap<String, String>>>` shared across connection handler tasks;
//   each task locks, reads/writes, then drops the guard immediately

// Progress:

// Extra:

// - [ ] persistence — dump the store to a file on SIGTERM and reload on startup

fn main() {
    println!("redis_clone: not yet implemented");
}
