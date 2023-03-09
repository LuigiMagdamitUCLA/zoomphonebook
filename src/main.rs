#![allow(dead_code)]
#![allow(unused_variables)]
extern crate colored;

use phonebook::Contact;

use std::env;
use std::process;

use colored::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == &String::from("create") {
        show_art();
        enter_contact_info();
    }
}

fn new_contact(args: Vec<String>) -> Contact {
    let c = Contact::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    return c;
}

fn enter_contact_info() {
    let mut name_line = String::new();
    let mut desc_line = String::new();
    let mut link_line = String::new();

    println!("Enter contact name");
    let b1 = std::io::stdin().read_line(&mut name_line).unwrap();
    println!("Enter description");
    let b2 = std::io::stdin().read_line(&mut desc_line).unwrap();
    println!("Enter link");
    let b3 = std::io::stdin().read_line(&mut link_line).unwrap();

    let args: [String; 3] = [name_line, desc_line, link_line];
    let args_vec: Vec<String> = args.to_vec();

    let contact: Contact = new_contact(args_vec);

    contact.serialize();
}
fn show_art() {
    let text = (" _______  __   __  _______  __    _  _______  _______  _______  _______  ___   _ 
    |       ||  | |  ||       ||  |  | ||       ||  _    ||       ||       ||   | | |
    |    _  ||  |_|  ||   _   ||   |_| ||    ___|| |_|   ||   _   ||   _   ||   |_| |
    |   |_| ||       ||  | |  ||       ||   |___ |       ||  | |  ||  | |  ||      _|
    |    ___||       ||  |_|  ||  _    ||    ___||  _   | |  |_|  ||  |_|  ||     |_ 
    |   |    |   _   ||       || | |   ||   |___ | |_|   ||       ||       ||    _  |
    |___|    |__| |__||_______||_|  |__||_______||_______||_______||_______||___| |_|").bold();
    println!("{}", text);
}