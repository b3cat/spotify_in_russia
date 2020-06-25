extern crate log;

mod available;
mod config;
mod giphy;
mod tgm;
mod mdz;

use std::env;
use log::{info, warn};
pub use config::{Config, SchedulerOpts};

const DEFAULT_GIF_URL: &str = "https://media3.giphy.com/media/RddAJiGxTPQFa/giphy.gif";

pub struct SpotifyEnvParams {
    telegram_key: String,
    chat_id: String,
    giphy_key: String,
}

impl SpotifyEnvParams {
    pub fn new() -> SpotifyEnvParams{
        SpotifyEnvParams {
            telegram_key: env::var("TGM_TOKEN").unwrap_or(String::from("")),
            chat_id: env::var("TGM_CHAT_ID").unwrap_or(String::from("")),
            giphy_key: env::var("GIPHY_TOKEN").unwrap_or(String::from("")),
        }
    }
}

pub struct SpotifyInRussia<'a> {
    checker: available::Checker<'a>,
    config: &'a config::Config,
    giphy: giphy::Giphy<'a, 'a>,
    mdz: mdz::Mdz<'a>,
    tgm: tgm::Tgm<'a, 'a, 'a>,
}

impl SpotifyInRussia<'_> {
    pub fn new<'a>(
        http_client: &'a reqwest::Client,
        config: &'a config::Config,
        env_params: &'a SpotifyEnvParams,
    ) -> SpotifyInRussia<'a> {
        SpotifyInRussia {
            checker: available::Checker::new(http_client, &config.check_reg_exp),
            giphy: giphy::Giphy::new(http_client, &env_params.giphy_key),
            tgm: tgm::Tgm::new(http_client, &env_params.telegram_key, &env_params.chat_id),
            mdz: mdz::Mdz::new(http_client),
            config: config,
        }
    }

    pub fn check_and_send(&self, send_cond: &str) -> Option<()> {
        let is_available = match self.checker.check() {
            Ok(res) => res,
            Err(err) => {
                warn!("Cannot get spotify status: {:?}, set false", err);
                false
            }
        };
    
        let answer = if is_available { &self.config.yes } else { &self.config.no };
        let last_news = self.mdz.get_the_last_news().unwrap_or(String::from("В мире ничего не произошло"));

        let message = format!("{}.\n\n{}", last_news, answer.get_pretty_answer());
        let giphy_query = &answer.giphy_query;
        
        info!("Answer is {}", message);

        info!("Giphy query is {}", giphy_query);
    
        if send_cond == "available" && !is_available {
            info!("Available mode. But not available, i'll not send message");
            return Some(())
        }
    
        let url = match self.giphy.get_rand_image_url(giphy_query){
            Ok(res) => res,
            Err(err) => {
                warn!("Get error when trying to search gif: {:?}, set false", err);
                String::from(DEFAULT_GIF_URL)
            }
        };
    
        info!("Image url is {}", url);
    
        match self.tgm.send_document(&url, &message) {
            Err(err) => {
                warn!("Cannot send message: {:?}", err);
                None
            }
            Ok(response) => { 
                match response.status() {
                    code if !code.is_success() => {
                        warn!("Not success status code! Reason: {}", code.canonical_reason().unwrap_or("no writable reason"));
                        None
                    },
                    _ => Some(())
                }
                
            }
        }
    }
}

