# Talking about Panic

## What is Panic?

- An unrecoverable error is something your program cannot (or should not) continue running after. Thats what panic do

- we can throw a panic using panic! macro

---

## Stack Unwinding vs Aborting

### Unwinding (default)
- Walks back up the call stack, runs destructors (drop) for each variable, cleans up memory.

### Aborting
- Immediately ends the program. Faster, smaller binary, but no cleanup.

**cargo.toml:**

```toml
[profile.release]
panic = "abort"
```

---

## Best Practices

- only use panic where it is important otherwise use result and pattern match to handle

### Examples:

```rust
// this will panic 
let num: i32 = "42".parse().unwrap();

// this will panic but we known that ip is correct still prefer result enum 
use std::net::IpAddr;
let home: IpAddr = "127.0.0.1"
  .parse()
  .expect("Hardcoded IP address should be valid");
```
