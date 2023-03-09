

use std::fs::{File, self};
use std::io::Write;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub name: String,
    pub description: String,
    pub link: String,
}

impl Contact {
    pub fn new(args: &[String]) -> Result<Contact, &'static str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }
        let name = args[0].clone();
        let description = args[1].clone();
        let link = args[2].clone();

        Ok(Contact { name, description, link })
    }
    pub fn from(args: [&str; 3]) -> Vec<String>{
        // this will literally not run if the arguments are incorrect
        let input: Vec<String> = [
            String::from(args[0]),
            String::from(args[1]),
            String::from(args[2]),
        ].to_vec();
        input
    }
    pub fn serialize(&self) {
        let serialized = serde_json::to_string(self).unwrap();
        let filename = format!("contacts/{}.json", self.name.replace('\n', ""));
        let mut file = File::create(filename).expect("Unable to create file");
        file.write_all(serialized.as_bytes()).expect("Unable to write data");
    }
    pub fn read(path: String) -> Result<Contact, &'static str>{
        let data = fs::read_to_string(path)
            .expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&data)
        .   expect("JSON does not have correct format.");
        let name = ("{}", json["name"].as_str().unwrap()).1;
        let description = ("{}", json["description"].as_str().unwrap()).1;
        let link = ("{}", json["link"].as_str().unwrap()).1;

        let args = Contact::from([name, description, link]);
        
        return Contact::new(&args);

    }
    pub fn open_link(&self) {
        match open::that(&self.link) {
            Ok(()) => println!("Opened link successfully at URL {}", &self.link),
            Err(_err) => println!("URL was unable to be opened! Try checking if the link is valid.")
        }
    }
}

pub mod cli {
    pub fn show_art() {
        let text = " _______  __   __  _______  __    _  _______  _______  _______  _______  ___   _ 
        |       ||  | |  ||       ||  |  | ||       ||  _    ||       ||       ||   | | |
        |    _  ||  |_|  ||   _   ||   |_| ||    ___|| |_|   ||   _   ||   _   ||   |_| |
        |   |_| ||       ||  | |  ||       ||   |___ |       ||  | |  ||  | |  ||      _|
        |    ___||       ||  |_|  ||  _    ||    ___||  _   | |  |_|  ||  |_|  ||     |_ 
        |   |    |   _   ||       || | |   ||   |___ | |_|   ||       ||       ||    _  |
        |___|    |__| |__||_______||_|  |__||_______||_______||_______||_______||___| |_|";
        println!("{}", text);
    }
}