# TODO: fake_sql_server (⭐ 9/10)

## Usage

```bash
cargo run --bin fake_sql_server
psql -h 127.0.0.1 -p 5433 -U test testdb
cargo test --bin fake_sql_server
```

## 1. Binary message codec

- [ ] Write a `read_message(stream) -> (u8, Vec<u8>)` that reads tag + length-prefixed body.
- [ ] Write `write_message(stream, tag, body)` that frames a response.

Acceptance check: codec round-trips a known byte sequence.

## 2. Startup handshake

- [ ] Read the client's Startup message (no tag byte, length + protocol version + params).
- [ ] Respond with `AuthenticationOk` (R + 8 bytes) then `ReadyForQuery` (Z + 5 bytes, status `I`).

Acceptance check: `psql` connects and shows the prompt after the handshake.

## 3. Simple query loop

- [ ] Read `Query` messages (tag `Q`); extract the SQL string.
- [ ] Respond with `EmptyQueryResponse` (`I`) for empty input; `CommandComplete` for anything else.

Acceptance check: typing `SELECT 1;` in psql returns "SELECT 0" without crashing.

## 4. RowDescription + DataRow

- [ ] For a hardcoded `SELECT` query, respond with `RowDescription`, one or more `DataRow`
  messages, then `CommandComplete`.

Acceptance check: `SELECT 1;` returns a single row with value `1`.

## 5. Tests

- [ ] Codec round-trips each message type.
- [ ] Handshake sequence produces the correct byte frames.
- [ ] Simple query returns CommandComplete.

## Extra: query execution

- [ ] Parse `SELECT <expr> FROM <table>` with a hand-rolled parser.
- [ ] Look up table names in a `HashMap<String, Vec<Row>>`; stream matching rows.

## Tips

- Implement one milestone at a time and keep each slice testable.
- Add a failing test first, then implement the smallest behavior to pass.
- Keep CLI/API surface stable while iterating internals.
- Validate both happy path and error path for every milestone.
- Run focused tests before broad checks.
