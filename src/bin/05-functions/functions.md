# Talking about Functions

## Concepts

- the "main" function, which is the entry point of many programs
- "fn" keyword allows you to declare new functions.

## Naming Convention

- uses snake case as the conventional style for function and variable names ( lowercase, underscore, eg: user_example)

---

## Function Types

### 1) Basic Function

```rust
fn main() {
    println!("hii");
}

fn sum() {
    println!("hii");
}
```

### 2) Function with Arguments

```rust
fn sum(a: u32, b: u32) {
    a + b  // if i simple give the last thing without a ; (not make it a statement) then function return it automatically
}
```

### 3) Function with Return Type

```rust
fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}
```

**Understanding:**
- Use `->` to specify return type
- Can use `return` keyword or just the last expression without semicolon

### 4) Public Functions

```rust
pub fn xyz() {
    // code
}
```

- `pub` --> add if you want to access them from outside.

### 5) Functions Returning Unit Type

```rust
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}
```

- Functions that don't return a value --> return the unit type `()`
- When a function returns `()` --> the return type can be omitted from the signature

---

## Associated Functions vs Methods

### Associated Functions

- Functions that are defined on a type generally
- Don't need to be called with an instance
- Generally used like constructors
- Called using double colons `::`

```rust
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

fn main() {
    let p = Point::origin();  // Associated function call
}
```

### Methods

- Associated functions that are called on a particular instance of a type
- Take `&self` as first argument
- Called using dot operator `.`

```rust
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    // This method requires the caller object to be mutable
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    
    println!("Area: {}", rectangle.area());  // Method call
}
```

**Understanding:**
- `&self` is sugar for `self: &Self`, where `Self` is the type of the caller object
- Methods are called using the dot operator
- `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`

---

## Common String Methods

### `.trim()`

- Removes leading and trailing whitespace (like spaces, tabs, and newlines) from a string
- When reading user input, the result often includes a ` ` (newline) at the end

```rust
let input = "  42 ";
let trimmed = input.trim(); // "42"
```

### `.parse::<T>()`

- Converts a string into another type eg like i32, u32, f64, etc.
- Returns a `Result<T, Err>` meaning it may succeed or fail

```rust
let num_str = "42";
let number: u32 = num_str.parse().unwrap(); // Converts "42" → 42
```

### `.expect("message")`

- Used on Result or Option types to unwrap the value or panic with a custom error message
- Extract value or panic with message

```rust
let input = "42";
let number: u32 = input.parse().expect("Please enter a valid number");
```

### `.unwrap()`

- This unwraps the returns, when the developer is sure that something will return something
- This should return or will crash the error
- It should not be used except a few situations like: when project starts you connect to a database. if the db doesn't exist it is good to crash

```rust
let number: u32 = "42".parse().unwrap(); // Ok
let number: u32 = "hi".parse().unwrap(); // panics!
```

**Alternatives:**
- `.expect("your message")` better for debugging
- Pattern matching (match, if let)
- `?` operator (in functions that return Result)