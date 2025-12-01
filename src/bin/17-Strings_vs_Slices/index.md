# Talking about Strings and Slice

## 1. String

- Heap allocated → data lives on the heap.
- Growable → you can add, remove, or change text.
- Owns its contents → it's responsible for cleaning up memory when it goes out of scope.
- Useful when you need to build, modify, or store a string.

---

## 2. &str (string slice)

- A view into a string, like a window over existing data.
- Can point to:
  - Part of a String
  - A string literal ("hello")
- Immutable → you cannot change it.
- Does not own data → it borrows from somewhere else.

---

## Comparison Table

| Feature          | `String`                             | `&str` (slice)                           |
| ---------------- | ------------------------------------ | ---------------------------------------- |
|  Ownership       | Owns its data (heap)                 | Borrowed reference, doesn't own data     |
|  Mutability      | Can be mutable (`push`, `push_str`)  | Always immutable                         |
|  Where stored    | Heap                                 | Usually points to heap or program binary |
|  When to use     | When you need to build/change string | When you only need to *read* string      |