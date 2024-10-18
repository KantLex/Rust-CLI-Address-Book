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

fn add_contact(contacts: $mut Vec<Contact>) {
    println!("\nAdd a New Contact");

    let name = read_input("Name: ");
    let phone = read_input("Phone: ");
    let email_input = read_input("Email (optional): ");

    let emial = if email_input.is_empty() {
        None
    } else {
        Some(email_input)
    };

    let contact = Contact {
        name,
        phone,
        email,
    };

    contacts.push(contact);

    println!("Contact added!");
}

fn read_input(prompt: &str) -> String {
    println!(
        "{}",
        prompt
    );
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}


