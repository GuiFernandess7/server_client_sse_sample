use log::{self, error};
use reqwest::blocking::{Client, ClientBuilder};
use env_logger;

mod args;

use args::ScraperArgs;
use clap::Parser;

fn main() {
    env_logger::init();
    let args = ScraperArgs::parse();

    if &args.action_type == "start" {
        let http_client = Client::new();
        let results = http_client.get("http://localhost:8000/stream/main")
                                .send();
        if results.is_ok() {
            println!("{:#?}", results.ok().unwrap().text().unwrap());
        }
     }
}


