Talking about comments

1) 
//  single line

2) 

/* This is a 
    multi-line comment 
*/

3)

/*    println!("This will be ignored");
    /* Nested comment! */
*/

4) Rust has special comments that are used to write API documentation for your code — not just notes to yourself.
- These docs are used by cargo doc to generate beautiful HTML documentation automatically.

4.1) /// — for documenting items (like functions, structs, enums, traits, etc.)
- Appears above the item.
- Markdown supported (for formatting, code blocks, headings, etc.)
- These comments are visible in generated docs.
{
    /// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
}

4.2) //! — for documenting modules or the whole crate
- Used at the top of a file or module to document the entire module or crate.
- lib.rs to document the crate
- mod.rs or submodules to describe the module's purpose
{
//! This module provides math utilities.
//! 
//! It includes functions like `add` and `subtract`.

/// Adds two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
}