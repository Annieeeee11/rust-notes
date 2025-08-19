/*
----- .trim() -----
Removes leading and trailing whitespace (like spaces, tabs, and newlines) from a string.
-- Why use it? --
When reading user input (e.g., with read_line()), the result often includes a \n (newline) at the end. .trim() removes that so it can be safely parsed as a number or used as a clean string.
-- Alternatives: --
.trim_start() – trims only the beginning (leading whitespace)
.trim_end() – trims only the end (trailing whitespace)


----- .parse::<T>() -----
Converts a string into another type -- like i32, u32, f64, etc.
Returns a Result<T, Err> – meaning it may succeed or fail.
-- Why use it? --
Needed when converting text input into a numeric type or any other type that implements the FromStr trait.
-- Be careful: --
.parse() can fail (e.g., parsing "hello" as a number). That's why it returns a Result.
-- Alternatives: --
Use the FromStr trait


----- .expect("message") -----
Used on Result or Option types to unwrap the value or panic with a custom error message if it’s Err or None.
simple version -- Extract value or panic with message
-- Why use it? --
Quick way to force a result when you're confident it won’t fail — and want to print a helpful message if it does.
-- Alternatives: --
.unwrap() – does the same thing but with a generic panic message.

----- .unwrap() -----
- this unwraps the returns, when the developer is sure that something will return something 
- this should return or will crash the error
- it should not be used except a few situation like: 
when project start you connect to a database. if the db dosent exist it is good to crash 


Also used on Result or Option types to extract the value, but panics with a default message if there’s an error.
simple version -- Extract value or panic (default msg)
-- Downside: --
Not beginner-friendly for error handling, because it crashes without context. Only use if you're confident it will succeed (e.g., in tests or prototypes).
-- Alternatives: --
.expect("your message") – better for debugging.
Pattern matching (match, if let)
? operator (in functions that return Result):
*/

fn trimm() {
    let input = "  42\n";
    let trimmed = input.trim(); // "42"
    //Alternatives
    let trimmed_both = input.trim_start().trim_end(); //return a new string slice (&str)
}

fn parsee() {
    let num_str = "42";
    let number: u32 = num_str.parse().unwrap(); // Converts "42" → 42
    // Alternatives
    // use std::str::FromStr; // add this in start to make this work
    let parsed = u32::from_str("42").unwrap();
}

fn expects() {
    let input = "42";
    let number: u32 = input.parse().expect("Please enter a valid number");
    // Alternatives
    match input.parse::<u32>() {
        Ok(num) => num,
        Err(e) => {
            println!("Invalid input: {}", e);
            return;
        }
    }
}

fn unwrapp() {
    let number: u32 = "42".parse().unwrap(); // Ok
    let number: u32 = "hi".parse().unwrap(); // panics!
    // Alternatives
    fn parse_input(input: &str) -> Result<u32, std::num::ParseIntError> {
        let number = input.trim().parse()?;
        Ok(number)
    }
}
