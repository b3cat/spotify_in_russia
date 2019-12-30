
extern crate reqwest;
extern crate serde;

use std::collections::HashMap;
use std::error;
use serde::{Serialize, Deserialize};

const API_BASE: &str = "https://meduza.io/api/w5/";

#[derive(Serialize, Deserialize, Debug)]
pub struct MdzDocument {
    title: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MdzScreenNews {
    documents: HashMap<String, MdzDocument>
}

pub struct Mdz<'a> {
    client: &'a reqwest::Client,
}

impl Mdz<'_> {
    pub fn new<'a>(client: &'a reqwest::Client) -> Mdz<'a> {
        Mdz {
            client,
        }
    }

    pub fn get_the_last_news(&self) -> Result<String, Box<dyn error::Error>> {
        let method = "screens/news";
        let endpoint = format!("{}{}", API_BASE, method);

        let res = self.client
            .get(&endpoint)
            .send()?
            .json::<MdzScreenNews>()?;

        let mut result_title = String::from("");

        for document_url in res.documents.keys() {
            if document_url.contains("news/") {
                match res.documents.get(document_url) {
                    Some(doc) => {
                        match &doc.title {
                            Some(title) => {
                                result_title = title.to_string();
                                break;
                            },
                            None => ()
                        }
                    },
                    None => ()
                }
            }
        }
        
        Ok(result_title)
    }
}