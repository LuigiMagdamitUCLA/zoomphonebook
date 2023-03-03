#![allow(dead_code)]
#![allow(unused_variables)]

use open;
use serde_json;
use serde::{Serialize, Deserialize};


use std::fs::File;
use std::fs;
use std::io::{BufWriter, Write};
use std::process;
fn main() {
    let args = Contact::from(["Paloma", "SPEAR", "LINK"]);
    let c = Contact::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    c.serialize();
    //open_link(&c);
}

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    pub name: String,
    pub description: String,
    pub link: String,
}

impl Contact {
    fn new(args: &[String]) -> Result<Contact, &'static str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }
        let name = args[0].clone();
        let description = args[1].clone();
        let link = args[2].clone();

        Ok(Contact { name, description, link })
    }
    fn from(args: [&str; 3]) -> Vec<String>{
        // this will literally not run if the arguments are incorrect
        let input: Vec<String> = [
            String::from(args[0]),
            String::from(args[1]),
            String::from(args[2]),
        ].to_vec();
        input
    }
    fn serialize(&self) {
        let serialized = serde_json::to_string(self).unwrap();
        let filename = format!("contacts/{}.json", self.name);
        let mut file = File::create(filename).expect("Unable to create file");
        file.write_all(serialized.as_bytes()).expect("Unable to write data");
    }
}

fn open_link(contact: &Contact) {
    match open::that(&contact.link) {
        Ok(()) => println!("Opened"),
        Err(_err) => println!("Dead!")
    }
}

// fn save_contact(contact: &Contact) {
//     let serialized = serde_json::to_string(&contact).unwrap();
//     dbg!(&serialized);
//     let filename = format!("contacts/{}.json", &contact.name);
//     let mut f = File::create(filename).expect("Unable to create file");
//     f.write_all(serialized.as_bytes()).expect("Unable to write data");
// }