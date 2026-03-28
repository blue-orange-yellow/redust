# Phase1: TPC Server + RESP Parser

## Goal
Connect with `redis-cli`, send `PING` receive `PONG`.

## Three steps to get there

1. TCP Server
Bind to `127.0.0.1:6379` and wait for connections with `accept()`.
The returned `stream` is a dedicated communication channel to the connected client.
Single-threaded, one connection at a time is fine for Phase 1.

2. RESP Parser
Read Raw bytes from `stream.read()` and parse them as RESP.
Commands from clients are always an Array of Bulk Strings.
The raw bytes looks like this:

```
*1\r\n$4\r\nPING\r\n
```

The parser reads this left to right in the following steps:
1. Read `*N\r\n` -> N is the number of elements in the array
2. For each element, read `$len\r\n` -> len is the byte length of the data that follows
3. Read exactly `len` bytes -> this is the data itself
4. Read the trailing `\r\n`
5. Repeat steps 2-4, N times

The output is a list of command + arguments, e.g. `Vec<Vec<u8>>`. For the example above, the result would be `[b"PING"]`.

3. Command Dispatch
Match on the first element of the parsed result. For Phase 1, only handle `PING`. Write `+PONG\r\n` back to the stream.

## Out of scope
`SET` / `GET`, data store, concurrent connections, TTL, detaied error handling. All deferred to Phase 2+.

## Verification
```bash
# Terminal 1
cargo run

# Terminal 2
redis-cli PING
# -> P0NG means done
```
