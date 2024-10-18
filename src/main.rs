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
        match menu() {
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

