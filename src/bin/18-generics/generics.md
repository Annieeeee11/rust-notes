# Talking about Generics

## What are Generics?

1) Generics is Writing Code That Works for Many Types

---

## Key Concepts

- Generics allow you to write flexible, reusable code that works with multiple types
- Use `<T>` syntax to define a generic type parameter
- The compiler generates specific versions for each type you use

## Example

```rust
fn sum<T>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let a = sum(1, 2);
    println!("{}", a);
}
```

**Understanding:**
- `<T>` means "this function works for any type T"
- The function can work with integers, floats, or any type that supports the `+` operation