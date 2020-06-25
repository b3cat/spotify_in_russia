extern crate clap;
extern crate spotify_in_russia;
extern crate reqwest;
extern crate log;
extern crate simple_logger;
extern crate clokwerk;
extern crate chrono;

use clap::{App, Arg};
use log::{info, error, Level};

use spotify_in_russia::{Config, SchedulerOpts, SpotifyInRussia, SpotifyEnvParams};
use clokwerk::{Scheduler, TimeUnits};
use std::sync::Arc;
use std::time::Duration;
use std::thread;

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
        .get_matches();

    let matches = Box::leak(Box::new(matches));
    let config_path = matches.value_of("config").unwrap_or("./config.toml");
    let config = Config::from_path(config_path);
    let config = Arc::from(config);

    let SchedulerOpts { spy_check_interval, daily_check_time, east_offfset } = &config.scheduler;
    let tz = chrono::FixedOffset::east(*east_offfset);
    let mut scheduler = Scheduler::with_tz(tz);

    let cfg = config.clone(); 
    scheduler.every(spy_check_interval.seconds()).run(move || {
        let http_client = reqwest::Client::new();
        let env_params = SpotifyEnvParams::new();
        let spotify = SpotifyInRussia::new(&http_client, &cfg, &env_params);

        info!("This is spy check, notificiation will be sent if spotify is available");
        match spotify.check_and_send("available") {
            Some(_) => info!("Check completed successfully"),
            _ => error!("Check failed"),
        };
    });

    let cfg = config.clone(); 
    scheduler.every(1.day()).at(daily_check_time).run(move || {
        let http_client = reqwest::Client::new();
        let env_params = SpotifyEnvParams::new();
        let spotify = SpotifyInRussia::new(&http_client, &cfg, &env_params);

        info!("Day check is starting now!");
        match spotify.check_and_send("always") {
            Some(_) => info!("Check completed successfully"),
            _ => error!("Check failed"),
        };
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(500));
    }
}

