# Phase4b: Rewrite with tokio

## Goal
Same behavior as Phase 4 - multiple `redis-cli` clients connect simultaneously and work independently.
The diferrence is the implementation: async tasks instead of OS threads.

## Three steps to get there
1. Replace std types with tokio equivalents
Add `tokio` to `Cargo.toml` with `features = ["full"]`.
Replace `std::net::TcpListener` with `tokio::net::TcpListener`, and `std::io::{Read, Write}` with `tokio::io::{AsyncReadExt, AsynWriteExt}`.
Change `fn main()` to `#[tokio::main] async fn main()`.

2. Replace thread::spawn with tokio::spawn
`std::thread::spawn(move || {...})` become `tokio::spawn(async move {...})`.
Instead of one OS thread per connection (several MB each), this creates one async task per connection (a few hundred bytes each).
A small number of OS threads cooperatively run many tasks by switching between them.

3. Add .await to every I/O operation
`listener.accept()`, `stream.read(&mut buf)`, `stream.write_all(...)` all get `.await` appended.
The parser, command dispatch, HashMap operations, and `Arc<Mutex<HashMap>> all stay exactly the same.

## Out of scope
`tokio::sync::Mutex`, `select!`, channels, graceful shutdown.

## Verification
```bash
# Terminal 1
cargo run

# Terminal 2
redis-cli
> SET mykey hello

# Terminal 3 (should respond immediately)
redis-cli
> GET mykey
# -> "hello"
