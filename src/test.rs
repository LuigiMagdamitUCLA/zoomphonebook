use crate::contact::Contact;

pub fn test() {
    let t: Contact = Contact {
        name: String::from(""),
        description: String::from(""),
        link: String::from(""),
    };
    dbg!(t);
}