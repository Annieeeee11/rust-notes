# Talking about Borrowing and References

## What is Borrowing?

- Borrowing allows you to reference a value without taking ownership of it
- Instead of moving ownership, you can pass a reference to a value
- The original owner keeps ownership and the value won't be dropped
- This solves the problem of having to return values just to give ownership back

---

## References

### What is a Reference?

- A reference is like a pointer: it's an address we can follow to access data
- The data is owned by some other variable
- Unlike a pointer, a reference is guaranteed to point to a valid value for the life of that reference
- References are created using the `&` operator

### Syntax

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);  // &s1 creates a reference to s1
```

**Understanding:**
- `&s1` creates a reference that refers to the value of `s1` but doesn't own it
- When the reference goes out of scope, the value it points to won't be dropped
- The opposite of referencing (`&`) is dereferencing (`*`)

---

## Immutable References

### Basic Usage

```rust
fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
}  // s goes out of scope, but it doesn't have ownership, so nothing is dropped
```

**Understanding:**
- By default, references are immutable
- You can have multiple immutable references to the same value
- Cannot modify the value through an immutable reference

### Multiple Immutable References

```rust
let s1 = String::from("hello");
let r1 = &s1;
let r2 = &s1;
let r3 = &s1;
// All valid! Multiple immutable references are allowed
```

---

## Mutable References

### Basic Usage

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);  // Pass a mutable reference
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

**Understanding:**
- Use `&mut` to create a mutable reference
- The original variable must be declared as `mut`
- Allows you to modify the borrowed value

### The Big Restriction

**You can have only ONE mutable reference to a particular piece of data at a time**

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;  // ERROR! Cannot have two mutable references
```

**Why this restriction?**
- Prevents data races at compile time
- A data race occurs when:
  1. Two or more pointers access the same data at the same time
  2. At least one pointer is being used to write to the data
  3. There's no mechanism to synchronize access to the data

---

## The Rules of References

### Rule 1: Mutable XOR Immutable

You can have EITHER:
- One mutable reference
- Any number of immutable references

But NOT both at the same time!

```rust
let mut s = String::from("hello");

let r1 = &s;      // OK
let r2 = &s;      // OK
let r3 = &mut s;  // ERROR! Cannot have mutable reference while immutable refs exist

println!("{}, {}", r1, r2);
```

### Rule 2: References Must Always Be Valid

- References must always point to valid data
- Rust prevents dangling references (references to freed memory)

```rust
fn dangle() -> &String {  // ERROR! This would return a reference to freed memory
    let s = String::from("hello");
    &s  // s goes out of scope and is dropped, reference would be invalid
}
```

---

## Reference Scope

### Non-Lexical Lifetimes (NLL)

- A reference's scope starts from where it's introduced
- Continues through the last time that reference is used
- Not necessarily until the end of the block

```rust
let mut s = String::from("hello");

let r1 = &s;      // Immutable reference
let r2 = &s;      // Another immutable reference
println!("{} and {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s;  // OK! Mutable reference after immutable refs are done
println!("{}", r3);
```

**Understanding:**
- The compiler can tell that immutable references are no longer being used
- After their last use, it's safe to create a mutable reference
- This makes Rust more flexible while maintaining safety

---

## Borrowing in Functions

### Passing Immutable References

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);  // s1 is still valid!
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

**Understanding:**
- Function takes a reference, doesn't take ownership
- Original variable remains valid after function call
- No need to return the value to give ownership back

### Passing Mutable References

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);  // Prints "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

---

## Common Patterns

### Pattern 1: Read-Only Access

Use immutable references when you just need to read data:

```rust
fn print_string(s: &String) {
    println!("{}", s);
}
```

### Pattern 2: Modification

Use mutable references when you need to modify data:

```rust
fn append_world(s: &mut String) {
    s.push_str(" world");
}
```

### Pattern 3: Multiple Readers

Multiple immutable references for concurrent reading:

```rust
fn compare_strings(s1: &String, s2: &String) -> bool {
    s1 == s2
}
```

---

## Dereferencing

### The `*` Operator

- Use `*` to follow a reference to the actual value
- Often not needed due to automatic dereferencing in Rust

```rust
let x = 5;
let y = &x;

assert_eq!(5, x);
assert_eq!(5, *y);  // Dereference to get the value
```

**Understanding:**
- `y` is a reference to `x`
- `*y` dereferences to get the actual value
- Rust often dereferences automatically in certain contexts

---

## Borrowing Rules Summary

### The Two Rules

1. **At any given time, you can have EITHER:**
   - One mutable reference
   - Any number of immutable references

2. **References must always be valid:**
   - No dangling references
   - References cannot outlive the data they refer to

---

## Why Borrowing Matters

### Memory Safety

- Prevents use after free bugs
- Prevents double free bugs
- Prevents dangling pointers
- All checked at compile time!

### Concurrency Safety

- Prevents data races
- Multiple readers OR single writer
- Safe concurrent access patterns

### Performance

- No runtime overhead
- Zero cost abstraction
- As fast as manual memory management
- Safer than garbage collection

---

## Common Use Cases

### Use Immutable References When:
- Reading data without modification
- Passing data to functions for inspection
- Multiple parts of code need to read the same data

### Use Mutable References When:
- Modifying data in place
- Passing data to functions that need to change it
- Only one part of code needs write access at a time

### Take Ownership When:
- The function needs to consume the value
- Transferring responsibility for cleanup
- Building new data structures from old ones