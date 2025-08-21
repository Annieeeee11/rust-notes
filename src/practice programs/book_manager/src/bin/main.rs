use std::io;

// book structure
struct Book {
    title: String,
    author: String,
    price: u32,
    id: u32,
}

//command enum
enum Commands {
    Add,
    Search,
    List,
    Remove,
    Exit,
}

//return the commands to avoid any errors
impl Commands {
    fn from_input(input: &str) -> Option<Commands> {
        match input.trim().to_lowercase().as_str() {
            "add" => Some(Commands::Add),
            "list" => Some(Commands::List),
            "search" => Some(Commands::Search),
            "remove" => Some(Commands::Remove),
            "exit" => Some(Commands::Exit),
            _ => None,
        }
    }
}

// input function
fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

// library that takes book structure
struct Library {
    book: Vec<Book>,
}

// library function
impl Library {

// normal constructor 
    /* fn new() -> Self {
        Self { books: Vec::new() }
    } */


// default trait 
    fn default() -> Self {
        Library { book: Vec::new() }
    }


// add
    fn add_book(&mut self) -> Result<(), String> {
        println!("enter book title: ");
        let title = user_input();
        println!("enter author: ");
        let author = user_input();
        println!("enter price: ");
        let price: u32 = user_input()
            .parse()
            .map_err(|_| "Invalid price".to_string())?;
        let id = (self.book.len() + 1) as u32;
        self.book.push(Book { id, title, author, price });
        println!("Book added successfully!");
        Ok(())
    }


// search
    fn search_book(&self) {
        println!("enter book title: ");
        let search_input = user_input().to_lowercase();
        let result = self.book.iter().find(|b| b.title.to_lowercase() == search_input);

        match result {
            Some(b) => println!("book -> ID: {}, Title: {}, Author: {}, Price: {}", b.id, b.title, b.author, b.price),
            None => println!("book not found"),
        }
    }


// list
    fn list_books(&self) {
        if self.book.is_empty() {
            println!("no books in library.");
        } else {
            for b in &self.book {
                println!("ID: {}, Title: {}, Author: {}, Price: {}", b.id, b.title, b.author, b.price);
            }
        }
    }


// remove
    fn remove_book(&mut self) -> Result<(), String> {
        println!("Enter book ID to remove:");
        let id: u32 = user_input()
        .parse()
        .map_err(|_| "Invalid ID".to_string())?; // Changes the error type into a String(return user-friendly messages, not scary Rust errors)

        if let Some(pos) = self.book.iter() // gives us an iterator over all books in self.book
        .position(|b| b.id == id) { // goes through each book and checks -- If found at index i then returns Some(i) -- returns None
            self.book.remove(pos);
            println!("Book removed successfully!");
            Ok(())
        } else {
            Err("Book ID not found".to_string()) 
//.to-string() Because Rust error handling usually works with Result<T, String>. We turn "Invalid ID" (a string literal) into an owned String
        }
    }
}

fn main() {
/* this can be like a way to make a object */
    // let my_library = Library { books: Vec::new() };


/* using constructor (new function will be made in impl library) */
    // let library = Library::new();


/* using deafult trait in rust */
    let mut library = Library::default();

    loop {
        println!("Enter command => add, search, list, remove, exit");
        let command = user_input();

        match Commands::from_input(&command) {
            Some(Commands::Add) => {
                if let Err(e) = library.add_book() {
                    println!("Error: {}", e);
                }
            }
            Some(Commands::List) => library.list_books(),
            Some(Commands::Search) => library.search_book(),
            Some(Commands::Remove) => {
                if let Err(e) = library.remove_book() {
                    println!("Error: {}", e);
                }
            }
            Some(Commands::Exit) => {
                println!("Goodbye!");
                break;
            }
            None => println!("Invalid command, try again."),
        }
    }
}
