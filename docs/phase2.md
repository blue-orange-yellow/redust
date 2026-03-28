# Phase2: SET / GET

## Goal
```bash
redis-cli SET mykey hello
# -> OK
redis-cli GET mykey
# -> "hello"
redis-cli GET unknown
# -> (nil)
```

## Three steps to get there
1. Data Store
Create a `HashMap<Vec<u8>, Vec<u8>>` outside the inner loop. Both keys and values are `Vec<u8>` to keep them binary-dafe.

2. Command Dispatch
Extend the current `PING`-only branch to handle `SET` and `GET`.

For `SET`, `results` will be `[b"SET", b"mykey", b"hello"]`(3 elements).
Insert `results[1]` as key and `results[2]` as value into the HashMap. Respond with `+OK\r\n`.

For `GET`, `results` will be `[b"Get", b"mykey"]`(2 elements).
Look up `results[1]` in the HashMap. if found, respond with a Bulk String(e.g. `$5\r\nhello\r\n`). if not found, respond with a Null Bulk String(`$-1\r\n`).

3. Dynamic Response Building
`PING` and `SET` return fixed byte strings, but `GET` requires building the response dynamically based on the value's length. This is the new challenge in Phase 2.

## Out of scope
TTL / Expiry, concurrent connections, `DEL` and other commands. All deffered to Phase 3+.

## Verification
```bash
# Terminal 1
cargo run

# Terminal 2
redis-cli SET mykey hello
# -> OK
redis-cli GET mykey
# -> "hello"
redis-cli GET unknown
# -> (nil)
```
