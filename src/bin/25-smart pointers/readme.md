# Talking about Smart Pointers

## What are Smart Pointers?

- A pointer is a general concept for a variable that contains an address in memory
- Smart pointers are data structures that act like a pointer but also have additional metadata and capabilities
- Unlike references, smart pointers own the data they point to
- Smart pointers are usually implemented using structs that implement the `Deref` and `Drop` traits

---

## Smart Pointers vs References

### References
- Pointers that only borrow data
- Indicated by the `&` symbol
- Don't have any special capabilities other than referring to data
- No overhead

### Smart Pointers
- Own the data they point to
- Have metadata and additional capabilities
- Implement `Deref` and `Drop` traits
- May have some overhead

---

## Common Smart Pointers in Rust

### 1) `Box<T>`

**What it does:**
- Allocates values on the heap rather than the stack
- Most straightforward smart pointer
- No performance overhead other than storing data on the heap

**When to use:**
- When you have a type whose size can't be known at compile time
- When you have a large amount of data and want to transfer ownership without copying
- When you want to own a value and only care that it implements a particular trait

**Concept:**
- Provides ownership for heap data
- Automatically deallocated when it goes out of scope
- Implements `Deref` trait so you can use it like a reference

---

### 2) `Rc<T>` (Reference Counted)

**What it does:**
- Enables multiple ownership of the same data
- Keeps track of the number of references to a value
- When the count reaches zero, the value is cleaned up
- Only for use in single threaded scenarios

**When to use:**
- When you want to allocate data on the heap for multiple parts of your program to read
- You can't determine at compile time which part will finish using the data last
- Need multiple owners of the same data

**Concept:**
- Immutable references only
- Reference counting has runtime cost
- Not thread safe (use `Arc<T>` for thread safe version)
- Use `Rc::clone()` to create new references

---

### 3) `RefCell<T>`

**What it does:**
- Enforces borrowing rules at runtime instead of compile time
- Allows interior mutability pattern
- Lets you mutate data even when there are immutable references to that data

**When to use:**
- When you're sure your code follows the borrowing rules but the compiler can't guarantee that
- Need to mutate data inside an immutable structure
- Single threaded scenarios only

**Concept:**
- Runtime borrowing checks
- Panics if borrowing rules are violated at runtime
- Use `.borrow()` for immutable borrow
- Use `.borrow_mut()` for mutable borrow
- Commonly used with `Rc<T>` for multiple owners with mutability

---

### 4) `Arc<T>` (Atomic Reference Counted)

**What it does:**
- Like `Rc<T>` but thread safe
- Uses atomic operations for reference counting
- Enables multiple ownership across threads

**When to use:**
- Need shared ownership across multiple threads
- Similar use cases to `Rc<T>` but in concurrent contexts

**Concept:**
- Thread safe version of `Rc<T>`
- Slightly more performance overhead than `Rc<T>`
- Still provides immutable access only
- Combine with `Mutex<T>` or `RwLock<T>` for interior mutability

---

### 5) `Mutex<T>` and `RwLock<T>`

**What they do:**
- Provide interior mutability with thread safety
- `Mutex<T>` allows one thread to access data at a time
- `RwLock<T>` allows multiple readers or one writer

**When to use:**
- Need to mutate data shared across threads
- `Mutex<T>` for simple mutual exclusion
- `RwLock<T>` when you have many readers and few writers

**Concept:**
- Must acquire lock before accessing data
- Automatically released when guard goes out of scope
- Can cause deadlocks if not used carefully
- Often used with `Arc<T>` for shared mutable state

---

## The Deref Trait

**What it does:**
- Allows smart pointers to be treated like regular references
- Enables the dereference operator `*`
- Enables deref coercion (automatic conversion)

**Understanding:**
- Implementing `Deref` lets you customize the behavior of the dereference operator
- Deref coercion converts a reference to a type into a reference to another type
- Works automatically when passing references to functions

---

## The Drop Trait

**What it does:**
- Lets you customize what happens when a value goes out of scope
- Called automatically when a value goes out of scope
- Used to release resources like files, network connections, or heap memory

