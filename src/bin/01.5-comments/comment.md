# Talking about Comments

Rust supports three types of comments to document and annotate your code.

## 1. Single-Line Comments

Use `//` for single-line comments:

```rust
// This is a single-line comment
let x = 5; // Comments can also appear at the end of a line
```

## 2. Multi-Line Comments

Use `/* */` for multi-line comments:

```rust
/* This is a 
   multi-line comment 
   spanning multiple lines */
```

## 3. Nested Comments

Rust allows nested comments, which is useful for commenting out blocks of code:

```rust
/* This outer comment can contain code
   println!("This will be ignored");
   /* This is a nested comment! */
   All of this is commented out
*/
```

---

**Note:** For documentation comments that generate API documentation, use `///` (for items) or `//!` (for modules).
