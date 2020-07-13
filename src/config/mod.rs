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
pub struct SchedulerOpts {
    pub spy_check_interval: u32,
    pub daily_check_time: String,
    pub east_offfset: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub check_url: String,
    pub yes: Answer,
    pub no: Answer,
    pub scheduler: SchedulerOpts,
}

impl Config {
    pub fn from_path(path: &std::path::Path) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        toml::from_str(&contents).unwrap()
    }
}