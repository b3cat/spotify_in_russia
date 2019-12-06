extern crate clap;
extern crate spotify_in_russia;
extern crate reqwest;
extern crate log;
extern crate simple_logger;

use clap::{App, Arg};
use log::{info, warn, Level};

use spotify_in_russia::{Checker, Config, Giphy, Tgm};

// TODO: вынести в конфиг
const DEFAULT_GIF_URL: &str = "https://media3.giphy.com/media/RddAJiGxTPQFa/giphy.gif";

fn check_and_send(checker: &Checker, giphy: &Giphy, tgm: &Tgm, cfg: &Config, send_cond: &str) -> Option<()> {
    let is_available = match checker.check() {
        Ok(res) => res,
        Err(err) => {
            warn!("Cannot get spotify status: {:?}, set false", err);
            false
        }
    };

    let answer = if is_available { &cfg.yes } else { &cfg.no };
    let message = answer.get_pretty_answer();
    let giphy_query = &answer.giphy_query;

    info!("Answer is {}", &message);
    info!("Giphy query is {}", giphy_query);

    if send_cond == "available" && !is_available {
        info!("Available mode. But not available, i'll not send message");
        return Some(())
    }

    let url = match giphy.get_rand_image_url(giphy_query){
        Ok(res) => res,
        Err(err) => {
            warn!("Get error when trying to search gif: {:?}, set false", err);
            String::from(DEFAULT_GIF_URL)
        }
    };

    info!("Image url is {}", url);

    match tgm.send_document(&url, &message) {
        Err(err) => {
            warn!("Cannot send message: {:?}", err);
            None
        }
        _ => Some(())
    }
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let matches = App::new("Spotify In Russia")
        .version("1.0")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .help("Config .toml file path")
            .takes_value(true)
        )
        .arg(Arg::with_name("notify")
            .short("n")
            .long("notify")
            .help("notify [always | available]")
            .takes_value(true)
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("./config.toml");
    let notify = matches.value_of("notify").unwrap_or("always");

    let cfg = Config::new(config_path);

    let http_client = reqwest::Client::new();

    info!("I'll try to send smth");
    
    let checker = Checker::new(&http_client, &cfg.check_reg_exp);
    let tgm = Tgm::new(&http_client, &cfg.keys.telegram, &cfg.chat_id);
    let giphy = Giphy::new(&http_client, &cfg.keys.giphy);

    check_and_send(&checker, &giphy, &tgm, &cfg, &notify).unwrap_or(())
}
