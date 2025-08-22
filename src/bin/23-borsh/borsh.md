Talking about Borsh ->

- Borsh (short for Binary Object Representation Serializer for Hashing) is a deterministic, 
- binary serialization format often used in Rust (and other languages) to encode and decode data in a consistent, unambiguous way

It was originally developed by the [NEAR Protocol](https://near.org/) team for use in smart contracts, but you can use it in any Rust project that needs a fast, predictable serialization layer.

---- install ----

- cargo add borsh

---- Dependencies ----

[dependencies]
borsh = { version = "1.5", features = ["derive"]}

