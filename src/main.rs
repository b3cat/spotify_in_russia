extern crate clap;
extern crate spotify_in_russia;
extern crate reqwest;
extern crate log;
extern crate simple_logger;

use clap::{App, Arg};
use log::{info, error, Level};

use spotify_in_russia::{Config, SpotifyInRussia, SpotifyEnvParams};

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

    let env_params = SpotifyEnvParams::new();
    
    let spoty = SpotifyInRussia::new(&http_client, &cfg, &env_params);

    info!("I'll try to send smth");

    match spoty.check_and_send(&notify) {
        Some(_) => info!("Message has been sent"),
        _ => error!("Sending has been failed"),
    }
}
