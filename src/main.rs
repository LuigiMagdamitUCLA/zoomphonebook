#![allow(dead_code)]
#![allow(unused_variables)]


use phonebook::Contact;
use phonebook::cli::show_art;
use phonebook::commands::*;
use std::env;
use std::process;
use glob::glob;

fn main() {

    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let spec = if args.len() == 3 {
        Some(&args[2])
    } else {
        None
    };
    show_art();
    if command == &String::from("create") {
        enter_contact_info();
    }
    if command == &String::from("show") {
        show_contacts();
    }
    if command == "read" {
        match spec {
            Some(spec_final) => command_read(spec_final),
            _ => println!("Needs second argument!")
        }
        
    }
    if command == "start" {
        //command_start(spec);
    }
    if command == "help" {
        command_help();
    }
}

fn new_contact(args: &[String]) -> Contact {
    let c = Contact::new(args).unwrap_or_else(|err| {
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

    let contact: Contact = new_contact(&args);

    contact.serialize();
}

fn show_contacts() {
    for entry in glob("./contacts/*.json").expect("msg") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}
