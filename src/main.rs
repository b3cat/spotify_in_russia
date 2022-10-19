extern crate spotify_in_russia;
extern crate reqwest;
extern crate log;
extern crate simple_logger;
extern crate clokwerk;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate structopt;

use simple_logger::{SimpleLogger};
use log::{info, error, LevelFilter};
use spotify_in_russia::{Config, SchedulerOpts, SpotifyInRussia, SpotifyEnvParams};
use clokwerk::{TimeUnits, AsyncScheduler, Job};
use std::time::Duration;
use reqwest::{Client as ReqwsetClient, redirect};
use std::sync::atomic::{AtomicBool, Ordering};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Spotify")]
struct CliOpts {
    #[structopt(short, long, default_value("./config.toml"))]
    config: std::path::PathBuf,
    #[structopt(short, long, default_value("INFO"), env = "LOG_LEVEL")]
    log_level: LevelFilter,
}

lazy_static! {
    static ref CLI_OPTS: CliOpts = CliOpts::from_args();
    static ref HTTP_CLIENT: ReqwsetClient = ReqwsetClient::builder()
        // Не фоловим редиректы
        .redirect(redirect::Policy::none())
        .build()
        .unwrap();

    static ref ENV_PARAMS: SpotifyEnvParams = SpotifyEnvParams::new();

    static ref CONFIG: Config = Config::from_path(&CLI_OPTS.config);

    static ref SPOTIFY_IN_RUSSIA: SpotifyInRussia<'static> = SpotifyInRussia::new(&HTTP_CLIENT, &CONFIG, &ENV_PARAMS);

    static ref FINISHED: AtomicBool = AtomicBool::new(false);
}

#[tokio::main]
async fn main() {
    SimpleLogger::new().with_level(CLI_OPTS.log_level).with_utc_timestamps().init().unwrap();

    let SchedulerOpts { spy_check_interval, daily_check_time, east_offfset } = &CONFIG.scheduler;
    let tz = chrono::FixedOffset::east(*east_offfset);
    let mut scheduler = AsyncScheduler::with_tz(tz);

    info!("Starting work");
    scheduler.every(spy_check_interval.seconds()).run(|| get_check_task("available"));
    scheduler.every(1.day()).at(daily_check_time).run(|| get_check_task("always"));

    loop {
        let is_finished = FINISHED.load(Ordering::Relaxed);
        if is_finished {
            break
        }

        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

async fn get_check_task(check_codition: &'static str) -> () {
    info!("Check send condition: {}", check_codition);
    match SPOTIFY_IN_RUSSIA.check_and_send(check_codition).await {
        Some(is_available) => {
            info!("Check completed successfully");
            if is_available {
                // если вышел, то больше чекать не хотим
                FINISHED.store(true, Ordering::Relaxed);
            }
        },
        _ => error!("Check failed"),
    };
}

