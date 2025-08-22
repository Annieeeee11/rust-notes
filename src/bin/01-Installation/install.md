1) install for mac =>
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

"fk window users"

2) vscode extenstions => rust, codeLLDB

3) terminal commands =>
- cargo init
- cargo new "filename"
- cargo run 
- cargo --version

3.1) to add dependencies
- cargo add {name}

3.2) for library 
- A library is a collection of reusable code that can be used by other programs or libraries. 
- In Rust a library is a compiled version of Rust code that doesn't have a main entry point. 
- It contains functions, structs, and other components that other code can use.

```
cargo init --lib
```

4) for Binaries
- A binary is a program or executable that can be run directly on an operating system. 
- In Rust, a binary is usually a .exe (on Windows) or an executable file (on Unix-based systems like Linux or macOS).
- A binary project in Rust is defined by a file named main.rs (or other files in the src directory). 
- When you compile a binary, Rust generates a standalone executable file. 
- The main purpose of the binary is to be run by the user, typically with some kind of entry point (fn main()).

```
cargo init --bin
```