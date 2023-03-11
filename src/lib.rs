

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
    pub fn print_data(&self) {
        println!("
            Name: {}
            Description: {},
            Link: {},
        ", self.name, self.description, self.link);
    }
    pub fn serialize(&self) {
        let serialized = serde_json::to_string(self).unwrap();
        let filename = format!("contacts/{}.json", self.name.replace('\n', ""));
        let mut file = File::create(filename).expect("Unable to create file");
        file.write_all(serialized.as_bytes()).expect("Unable to write data");
    }
    pub fn read(path: &str) -> Result<Contact, &'static str>{
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
        let filename = format!("./appointments/{}-{}-{}.json", self.name.replace('\n', ""), self.day.replace('\n', ""), self.time.replace('\n', ""));
        let mut file = File::create(filename).expect("Unable to create file");
        file.write_all(serialized.as_bytes()).expect("Unable to write data");
    }
    pub fn print_data(&self) {
        println!("
            Name: {}
            Description: {},
            Day: {},
            Time: {},
        ", self.name, self.description, self.day, self.time);
    }
}

pub mod commands {
    pub fn command_read(spec: &str) {
        let path = format!("./contacts/{}.json", spec); 
        let contact_read = super::Contact::read(&path);
        
        match contact_read {
            Ok(contact_success) => contact_success.print_data(),
            Err(err) => println!("{}", err)
       }
    }
    pub fn command_start(spec: &str) {
        let path = format!("./contacts/{}.json", spec); 
        let contact_read = super::Contact::read(&path);

        match contact_read {
            Ok(contact_success) => contact_success.open_link(),
            Err(err) => println!("{}", err)
       }
    }
    pub fn command_help() {
        println!("These are the commands \n
            create:\tto enter contact creation prompt\n
            read [name]:\t to access contact file at [name]\n
            show:\t to show all contact files\n
            start [name]:\t to open the zoom link at [name]\n");
    }
    pub fn new_contact(args: &[String]) -> super::Contact {
        let c = super::Contact::new(args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            std::process::exit(1);
        });
        return c;
    }
    pub fn new_appointment(args: &[String]) -> super::Appointment {
        let c = super::Appointment::new(args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            std::process::exit(1);
        });
        c.print_data();
        return c;
    }
    pub fn enter_contact_info() {
        let mut name_line = String::new();
        let mut desc_line = String::new();
        let mut link_line = String::new();
    
        println!("Enter contact name");
        std::io::stdin().read_line(&mut name_line).unwrap();
        println!("Enter description");
        std::io::stdin().read_line(&mut desc_line).unwrap();
        println!("Enter link");
        std::io::stdin().read_line(&mut link_line).unwrap();
    
        let args: [String; 3] = [name_line, desc_line, link_line];
    
        let contact: super::Contact = new_contact(&args);
    
        contact.serialize();
    }

    pub fn enter_appointment_info() {
        let mut name_line = String::new();
        let mut desc_line = String::new();
        let mut day_line = String::new();
        let mut time_line = String::new();
    
        println!("Enter contact name");
        std::io::stdin().read_line(&mut name_line).unwrap();
        println!("Enter description");
        std::io::stdin().read_line(&mut desc_line).unwrap();
        println!("Enter day [MM-DD-YYYY]");
        std::io::stdin().read_line(&mut day_line).unwrap();
        println!("Enter time [24:00]");
        std::io::stdin().read_line(&mut time_line).unwrap();
        
    
        let args: [String; 4] = [name_line, desc_line, day_line, time_line];
    
        let contact: super::Appointment = new_appointment(&args);
        
        contact.serialize();
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