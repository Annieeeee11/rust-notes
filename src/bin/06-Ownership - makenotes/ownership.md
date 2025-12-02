# Talking about Ownership

## What is Ownership?

- Ownership is make rust memory safe without needing a garbage collector
- Ownership is a set of rules that govern how a Rust program manages memory
- All programs have to manage the way they use a computer's memory while running

---

## Memory Management Approaches

### 1) Garbage Collection
- Some languages have garbage collection that constantly looks for no longer used memory (Java, Python, JavaScript)

### 2) Manual Memory Management
- In other languages, the programmer must explicitly allocate and free the memory (C, C++)

### 3) Ownership System (Rust)
- Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks
- If any of the rules are violated, the program won't compile

---

## The Stack and The Heap

### Stack

- Stores values in the order it gets them and removes the values in the opposite order (LIFO --> Last In, First Out)
- All data stored on the stack must have a known, fixed size
- Fast because it doesn't have to search for a place to store new data
- Data like integers, floats, booleans, chars are stored here
- **Number gets cloned on stack**

### Heap

- Less organized: when you put data on the heap, you request a certain amount of space
- The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer
- Slower than accessing data on the stack because you have to follow a pointer to get there
- Data like String, Vec, and other growable types are stored here
- **Ownership works on heap**

---

## Ownership Rules

1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

---

## Variable Scope

- A scope is the range within a program for which an item is valid
- When a variable comes into scope, it is valid
- It remains valid until it goes out of scope

---

## The String Type

### String Literals vs String Type

**String Literal (`&str`):**
- Immutable
- Fixed size, known at compile time
- Stored in the program binary
- Fast and efficient

**String Type (`String`):**
- Mutable
- Can grow or shrink
- Allocated on the heap
- Ownership rules apply

---

## Memory and Allocation

### Automatic Memory Management

- With the `String` type, in order to support a mutable, growable piece of text, we need to allocate memory on the heap
- The memory is automatically returned once the variable that owns it goes out of scope
- Rust calls a special function called `drop` automatically at the closing curly bracket

---

## Ways Variables and Data Interact

### 1) Move

- When you assign one variable to another with heap data, Rust moves the ownership
- The first variable is no longer valid
- This prevents double free errors

### 2) Clone

- If we want to deeply copy the heap data, we can use the `clone` method
- This creates a complete copy of the data
- More expensive operation

### 3) Copy

- Types stored entirely on the stack (like integers) implement the `Copy` trait
- These types are copied automatically, not moved
- Both variables remain valid after assignment
- **Numbers get cloned on stack**

---

## Copy Trait Types

Types that implement `Copy` trait:
- All integer types (`u32`, `i32`, etc.)
- Boolean type (`bool`)
- All floating point types (`f64`, `f32`)
- Character type (`char`)
- Tuples (only if they contain types that also implement `Copy`)

---

## Ownership and Functions

### Passing to Functions

- Passing a variable to a function will move or copy, just as assignment does
- Heap data gets moved
- Stack data gets copied

### Returning Values

- Returning values can also transfer ownership
- The ownership of a variable follows the same pattern every time

---

## Why Ownership Matters

- **Memory Safety**: No null pointer dereferences, no use after free, no double free
- **Thread Safety**: Ownership rules prevent data races at compile time
- **Zero-cost Abstractions**: No runtime overhead for memory safety
- **Predictable Performance**: No garbage collection pauses 