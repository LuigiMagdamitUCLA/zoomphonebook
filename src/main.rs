use phonebook::cli::show_art;
use phonebook::commands::*;
use std::env;
use glob::glob;
use std::fs::{File};
use std::io::Write;
use serde::{Serialize, Deserialize};
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
        let strings = ["a".to_owned(), "b".to_owned(), "c".to_owned(), "d".to_owned()].to_vec();
        let a = Appointment::new(&strings);
        a.unwrap().serialize();
        // dbg!(a.unwrap());
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Appointment {
    pub name: String,
    pub description: String,
    pub day: String,
    pub time: String
}

impl Appointment {
    pub fn new(args: &[String]) -> Result<Appointment, &'static str> {
        if args.len() != 4 {
            return Err("Not enough arguments")
        }
        let name = args[0].clone();
        let description = args[1].clone();
        let day = args[2].clone();
        let time = args[3].clone();

        Ok( Appointment { name: name, description: description, day: day, time: time })
    }
    pub fn from(args: [&str; 4]) -> Vec<String> {
        let input: Vec<String> = [
            String::from(args[0]),
            String::from(args[1]),
            String::from(args[2]),
            String::from(args[3]),
        ].to_vec();
        input
    }
    pub fn serialize(&self) {
        let serialized = serde_json::to_string(self).unwrap();
        let filename = format!("./appointments/{}.json", self.name.replace('\n', ""));
        let mut file = File::create(filename).expect("Unable to create file");
        file.write_all(serialized.as_bytes()).expect("Unable to write data");
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
