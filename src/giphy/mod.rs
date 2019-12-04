extern crate reqwest;
extern crate serde;

use serde::{Serialize, Deserialize};

const API_BASE: &'static str = "https://api.giphy.com/";

#[derive(Serialize, Deserialize)]
pub struct RandomImageRequest {
    tag: String,
    rating: String,
}

#[derive(Serialize, Deserialize)]
struct OptionUrls {
    url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Urls {
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    original: OptionUrls,
    original_still: Urls,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseData {
    images: Image,
}

#[derive(Serialize, Deserialize)]
pub struct RandomImageResponse {
    data: ResponseData,
}

pub struct Giphy<'a, 'b> {
    api_key: &'a str,
    client: &'b reqwest::Client,
}

impl Giphy<'_, '_> {
    pub fn new<'a, 'b>(http_client: &'b reqwest::Client, api_key: &'a str) -> Giphy<'a, 'b> {
        Giphy {
            api_key,
            client: http_client,
        }
    }

    pub fn get_rand_image_url(&self, query: &str) -> Result<String, reqwest::Error> {
        let method = "v1/gifs/random";
        let endpoint = format!("{}{}", API_BASE, method);
        let resp = self.client
            .get(&endpoint)
            .query(&[("api_key", self.api_key)])
            .query(&RandomImageRequest { tag: String::from(query), rating: String::from("g") })
            .send()?
            .json::<RandomImageResponse>()?;
        
        // хз, какая лучше, пусть будет обе)
        Ok(resp.data.images.original.url
            .unwrap_or(resp.data.images.original_still.url))
    }
}