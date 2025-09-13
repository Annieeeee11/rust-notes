/*
--  #[cfg(test)] → compile this tests module only when running cargo test.
--  use super::*; → import everything from the parent module so we can test it.
--  #[test] → tells Rust this function is a test case.
--  assert_eq! → a macro that checks if left == right, otherwise fails the test.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(3), 5);  // if false → panic! → test fails
    }
}

// TEST MACROS
#[test]
fn test_comparisons() {
    assert!(2 + 2 == 4);
    assert_eq!(add_two(3), 5);
    assert_ne!(add_two(3), 6);
}

// Intentional Failures
#[test]
#[should_panic]
fn test_panic() {
    panic!("This should panic");
}


// Returning Result in Tests
#[test]
fn test_with_result() -> Result<(), String> {
    if add_two(2) == 4 {
        Ok(())
    } else {
        Err(String::from("add_two did not add correctly"))
    }
}


// If try_parse_number("42") fails, the error bubbles up via ?. That makes the test fail automatically, without panicking manually
fn try_parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

#[test]
fn parse_valid_number() -> Result<(), Box<dyn std::error::Error>> {
    let num = try_parse_number("42")?; // use `?` instead of unwrap
    assert_eq!(num, 42);
    Ok(())
}