**Understanding:**
- You can't call `drop` manually (use `std::mem::drop` instead)
- Rust automatically calls `drop` when a value goes out of scope
- Smart pointers use this to clean up heap memory

---

## Interior Mutability Pattern

**What it is:**
- A design pattern that allows you to mutate data even when there are immutable references to that data
- Uses `unsafe` code internally but provides a safe API
- Bends the borrowing rules at runtime instead of compile time

**Types:**
- `RefCell<T>` for single threaded scenarios
- `Mutex<T>` and `RwLock<T>` for multi threaded scenarios
- `Cell<T>` for simple Copy types

---

## Memory Safety Guarantees

**Compile-time checks (default):**
- References and `Box<T>` are checked at compile time
- Zero runtime cost
- Prevents most memory errors

**Runtime checks:**
- `RefCell<T>`, `Mutex<T>`, `RwLock<T>` check at runtime
- Small performance cost
- Panics or blocks if rules are violated

---

## When to Use Each Smart Pointer

- **`Box<T>`** → Single owner, heap allocation, known at compile time
- **`Rc<T>`** → Multiple owners, single threaded, immutable
- **`Arc<T>`** → Multiple owners, multi threaded, immutable
- **`RefCell<T>`** → Single owner, runtime borrow checking, mutable
- **`Cell<T>`** → Single owner, interior mutability for Copy types
- **`Rc<RefCell<T>>`** → Multiple owners, single threaded, mutable
- **`Arc<Mutex<T>>`** → Multiple owners, multi threaded, mutable
- **`Arc<RwLock<T>>`** → Multiple owners, multi threaded, many readers

---

## Weak References and Reference Cycles

### The Problem: Reference Cycles

**What happens:**
- Two `Rc<T>` values reference each other
- Reference count never reaches zero
- Memory is never freed (memory leak)

**Example scenario:**
```rust
// Node A points to Node B
// Node B points to Node A
// Both have reference count of 2
// When both go out of scope, count drops to 1
// Neither is ever deallocated!
```

### The Solution: `Weak<T>`

**What it does:**
- Creates a weak reference that doesn't increase reference count
- Doesn't prevent the value from being dropped
- Must be upgraded to `Rc<T>` before use
- Returns `Option<Rc<T>>` when upgraded

**When to use:**
- Parent-child relationships (child holds weak reference to parent)
- Graphs with cycles
- Observer patterns
- Cache implementations

**Methods:**
- `Rc::downgrade(&rc)` → Creates `Weak<T>` from `Rc<T>`
- `weak.upgrade()` → Returns `Option<Rc<T>>`
- `Rc::weak_count(&rc)` → Number of weak references
- `Rc::strong_count(&rc)` → Number of strong references

---

## Deref Coercion Rules

Rust performs deref coercion automatically in these situations:
1. When passing arguments to functions
2. When returning values from functions
3. When using method syntax

**Three types of deref coercion:**

1. **`&T` to `&U`** when `T: Deref<Target=U>`
   - From immutable reference to immutable reference

2. **`&mut T` to `&mut U`** when `T: DerefMut<Target=U>`
   - From mutable reference to mutable reference

3. **`&mut T` to `&U`** when `T: Deref<Target=U>`
   - From mutable reference to immutable reference
   - Note: Cannot go from `&T` to `&mut U` (would violate borrowing rules)

---

## Cell vs RefCell

### `Cell<T>`

**Characteristics:**
- For `Copy` types only
- No runtime overhead
- Replaces entire value with `set()`
- Gets copy of value with `get()`
- Cannot get references to inner value

**Use when:**
- Working with simple Copy types (integers, bools, etc.)
- Need interior mutability without borrowing
- Want zero runtime cost

### `RefCell<T>`

**Characteristics:**
- For any type
- Runtime borrow checking
- Can borrow references with `borrow()` and `borrow_mut()`
- Panics if borrowing rules violated at runtime
- Tracks active borrows

**Use when:**
- Working with non-Copy types
- Need to borrow references to inner data
- Compiler can't verify safety but you know it's safe