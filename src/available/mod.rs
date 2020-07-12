extern crate reqwest;
extern crate log;

use log::{info};
use reqwest::{StatusCode, Error as ReqwestError};

pub struct Checker<'a> {
    client: &'a reqwest::Client,
    check_url: &'a str,
}

impl Checker<'_> {
    pub fn new<'a>(http_client: &'a reqwest::Client, check_url: &'a str) -> Checker<'a> {
        Checker {
            client: http_client,
            check_url,
        }
    }

    pub fn check(&self) -> Result<bool, ReqwestError> {
        let res = &self.client.get(self.check_url).send()?;
        let status_code = res.status();

        match status_code {
            StatusCode::OK => Ok(true),
            _ => {
                info!("Unavailable cause bad code was found: {}", status_code.as_u16());
                Ok(false)
            },
        }
    }
}