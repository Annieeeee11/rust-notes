# Talking about Structs

## What are Structs?

- A struct (structure) is a custom data type that lets you group related data together
- Similar to objects in JavaScript or classes in other languages
- Structs let you name and package together multiple related values

---

## Defining a Struct

```rust
pub struct User {
    pub name: String,
    pub id: u32,
    pub email: String,
    pub age: u32,
}
```

**Understanding:**
- `struct` keyword defines a new struct
- `pub` makes the struct and its fields public (accessible from other modules)
- Each field has a name and a type

---

## Creating Struct Instances

```rust
fn main() {
    let user1 = User {
        name: String::from("John"),
        id: 1,
        email: String::from("john@example.com"),
        age: 20,
    };
    
    println!("{}, {}, {}, {}", user1.name, user1.id, user1.email, user1.age);
}
```

**Understanding:**
- Create an instance by specifying values for each field
- Access fields using dot notation: `user1.name`

---

## Implementing Methods

```rust
impl User {
    pub fn get_info(&self) {
        println!("User: {}, ID: {}", self.name, self.id);
    }
}
```

**Understanding:**
- `impl` block lets you define methods for a struct
- `&self` refers to the instance the method is called on
- Methods can access struct fields using `self.field_name`

---

## Key Concepts

- **Ownership**: Structs own their data
- **Methods**: Functions associated with a struct type
- **Associated functions**: Functions in `impl` block that don't take `self` (like constructors)
- **Visibility**: Use `pub` to make structs and fields public 