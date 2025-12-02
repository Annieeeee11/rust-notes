# Talking about Advanced Features

## What are Advanced Features?

- Rust has features that are less commonly used but powerful
- These features provide more control and flexibility
- Understanding these helps you write more sophisticated Rust code
- Most Rust code doesn't need these features, but they're available when needed

---

## Unsafe Rust

### What is Unsafe?

- Rust has a second language hidden inside it that doesn't enforce memory safety guarantees
- Called unsafe Rust and works like regular Rust but gives you extra superpowers
- Exists because static analysis is conservative

### Unsafe Superpowers

You can take five actions in unsafe Rust:

1. Dereference a raw pointer
2. Call an unsafe function or method
3. Access or modify a mutable static variable
4. Implement an unsafe trait
5. Access fields of unions

**Understanding:**
- Unsafe doesn't turn off the borrow checker or disable Rust's safety checks
- You still get some degree of safety inside an unsafe block
- Use `unsafe` keyword to mark a block of code as unsafe

### When to Use Unsafe

- Interfacing with C code
- Building safe abstractions that the compiler can't verify
- Performance critical code where you need more control
- Working with hardware or operating system primitives

---

## Raw Pointers

### Types of Raw Pointers

- `*const T` - immutable raw pointer
- `*mut T` - mutable raw pointer

**Understanding:**
- Can ignore borrowing rules
- Aren't guaranteed to point to valid memory
- Can be null
- Don't implement automatic cleanup
- Creating raw pointers is safe, dereferencing them requires unsafe

---

## Advanced Traits

### Associated Types

- Connect a type placeholder with a trait
- Similar to generics but can only have one concrete type per implementation
- Makes the trait definition cleaner

**Understanding:**
- Use when you want one implementation per type
- The type is determined by the implementation
- More ergonomic than generics for some cases

### Default Generic Type Parameters

- Can specify a default type for generic type parameters
- Use `<PlaceholderType=ConcreteType>` syntax
- Useful for operator overloading

### Fully Qualified Syntax

- Used to disambiguate when multiple traits have methods with the same name
- Syntax: `<Type as Trait>::function(receiver_if_method, args)`
- Tells Rust exactly which implementation to use

### Supertraits

- Require one trait to have another trait's functionality
- Syntax: `trait MyTrait: OtherTrait`
- The dependent trait is called a supertrait

### Newtype Pattern

- Create a new type that wraps an existing type
- Allows implementing external traits on external types
- Zero runtime cost due to compile time optimization
- Provides type safety and abstraction

---

## Advanced Types

### Type Aliases

- Create a synonym for an existing type
- Use `type` keyword
- Doesn't create a new type, just an alias
- Useful for reducing repetition

**Understanding:**
- Makes long type names more manageable
- Commonly used with `Result<T, E>` types
- The alias and original type are interchangeable

### Never Type

- `!` type that never returns
- Used in functions that never return (like `panic!`)
- Useful in match arms and other control flow

**Understanding:**
- Can be coerced into any other type
- Useful for functions that loop forever or always panic
- Makes certain patterns more expressive

### Dynamically Sized Types (DST)

- Types whose size can only be known at runtime
- Examples: `str` (not `&str`), trait objects
- Must always be used behind a pointer
- Rust needs to know size at compile time for stack allocation

**Understanding:**
- `&str` is two values: address and length
- Trait objects store pointer to data and pointer to vtable
- The `Sized` trait is automatically implemented for types with known size

---

## Advanced Functions and Closures

### Function Pointers

- Can pass regular functions to functions
- Use `fn` type (not `Fn` trait)
- Function pointers implement all three closure traits (`Fn`, `FnMut`, `FnOnce`)

**Understanding:**
- Unlike closures, `fn` is a type rather than a trait
- Can use function pointers where closures are expected
- Useful when interfacing with C code

### Returning Closures

- Can't return closures directly because they don't have a known size
- Must return them behind a pointer
- Use `Box<dyn Fn()>` or similar

**Understanding:**
- Closures are represented by traits
- Each closure has its own anonymous type
- Boxing allows returning different closure types

---

## Macros

### Declarative Macros

- Use `macro_rules!` to define
- Pattern matching on Rust code structure
- Can take variable number of arguments
- Expanded at compile time

**Understanding:**
- More powerful than functions
- Can generate code based on input
- Operate on syntax trees, not values

### Procedural Macros

Three kinds of procedural macros:

1. **Custom derive** - `#[derive(MyTrait)]`
2. **Attribute-like** - `#[my_attribute]`
3. **Function-like** - `my_macro!()`

**Understanding:**
- Operate on token streams
- Must be defined in their own crate
- More powerful but more complex than declarative macros

---

## Advanced Lifetimes

### Lifetime Subtyping

- One lifetime can outlive another
- Ensures a reference is valid for the entire time it's needed
- Compiler infers most of the time

### Lifetime Bounds

- Specify that a type must live at least as long as a reference
- Syntax: `T: 'a` means type T must live at least as long as lifetime 'a

### Higher-Ranked Trait Bounds (HRTB)

- Use `for<'a>` syntax
- Specify that a trait bound must hold for all possible lifetimes
- Useful with closures and function pointers

**Understanding:**
- Allows more flexible lifetime specifications
- Compiler can work with any lifetime
- Common in advanced generic code

---

## Operator Overloading

**What it is:**
- Customize the behavior of operators like `+`, `-`, `*`, etc.
- Implement traits from `std::ops` module
- Makes custom types more ergonomic

**Common Traits:**
- `Add`, `Sub`, `Mul`, `Div` for arithmetic
- `Index`, `IndexMut` for indexing
- `Deref`, `DerefMut` for dereferencing

---

## Foreign Function Interface (FFI)

**What it is:**
- Allows Rust to interact with code written in other languages
- Most commonly used with C
- Requires `unsafe` blocks

**Concepts:**
- `extern` keyword to declare foreign functions
- `#[no_mangle]` to prevent name mangling
- ABI (Application Binary Interface) specification

**Understanding:**
- Rust can call C functions
- C can call Rust functions
- Useful for integrating with existing libraries
- Requires careful handling of memory and types