Talking about tes -> 

- rust provides alot of safety checks but rust cant check our logical error 
- eg => if a add function add 2 numbers or not 

- for that we can make test to check our logic 

- Rust uses the #[test] attribute to mark a test function.
```
#[cfg(test)] // mods
#[test] // test
#[ignore] // ignore test
```


## run checks
- check using ```cargo test``` => Runs all functions marked with #[test]
- Runs tests in parallel by default.
- Captures stdout (your println! won’t show unless test fails or you run cargo test -- --nocapture).
- You can filter which tests to run:
```
cargo test it_adds_two
```

- cargo test → compiles your code in test mode and runs the test binary (a special executable that knows how to discover and execute tests)

## Rust macros for Testing

- assert!(condition) → fails if condition is false.
- assert_eq!(left, right) → fails if left != right, shows both values.
- assert_ne!(left, right) → fails if left == right.


## Intentional Failures
- If i expect a function to panic in some cases, i can test that too using #[should_panic]


## Organizing Tests
- Rust testing is split into two categories:

### Unit Tests

- Live inside your crate (in the same file as your code).
- Use #[cfg(test)] mod tests { ... }.
- Test small, private pieces of logic.

```
// lib.rs
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    /* use super::*; brings the parent module’s items into scope (so tests can call add).
       These tests (and any helpers inside tests) are compiled only for cargo test.
    */

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }
}
```

### Integration Tests

- Live in a tests/ directory at the project root.
- Each file is compiled as a separate crate.
- Used to test public API surface of your library.

```
// src/lib.rs
// tests/integration_test.rs

use my_crate; // bring your crate in like any other external dependency

#[test]
fn it_adds_two() {
    assert_eq!(my_crate::add_two(2), 4);
}
```

## cargo commands 

- cargo test                          => Run all tests in parallel, hiding passing output
- cargo test -- --test-threads=1	    => Run tests sequentially
- cargo test -- --show-output       	=> Show println! output for passing tests too
- cargo test <name>	                => Run a single test by exact name
- cargo test <substring>	            => Run all tests whose names contain substring
- cargo test -- --ignored	            => Run only ignored tests
- cargo test -- --include-ignored  	=> Run all tests, even ignored ones