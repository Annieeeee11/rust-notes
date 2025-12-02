# Talking about Lifetimes

## What are Lifetimes?

- A lifetime is a construct the compiler (or more specifically, its borrow checker) uses to ensure all borrows are valid.

- we only have to deal with this when we are borrowing strings

- used with Anchor for sure :))

---

## Understand with an example

```rust
fn main() {
    let str1 = String::from("hello1");
    let ans;
    {
    let str2 = String::from("hey2");
    let ans = long_string(&str1, &str2);
    }
    println!("{}", ans)
}
```

### The Problem:

- eg  =>  in the above case if the str2 is greater than str1 then where will ans point while printing.
        cause str2 scops ends and then ans prints so is answer is str2 what will it print?
        cause str2 wil be cleand from the memory by then so that will become a dangling pointer

- what rust compiler think => so rust things other code can also some how cause the same memory issues while
                            borrow code so it implements/specify lifetime and stop the code, now we can you
                            use the lifetime concept the write code and that will cause no issues

---

## Solution

- to solve this problem rust introducted lifetimes everytime someone borrow they need to specify the
  lifetime of the borrow using lifetime annotation using generic parameters syntax
  => 'a  ,  'b  ,  't  ,  <'a , 'b>

- They don't create new lifetimes, they only name existing ones so the compiler can relate them

---

## Solution Example

### How to use:
- first define the parameters using the generic syntax i front of function 'a  ,  'b
  these ' are used infront of variables to specify that it is used for the lifetime
  using <> we will define parameters and then in the () we will specify what parameters
  points to what variable and at the end we will define what variable's lifetime
  will the return type follow

**STEP 1:**

```rust
fn test<'a , 'b>() {
    // code
}
```

**STEP 2:**

```rust
fn test<'a , 'b>(s1: &'a String, s2: &'b String) {
    // code
}
```

**STEP 3:**

```rust
fn test<'a , 'b>(s1: &'a String, s2: &'b String) -> &'a String {
    // code
}
```

---

## Detailed Solution Example

```rust
fn main() {
    let str1 = String::from("hello1");
    let ans;
    {
    let str2 = String::from("hey2");
    let ans = long_string(&str1, &str2);
    }
    println!("{}", ans);
}

fn long_string<'a , 'b>(s1: &'a String, s2: &'b String) -> &'a String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

### Case 1:
- the code will compiler here casue the lifetime is specified to s1 lifetime so it will let
  the ans print cause s1 life time is longer

**This means:**
- Function takes two references (s1, s2) with lifetimes 'a and 'b.
- It returns a reference that is guaranteed to live as long as 'a.

- you can only safely return s1 here so if you try to return s2, Rust will complain unless 'b outlives 'a.

```rust
fn main() {
    let str1 = String::from("hello1");
    let ans;
    {
    let str2 = String::from("hey2");
    let ans = long_string(&str1, &str2);
    }
    println!("{}", ans);
}

fn long_string<'a , 'b>(s1: &'a String, s2: &'b String) -> &'a String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

### Case 2:
- the compiler will throw error casue it wil say that s2 lifetime is very sort and it makes
  a smart decision to not run ans because str2 life span is short and return value isnt valid

- you promised (-> &'a String) that the return will be valid as long as 'a. Returning s2
  (shorter lifetime) breaks that promise

---

## Other Ways/Cases

### Case 1:
- in this case if we take a single peremeter ( 'a ) so rust considers the worst case that
  means it considers the smaller lifetime and return that, which means that ans scope is
  till the end of the main function but dont let the user use it after the brackets make its
  lifetime till the brackets only

```rust
fn main() {
    let str1 = String::from("hello1");
    let ans;
    {
    let str2 = String::from("hey2");
    let ans = long_string(&str1, &str2);
    println!("{}", ans);
    }
}

fn long_string<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        return s1;
    }
        return s2;
}
```

### Case 2:
- in a case where we know that the one variable is nevr return or used and the lifetime
  should not be taken from that then we can introduces two annotations there

```rust
fn main() {
    let str1 = String::from("hello1");
    let str2 = String::from("hey2");
    let ans;
    {
    let str3 = String::from("");
    let ans = long_string(&str1, &str2, &str3);
    println!("{}", ans);
    }
}

fn long_string<'a , 'b>(s1: &'a String, s2: &'a String, s3: &'b String) -> &'a String {
    if s1.len() > s2.len() {
        return s1;
    }
        return s2;
}
```

---

## Lifetime with Structs

- lifetime are same with struct so if we pass a reference in a struct then we gotta specify the lifetime too

- When you use references as fields inside a struct, you generally need to add lifetime parameters
  to ensure that references do not outlive the data they point to.

```rust
#[derive(Debug)]
struct User <'a>{
    usernmae: &'a str,
    password: &'a str,
}

fn main() {
    let s1 = String::from("v");
    let s2 = String::from("h");

    let v = User {
        username: &s1,
        password:&s2,
    }

    println!("{:?}", v)
}
```

### Impl blocks

```rust
struct User<'a> {
    name: &'a str
}

impl<'a> User<'a> {
    fn new(name: &'a str) -> Self {
        User { name }
    }
}
fn main() {
   let name = String::from("anaya");
   let user = User::new(&name);
   println!("{}", user.name);
}
```