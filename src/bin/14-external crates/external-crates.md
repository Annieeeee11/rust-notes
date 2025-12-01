# Talking about External Crates

## What are External Crates?

- External crates are libraries created by the Rust community that you can add to your project
- They provide additional functionality beyond what's in the standard library
- Use `cargo add {crate_name}` to add them to your project

---

## Common External Crates

### 1) dotenv - Environment Variables

- Used for loading environment variables from a `.env` file
- Helps keep sensitive data (like API keys) out of your code

**Install:**

```bash
cargo add dotenv
```

**Example:**

```rust
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let var = env::var("XYZ");

    match var {
        Ok(str) => println!("{}", str),
        Err(_e) => println!("error"),
    }
}
```

**Understanding:**
- `dotenv().ok()` loads variables from `.env` file
- `env::var("XYZ")` retrieves the environment variable
- Returns a `Result` that you need to handle with match or unwrap

---

### 2) chrono - Date and Time

- Used to fetch current time and work with dates
- Provides UTC and local time functionality

**Install:**

```bash
cargo add chrono
```

**Example:**

```rust
use chrono::{Utc, Local};

fn main() {
    let utc = Utc::now();
    let local_time = Local::now();
    println!("{} , {}", utc, local_time);
}
```

**Understanding:**
- `Utc::now()` gets the current UTC time
- `Local::now()` gets the current local time
- Both can be formatted and manipulated for various time operations

---

## How to Find Crates

- Visit [crates.io](https://crates.io/) the official Rust package registry
- Search for functionality you need
- Check documentation, downloads, and recent updates before using

