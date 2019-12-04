extern crate serde;
extern crate toml;
extern crate rand; 

use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;

pub type StringSegments = Vec<Vec<String>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Answer {
    pub giphy_query: String,
    pub parts: StringSegments,
}

impl Answer {
    pub fn get_pretty_answer(&self) -> String {
        let mut result: Vec<&str> = vec!();

        for part in &self.parts {
            result.push(
                match part.choose(&mut rand::thread_rng()) {
                    Some(el) => el,
                    None => "",
                }
            );
        }

        result.join(" ")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKeys {
    pub giphy: String,
    pub telegram: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub check_reg_exp: String,
    pub chat_id: String,
    pub keys: ApiKeys,
    pub yes: Answer,
    pub no: Answer,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        toml::from_str(&contents).unwrap()
    }
}