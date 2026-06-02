# TODO: redis_clone

## Usage

```bash
cargo run --bin redis_clone
redis-cli -p 6380 SET foo bar
redis-cli -p 6380 GET foo
redis-cli -p 6380 DEL foo
cargo test --bin redis_clone
```

## 1. RESP parser

- [ ] Parse a RESP array command from a `BufReader<TcpStream>`.
- [ ] Return a `Vec<String>` of the command tokens (e.g. `["SET", "foo", "bar"]`).
- [ ] Return a parse error for malformed input.

Acceptance check: parsing `*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n` yields `["SET","foo","bar"]`.

## 2. In-memory store

- [ ] Define `type Store = Arc<Mutex<HashMap<String, String>>>`.
- [ ] Implement `SET key value` → `+OK\r\n`.
- [ ] Implement `GET key` → bulk string or `$-1\r\n` (nil) if missing.
- [ ] Implement `DEL key` → `:1\r\n` if deleted, `:0\r\n` if not found.

Acceptance check: SET then GET returns the stored value.

## 3. TCP server

- [ ] Bind `TcpListener` to `127.0.0.1:6380`.
- [ ] `tokio::spawn` a handler task per accepted connection.
- [ ] Loop inside each handler: parse one command, dispatch, write response.

Acceptance check: `redis-cli -p 6380 PING` returns `+PONG\r\n`.

## 4. Tests

- [ ] RESP parser round-trips SET/GET/DEL commands.
- [ ] SET then GET returns value.
- [ ] GET on missing key returns nil.
- [ ] DEL returns 1 for existing key, 0 for missing.

## Extra: persistence

- [ ] On SIGTERM, serialise the HashMap to a JSON file.
- [ ] On startup, load the file if it exists.
