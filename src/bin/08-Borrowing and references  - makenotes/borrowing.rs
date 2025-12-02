/* ----- BORROWING AND REFERENCES ----- */
// 1. REFERENCES (&):
//    - Allow you to refer to a value without taking ownership
//    - Created using the & operator
//    - Original owner keeps ownership

// 2. IMMUTABLE REFERENCES (&T):
//    - Allow reading but not modifying
//    - Can have multiple immutable references
//    - Most common type of reference

// 3. MUTABLE REFERENCES (&mut T):
//    - Allow reading and modifying
//    - Only ONE mutable reference at a time
//    - Cannot coexist with immutable references

// 4. BORROWING RULES:
//    - Either one mutable reference OR any number of immutable references
//    - References must always be valid (no dangling references)
//    - Enforced at compile time

// 5. REFERENCE SCOPE:
//    - Starts where reference is introduced
//    - Ends at last use

// 6. DEREFERENCING (*):
//    - Use * to access the value behind a reference
//    - Often automatic in Rust (method calls, comparisons)
//    - Explicitly needed for some operations

fn main() {
    basic_reference_ex();
    immutable_references_ex();
    mutable_reference_ex();
    multiple_immutable_refs();
    mutable_reference_restriction();
    reference_scope_ex();
    borrowing_in_functions();
    dereferencing_ex();
    practical_ex();
}

// BASIC REFERENCE EXAMPLE
fn basic_reference_ex() {
    let s1 = String::from("anaya");
    
    // Without references, we'd have to return the value to keep using it
    // With references, we can pass it without transferring ownership
    let len = calculate_length(&s1);
    
    println!("The length of '{}' is {}.", s1, len);
    println!("s1 is still valid because we only borrowed it!");
}

fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
}  // s goes out of scope, but since it doesn't have ownership, nothing is dropped

// IMMUTABLE REFERENCES
fn immutable_references_ex() {
    let s = String::from("anaya");
    
    // Create an immutable reference
    let r = &s;
    
    println!("Original: {}", s);
    println!("Reference: {}", r);
    
    // Cannot modify through immutable reference
    // r.push_str(" world");  // ERROR Cannot modify through immutable reference
}

// MUTABLE REFERENCE
fn mutable_reference_ex() {
    let mut s = String::from("pretty");
    println!("Before: {}", s);
    
    // Pass a mutable reference to modify the value
    change(&mut s);
    
    println!("After: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", anaya");
}

// MULTIPLE IMMUTABLE REFERENCES
fn multiple_immutable_refs() {
    let s = String::from("anaya");
    
    // You can have multiple immutable references
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    
    println!("r1: {}", r1);
    println!("r2: {}", r2);
    println!("r3: {}", r3);
    // Multiple immutable references are allowed
}

// MUTABLE REFERENCE RESTRICTION
fn mutable_reference_restriction() {
    let mut s = String::from("pretty");
    
    let r1 = &mut s;
    // let r2 = &mut s;  // ERROR! Cannot have two mutable references at once
    
    r1.push_str(" anaya");
    println!("r1: {}", r1);
    
    // After r1 is done being used, we can create another mutable reference
    let r2 = &mut s;
    r2.push_str("!");
    println!("r2: {}", r2);
    
    // Only ONE mutable reference at a time
}

// REFERENCE SCOPE (Non-Lexical Lifetimes)
fn reference_scope_ex() {
    let mut s = String::from("pretty");
    
    // Immutable references
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    // This is OK! Mutable reference can be created after immutable refs are done
    let r3 = &mut s;
    r3.push_str(" anaya");
    println!("r3: {}", r3);
    
    // References scope ends at their last use
}

// BORROWING IN FUNCTIONS
fn borrowing_in_functions() {
    let s1 = String::from("pretty");
    
    // Pass immutable reference
    print_string(&s1);
    println!("s1 is still valid: {}", s1);
    
    let mut s2 = String::from("hello");
    println!("Before modification: {}", s2);
    
    // Pass mutable reference
    append_world(&mut s2);
    println!("After modification: {}", s2);
}

fn print_string(s: &String) {
    println!("Printing: {}", s);
}

fn append_world(s: &mut String) {
    s.push_str(" anaya");
}

// DEREFERENCING
fn dereferencing_ex() {
    let x = 5;
    let y = &x;  // y is a reference to x
    
    println!("x = {}", x);
    println!("y (reference) = {}", y);
    println!("*y (dereferenced) = {}", *y);
    
    // Comparison requires dereferencing
    assert_eq!(5, x);
    assert_eq!(5, *y);  // Must dereference to compare values
    
    // String example
    let s = String::from("hello");
    let r = &s;
    
    // Rust often dereferences automatically
    println!("Length via reference: {}", r.len());  // Automatic dereferencing
}

// PRACTICAL EXAMPLES
fn practical_ex() {
    // Example 1: Reading without taking ownership
    let text = String::from("Hello Rust");
    let word_count = count_words(&text);
    println!("'{}' has {} words", text, word_count);
    
    // Example 2: Modifying in place
    let mut message = String::from("Hello");
    add_exclamation(&mut message);
    println!("Modified message: {}", message);
    
    // Example 3: Multiple readers
    let s1 = String::from("hello");
    let s2 = String::from("hello");
    let are_equal = compare_strings(&s1, &s2);
    println!("Are '{}' and '{}' equal? {}", s1, s2, are_equal);
    
    // Example 4: Chaining operations with mutable references
    let mut data = String::from("Rust");
    add_prefix(&mut data);
    add_suffix(&mut data);
    println!("Final data: {}", data);
}

fn count_words(s: &String) -> usize {
    s.split_whitespace().count()
}

fn add_exclamation(s: &mut String) {
    s.push_str("!!!");
}

fn compare_strings(s1: &String, s2: &String) -> bool {
    s1 == s2
}

fn add_prefix(s: &mut String) {
    *s = format!("rust {}", s);
}

fn add_suffix(s: &mut String) {
    s.push_str(" is hard");
}

/* 
COMMON ERRORS AND SOLUTIONS:

ERROR: Cannot borrow as mutable more than once
SOLUTION: Ensure only one mutable reference exists at a time

ERROR: Cannot borrow as mutable because it is also borrowed as immutable
SOLUTION: Ensure immutable references are done being used before creating mutable ref

ERROR: Cannot borrow as immutable because it is also borrowed as mutable
SOLUTION: Ensure mutable reference is done being used before creating immutable refs

ERROR: This function's return type contains a borrowed value, but there is no value for it to be borrowed from
SOLUTION: Return owned data or use lifetimes to specify relationship between references
*/

