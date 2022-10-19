
extern crate reqwest;
extern crate serde;

use std::collections::HashMap;
use std::error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MdzDocument {
    title: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MdzScreenNews {
    documents: HashMap<String, MdzDocument>
}

pub struct Mdz<'a, 'b> {
    client: &'a reqwest::Client,
    url: &'b str,
}

impl Mdz<'_, '_> {
    pub fn new<'a, 'b>(client: &'a reqwest::Client, meduza_url: &'b str) -> Mdz<'a, 'b> {
        Mdz {
            client,
            url: meduza_url,
        }
    }

    pub async fn get_the_last_news(&self) -> Result<String, Box<dyn error::Error>> {
        let method = "screens/news";
        let endpoint = format!("{}{}", self.url, method);

        let res = self.client
            .get(&endpoint)
            .send()
            .await?
            .json::<MdzScreenNews>().await?;

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