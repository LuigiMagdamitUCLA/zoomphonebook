use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub name: String,
    pub description: String,
    pub link: String,
}