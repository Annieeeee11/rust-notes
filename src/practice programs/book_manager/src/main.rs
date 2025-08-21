use std::io;

struct Book{  // book structure
    title: String,
    author: String,
    price: u32,
    id: u32,
}

// enum Commands { //command enum
//     Add,
//     Search,
//     List,
//     Remove,
//     Exit,
// }

fn user_input() -> String { // input function
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn add_book(library: &mut Vec<Book>) {
    println!("Enter book title: ");
    let title = user_input();
    println!("Enter author: ");
    let author = user_input();
    println!("Enter price: ");
    let price: u32 = user_input().parse().unwrap_or(0);
    let id = library.len() as u32 + 1; // simple auto ID
    let book = Book { title, author, price, id };
    library.push(book);
    println!("Book added!");
}

fn list_books(library: &Vec<Book>) {
    for book in library {
        println!("ID: {}, Title: {}, Author: {}, Price: {}", book.id, book.title, book.author, book.price);
    }
}

fn search_book(library: &Vec<Book>) {
    println!("Enter book title to search:");
    let search_input = user_input();
    let search_input = search_input.to_lowercase();
    for book in library {
        if book.title.to_lowercase() == search_input {
            println!("ID: {}, Title: {}, Author: {}, Price: {}", book.id, book.title, book.author, book.price);
        }
    }
}

fn remove_book(library: &mut Vec<Book>) {
    println!("Enter book title to delete:");
    let search_input = user_input();
    for book in library {
        if book.title.to_lowercase() == search_input {
            book.price.remove();
            book.id.remove();
            book.author.remove();
            book.title.remove();
        }
    }
}

fn main() {
    let mut library: Vec<Book> = Vec::new(); // We’ll store multiple books here

    loop {
        println!("Enter command => add, Search, List, Remove, Exit");
        let command = user_input(); 
        match command.to_lowercase().as_str() {
            "add" => add_book(&mut library),
            "search" => search_book(&library),
            "list" => list_books(&library),
            "remove" => remove_book(&mut library),
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
}