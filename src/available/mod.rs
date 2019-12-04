extern crate reqwest;
extern crate regex;

use regex::Regex;

const SPOTIFY_COUNTRY_URL: &'static str = "https://www.spotify.com/ru-ru/select-your-country/";

#[derive(Debug)]
pub enum CheckError {
    ReqwestErr(reqwest::Error),
    Error(String),
}

impl From<reqwest::Error> for CheckError {
    fn from(error: reqwest::Error) -> Self {
        CheckError::ReqwestErr(error)
    }
}

impl From<String> for CheckError {
    fn from(error: String) -> Self {
        CheckError::Error(error)
    }
}

pub struct Checker<'a> {
    client: &'a reqwest::Client,
    check_reg_exp: Regex,
}

impl Checker<'_> {
    pub fn new<'a>(http_client: &'a reqwest::Client, reg_exp_str: &str) -> Checker<'a> {
        Checker {
            client: http_client,
            check_reg_exp: Regex::new(reg_exp_str).unwrap()
        }
    }

    pub fn check(&self) -> Result<bool, CheckError> {
        let res = &self.client.get(SPOTIFY_COUNTRY_URL).send()?.text()?;
        let m = self.check_reg_exp.find(&res);

        Ok(if let Some(_) = m { true } else { false })
    }
}