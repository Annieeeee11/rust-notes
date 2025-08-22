Talking about macro -

- macro are run during the compile time 

- META PROGRAMMING => a program being able to another program
- if you use marco your rust file gets compiles to a different rust file which is what then finally gets converts to the binary -- you write one line of code that converts into multiple lines of code -- code write more code 

- when we add ! its not a function it is a macro 

- to write a lot of code for doing small stuff will look ugly and will be difficult so they created macro that to make big code easy and small of our use case using matea programming 

- eg -> println!, vec!, panic!, etc 

------ expand code ------

- to see the expanded we need to add a crate => cargo install cargo-expand

- and then run cargo expand in the terminal so what it do is it expands the code that is writing in the file example if you use println then you cansee what it actually look like and we would have to type if we didnt used macros

- you can use () or [] with macro to make function works 


-------- TYPES OF MACRO ------

1) Declerative macro -> replace the code writing with a different code during compile time -- println, vec, etc 


2) Procedural macro -> allows you to define custom behavior for code -- they operates on rust abstract syntax tree (ast) -- are commonly used for things like deriving traits automactically or creating custom attributes   -- eg #[derive(Debug)]

println!("{:?}")  -> debug -- see everything debug it (vector, struct, enum etc)
println!("{}")  -> display -- display in pretty manner (int, string, float etc)

- Procedural macro - make life easy we can write traits manully instead of deriving but thaat makes our life difficult so its better to use this macro 

- manual way without procedural

```
use std::{Formatter, Debug};
 
struct User {
    username: String,
}
// this debug tells the println how to print the struct
impl Debug for User {
// this is important function we have to make it because it is present in debug trait

    fn fmt(&self,f: &mut Formatter<'_> ) -> std::fmt::Result {

// write is a macro with tell to print he value in a formatted way 

        write!(f,"{}", self.username)
    }
// the result in this function is different its not the result enum it doesnt return ok , err
}
```


---- there are tree types of macro in procedural macro ----

1) custom macro
- allows you to define how rust erives certain traits for types.

```
#[derive(Debug)]
struct User {
   username: String,
}
```


2) Attributes macro
- mostly used in http server
- it just takes a attribute/macro instead of a trait eg->

```
#[rout("GET")]
fn home() {
    println!("hello);
}

#[rout("POST")]
fn page() {
    println!("page");
}
```


3) function macro
- 
