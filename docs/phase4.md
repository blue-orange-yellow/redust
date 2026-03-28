# Phase4: Concurrency (std::thread)

## Goal
Multiple `redis-cli` clients can connect simultaneously and independently execute `SET` / `GET` / `PING`.

## Current problem
The inner loop blocks the outer loop from returning to `accept()`. While client A is connected, client B is stuck waiting.

## Three steps to get there
1. Spawn a thread per connection
After `accept()` return a `stream`, pass it to `std::thread::spawn` so the main thread can immediately loop back to `accept()` the next connection.
```rust
loop {
    let (stream, _) = listener.accept()?;
    std::thread::spawn(move || {
        // handle stream read/write here
    })
}
```

2. Share the HashMap across threads
The HashMap now needs to be accessed from multiple threads.
This requires `Arc<Mutex<HashMap>>`.

`Arc` - a reference-counted pointer that allows multiple threads to share ownership of the same data.
`MUtex` - a lock that ensures only one thread can access the data at a time.

```rust
use std::sync::{Arc, Mutex};

let store = Arc::new(Mutex::new(HashMap::new()))

loop {
    let (stream, _) = listener.accept()?;
    let store = Arc::clone(&store);
    
    std::thread::spawn(move || {
        // access HashMap via store.lock().unwrap()
    })
}
```

3. Lock before every HashMap operation
For `SET`: `store.lock().unwrap().insert(key, (value, expires_at))`.
For `get`: `store.lock().unwrap().get(&key)`.
Each operation acquires the lock, does its work, and releases it when the lock guard goes out of scope.

## Out of scope
Thread pool, async runtime (tokio), lock optimization.

## Verification
```bash
# Terminal 1
cargo run

# Terminal 2
redis-cli
> SET mykey hello

# Terminal 3 (should connect immediately while Terminal 2 is still active)
redis-cli
> GET mykey
# -> "hello"
```
If Terminal 3 responds instantly while Terminal 2 is still connected, it works.
