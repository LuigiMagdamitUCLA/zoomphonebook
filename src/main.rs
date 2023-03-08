#![allow(dead_code)]
#![allow(unused_variables)]

use phonebook::Contact;


use std::process;
fn main() {
    let args = Contact::from(["Palomaurrr", "SPEAR", "fuck"]);
    let c = Contact::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    c.serialize();
    
    Contact::open_link(&c);
}






// fn save_contact(contact: &Contact) {
//     let serialized = serde_json::to_string(&contact).unwrap();
//     dbg!(&serialized);
//     let filename = format!("contacts/{}.json", &contact.name);
//     let mut f = File::create(filename).expect("Unable to create file");
//     f.write_all(serialized.as_bytes()).expect("Unable to write data");
// }