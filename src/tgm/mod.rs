extern crate reqwest;
extern crate serde;

use serde::{Serialize};

const API_BASE: &str = "https://api.telegram.org/";

pub struct Tgm<'a, 'b, 'c> {
    token: &'a str,
    client: &'b reqwest::Client,
    chat_id: &'c str,
}

#[derive(Serialize, Debug)]
struct SendDocumentParams<'a, 'b> {
    document: &'a str,
    caption: &'b str,
} 

impl Tgm<'_, '_, '_> {
    pub fn new<'a, 'b, 'c>(http_client: &'b reqwest::Client, api_key: &'a str, chat_id: &'c str) -> Tgm<'a, 'b, 'c> {
        Tgm {
            token: api_key,
            client: http_client,
            chat_id,
        }
    }

    pub fn send_document(&self, document: &str, caption: &str) -> reqwest::Result<reqwest::Response> {
        let method = "sendDocument";
        let endpoint = format!("{}{}/{}", API_BASE, self.token, method);
        
        self.client.get(&endpoint)
            .query(&[("chat_id", self.chat_id)])
            .query(&SendDocumentParams { document, caption })
            .send()
    }   
}
