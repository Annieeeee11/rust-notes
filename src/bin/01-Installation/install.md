# Talking about Rust Installation & Setup for Mac 

## 1) Install for Mac

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> "fk window users"

---

## 2) VSCode Extensions

- **rust**
- **codeLLDB**

---

## 3) Terminal Commands

```bash
cargo init                  # Initialize a new Rust project
cargo new "filename"        # Create a new Rust project
cargo run                   # Build and run the project
cargo --version             # Check cargo version
```

### 3.1) To Add Dependencies

```bash
cargo add {name}
```

### 3.2) For Library

- A library is a collection of reusable code that can be used by other programs or libraries.
- In Rust a library is a compiled version of Rust code that doesn't have a main entry point.
- It contains functions, structs, and other components that other code can use.

```bash
cargo init --lib
```

---

## 4) For Binaries

- A binary is a program or executable that can be run directly on an operating system.
- In Rust, a binary is usually a .exe (on Windows) or an executable file (on Unix based systems like Linux or macOS).
- A binary project in Rust is defined by a file named main.rs (or other files in the src directory).
- When you compile a binary, Rust generates a standalone executable file.
- The main purpose of the binary is to be run by the user, typically with some kind of entry point (fn main()).

```bash
cargo init --bin
```