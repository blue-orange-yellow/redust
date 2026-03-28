# Phase3: TTL / Expiry

## Goal
```bash
redis-cli SET mykey hello EX 5
# -> OK
redis-cli GET mykey
# -> "hello"
# wait 5 seconds
redis-cli GET mykey
# -> (nil)
```

## Three steps to get there
1. Extend the Hashmap value type
Currently the store is `HashMap<Vec<u8>, Vec<u8>>`. Add an expiration timestamp alongside the value:
```
HashMap<Vec<u8>, (Vec<u8>, Option<Instant>)>
```

`Some<Instand>` means the key has an expiration. `None` means no expiration (same behavior as before).

2. Extend SET command parsing
When retrieving a value from the HashMap, compare `Instant::now()` against the stored expiration.
If the current time is past the deadline, treat it as if the key does not exist.
This is called lazy expiration - the key is only considered expired when it is accessed.

## Out of scope
`PX` (millisecond precision), `EXAT` (Unix timestamp), background automatic deletion.

## New Constructs
```rust
use std::time::Instant;
use std::time::Duration;

// 5 seconds from now
let expires_at = Instant::now() + Duration::from_secs(5);

// check if expired
if Instant::now() > expires_at {
    // expired
}
```

## Verification
```bash
# Terminal 1
cargo run

# Terminal 2
redis-cli SET mykey hello EX 5
# -> OK
redis-cli GET mykey
# -> "hello"
# wait 5 seconds
redis-cli GET mykey
# -> (nil)
