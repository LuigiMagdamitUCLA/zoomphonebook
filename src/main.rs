use phonebook::cli::show_art;
use phonebook::commands::*;
use std::env;
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
        match spec {
            Some(spec_final) => command_start(spec_final),
            _ => println!("Needs second argument!")
        }
    }
    if command == "help" {
        command_help();
    }
    if command == "a" {
        enter_appointment_info();
    }
}
fn show_contacts() {
    for entry in glob("./contacts/*.json").expect("msg") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}
