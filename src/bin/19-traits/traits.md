# Talking about Traits

## What are Traits?

- traits are similar to interfaces implement in class in javascript

- everything that use a traits it impose a specific structure that should be followed

<!-- - Generics = "this code works for any type T."
fn max<T>(a: T, b: T) -> T { ... } -->

---

## How Traits Work

- Define a trait with required methods
- Implement the trait for different types
- Each type must provide its own implementation

## Example

```rust
trait Shape {
    fn area(&self) -> u32;
}

struct Rect {
    width: u32,
    height: u32,
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

struct Circle {
    round: u32,
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        return self.round * self.round;
    }
}
```

---

## Trait Bounds with Generics

- You can combine traits with generics to specify what capabilities a type must have

```rust
fn get_area<T: Shape>(s: T) -> u32 {
    return s.area();
}
```

**Understand:**
- `<T: Shape>` means "T can be any type, but it must implement the Shape trait"
- This ensures the type has the required methods (like `area()`)