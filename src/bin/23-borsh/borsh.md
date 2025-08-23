Talking about Borsh ->

<!-- need for solana dev -->
---------- for solana ----------

- It’s a serialization library (like serde, but designed specifically for blockchain use cases).
- convert struct into bunch of bits 

1) Serialization = convert your Rust struct into a sequence of bytes.
2) Deserialization = convert that byte sequence back into the struct.


- On blockchains like Solana, borsh is preferred because:
1) It produces a deterministic binary format (no ambiguity, no wasted bytes).
2) It’s faster and smaller than JSON/Serde — crucial when every byte costs money (transaction fees, storage).
3) It’s simple: fixed encoding rules, no extra configs.


- Where it’s used
1) Solana programs (smart contracts) → almost all program instructions & account data are encoded/decoded with borsh.
2) NEAR protocol → also uses borsh as the default serialization format.

---------- Define ----------

- Borsh (short for Binary Object Representation Serializer for Hashing) is a deterministic, 
- binary serialization format often used in Rust (and other languages) to encode and decode data in a consistent, unambiguous way

It was originally developed by the [NEAR Protocol](https://near.org/) team for use in smart contracts, but you can use it in any Rust project that needs a fast, predictable serialization layer.


---- install ----

- cargo add borsh


---- Dependencies ----

[dependencies]
borsh = { version = "1.5", features = ["derive"]}
