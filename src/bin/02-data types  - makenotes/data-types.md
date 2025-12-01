# Talking about Data Types

## What are Data Types?

- Every value in Rust has a data type
- Rust is a statically typed language, which means it must know the types of all variables at compile time
- The compiler can usually infer the type based on the value and how we use it

---

## Scalar Types

A scalar type represents a single value. Rust has four primary scalar types:

### 1) Integers

Integers are numbers without a fractional component.

**Signed integers (can be negative):**
- `i8`, `i16`, `i32`, `i64`, `i128`, `isize`

**Unsigned integers (only positive):**
- `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

**Understanding:**
- The number after `i` or `u` indicates how many bits of memory it uses
- `isize` and `usize` depend on your computer architecture (32-bit or 64-bit)
- Integers default to `i32`

### 2) Floating-Point Numbers

Numbers with decimal points.

- `f32` - 32-bit floating point
- `f64` - 64-bit floating point (default)

**Understanding:**
- Use `:.N` in println to format to N decimal places
- `f64` is more precise than `f32`
- Floating-point numbers default to `f64`

### 3) Boolean

The boolean type has two possible values: `true` and `false`.

**Understanding:**
- Used in conditionals (if statements)
- Takes up 1 byte of memory

### 4) Character

The `char` type represents a single Unicode character.

**Understanding:**
- Use single quotes for `char` (double quotes are for strings)
- Can represent any Unicode character, including emoji
- Takes up 4 bytes of memory

---

## Compound Types

Compound types can group multiple values into one type.

### 1) Tuples

A tuple groups together values of different types.

**Understanding:**
- Fixed length - once declared, cannot grow or shrink
- Access elements using `.0`, `.1`, `.2`, etc.
- Can contain different types
- Can destructure to extract values

### 2) Arrays

An array is a collection of multiple values of the same type.

**Understanding:**
- Fixed length
- All elements must be the same type
- Stored on the stack (not heap)
- Use `[type; length]` syntax for type annotation
- Access elements using index notation `[0]`, `[1]`, etc.

---

## Type Annotations

Sometimes you need to explicitly tell Rust what type you want.

**Understanding:**
- Without type annotation, the compiler wouldn't know what type to use in some cases
- Type annotations help when the compiler can't infer the type
- Use `: type` syntax after variable name

---

## Type Conversion

Rust doesn't automatically convert between types. You must be explicit.

**Understanding:**
- Use `as` keyword to cast between numeric types
- Rust is strict about type safety to prevent bugs
- Cannot add different numeric types without conversion

---

## Number Formatting

**Understanding:**
- Use `:.N` to format to N decimal places
- Useful for displaying currency or measurements
- Works with println! macro