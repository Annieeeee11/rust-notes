Talking about serde ->

- converts a string into struct data -- json data into struct.

- convert struct to string => serialization
- comvert string to struct =>  deserialization


- its a popular serialization and deserialization framework/library in Rust. 
- It provides a way to convert Rust data structures into different formats (serialization) and vice versa (deserialization). 
- The most common use cases involve working with formats like JSON, YAML, TOML, and others.

---- install ----

- cargo add serde  => this helps to provide the serialization and deserialization structure
- cargo add serde_json  => this helps in formate

- Use serde_json alone → if you only want quick parsing into dynamic JSON (Value).
- Use serde + serde_json → if you want to work with your own Rust structs/enums in JSON.

---- Dependencies ----

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

--- extra ---

- to use macro we need to add the derived feature in serde

[dependencies]
serde = { version = "1.0", features = ["derive"]}