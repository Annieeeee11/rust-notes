# Talking about Option Enum

## What is Option?

- The Option enum was introduced to handle the concept of nullability safely in Rust.
- Unlike languages like JavaScript which have `null` or `undefined`, Rust does NOT have null.
- Instead, Rust uses the Option enum to represent the possibility of absence of a value.

---

## Key Points:

- Option handles null-like behavior in Rust.
- Use Option when a returned value might not exist.
- Instead of returning null, Rust functions return Option<T>.

---

## Option Enum Structure

```rust
pub enum Option<T> {
    None,
    Some(T)
}
```

**Two variants:**
- `Some(T)` --> contains a value of type T
- `None` --> represents absence of a value

---

## Example Usage

### Custom Option Enum:

```rust
enum Option1 {
    Some(u32),
    None,
}

fn testfunction1(str: String) -> Option1 {
    return Option1::Some(2);
    // or return Option1::None;
}

fn main() {
    let ans = testfunction1(String::from("hello"));
    match ans {
        Option1::Some(val) => println!("Found: {}", val),
        Option1::None => println!("Nothing found"),
    }
}
```

### Using Built-in Option:

```rust
fn first_a(s: String) -> Option<u32> {
    let mut index = 0;
    for c in s.chars() {
        index = index + 1;
        if c == 'a' {
            return Some(index);
        }
    }
    None
}

fn main() {
    let result = first_a(String::from("hello"));
    match result {
        Some(location) => println!("Found 'a' at position: {}", location),
        None => println!("Character 'a' not found")
    }
}
```

---

## Why Use Option?

- **Safety**: Compiler forces you to handle the `None` case
- **No null pointer errors**: Can't accidentally use a null value
- **Explicit**: Makes it clear when a value might be absent
- **Pattern matching**: Easy to handle both cases with `match`