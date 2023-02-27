#![allow(dead_code)]
#![allow(unused_variables)]

use open;
use serde_json;

use contact::Contact;
use std::fs::File;
use std::fs;
use std::io::{BufWriter, Write};
mod contact;

fn main() {
    let c = return_contact(String::from("Paloma"), 
        String::from("SPEAR Counselor"), 
        String::from("https://www.google.com/"));

    dbg!(&c.link);
    //open_link(&c);
    save_contact(&c);
}

fn return_contact(name: String, description: String, link: String) -> Contact {
    Contact{
        name: name,
        description: description,
        link: link
    }
}

fn open_link(contact: &Contact) {
    match open::that(&contact.link) {
        Ok(()) => println!("Opened"),
        Err(_err) => println!("Dead!")
    }
}

fn save_contact(contact: &Contact) {
    let serialized = serde_json::to_string(&contact).unwrap();
    dbg!(&serialized);
    let filename = format!("contacts/{}.json", &contact.name);
    let mut f = File::create(filename).expect("Unable to create file");
    f.write_all(serialized.as_bytes()).expect("Unable to write data");
}