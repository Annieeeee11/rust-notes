# Talking about Enums

## What is an Enum?

- A type that can be one value out of a fixed set of possible variants.
  or maybe Represents a choice or state.

## When to Use Enums

- use it When you want to limit a variable to a few possible options and handle them explicitly.

---

## Differences from Other Languages

- This is why enums in Rust are more powerful than C style enums, they can be both a choice and a data container

## Enums vs Structs

- enums can store data too in rust and the difference in struct and enum is that you can choose only one of the
  value present in enum likhe i have 2 values north or south then i can only choose north yaa south
  and the data will store according, like the tag will be stored (north or south) and if any data related to that

---

## Using Match with Enums

- "match" isoften used with enums, because it lets you handle each variant of the enum explicitly.
  Some and None are enum variants of Option<T> that you often use inside a match.