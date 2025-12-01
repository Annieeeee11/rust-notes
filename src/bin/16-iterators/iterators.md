# Talking about Iterators

## What is an Iterator?

- iterator in Rust is just something that lets you go through a sequence of items one by one.

- for loop also use iterators to make code work

---

## Under the hood, an iterator is just something with a .next() method

### Core of Iterators: next()

```rust
{
    let numbers = vec![10, 20, 30];
    let mut iter = numbers.iter();

    println!("{:?}", iter.next()); // Some(10)
    println!("{:?}", iter.next()); // Some(20)
    println!("{:?}", iter.next()); // Some(30)
    println!("{:?}", iter.next()); // None (no more items) 

    // Each call to .next() gives you Option<T>: Some(value) if there's an item ---- None when it's finished
}
```

---

## How is it different from normal loops?

### Lazy evaluation
- Iterators don't do work until you ask.

```rust
{
   let nums = vec![1, 2, 3];
   let doubled = nums.iter().map(|x| x * 2);
   // Nothing happens yet    
}
```

- Only when you collect or loop do they actually run:

```rust
{
   let result: Vec<_> = doubled.collect();
   println!("{:?}", result); // [2, 4, 6]  
}
```

### Functional style

```rust
{
    // Instead of for + manual push:
    let mut doubled = Vec::new();
    for x in &nums {
        doubled.push(x * 2);
    }

    // you can write 
    let doubled: Vec<_> = nums.iter().map(|x| x * 2).collect();  
}
```

---

## Iterators Type

### Iter
If you want immutable
references to the inner
variables and don't want
to transfer ownership

### IterMut
If you want mutable
references to the inner
variables and don't want
to transfer ownership

### IterInto
If you want to move the
variable into the iterator
and don't want to use it
afterwards

---

## Two kinds of iterator

### Iterator Adaptors
- transform an iterator into another iterator. returns another iterator by changing the original iterator a bit

**Examples:**
- .map()
- .filter()
- .take()
- .enumerate()

### Consuming Adaptors
- run through the iterator and produce a final value. basically you cant use the vector anymore cause it takes/consume it

**Examples:**
- .collect()
- .sum()
- .count()
- .for_each()