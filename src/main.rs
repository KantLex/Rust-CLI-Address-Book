#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
    email: Option<String>,
}

enum MenuChoice {
    Add,
    List,
    Search,
    Quit,
}

use std::io::{self, Write};

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        match show_menu() {
            MenuChoice::Add => add_contact(&mut contacts),
            MenuChoice::List => list_contacts(&contacts),
            MenuChoice::Search => search_contacts(&contacts),
            MenuChoice::Quit => {
                println!("Goodbye!");
                break;
            }
        }
    }
}

fn show_menu() -> MenuChoice {
    println!("\nAddress Book:");
    println!("1. Add contact");
    println!("2. List contacts");
    println!("3. Search contacts");
    println!("4. Quit");
    println!("Enter your choice:");
    io::std out().flush().expect("Failed to flush stdout");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => MenuChoice::Add,
        "2" => MenuChoice::List,
        "3" => MenuChoice::Search,
        "4" => MenuChoice::Quit,
        _ => {
            println!("Invalid choice");
            show_menu()
        }
    }
}



