use futures_util::SinkExt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;
use futures_util::stream::StreamExt;
use log::{self, error};
use env_logger;

mod args;

use args::ScraperArgs;
use clap::Parser;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = ScraperArgs::parse();

    if &args.action_type == "start" {
        match connect_async(Url::parse("ws://127.0.0.1:8000/ws").unwrap()).await {
            Ok((ws_stream, _)) => {
                log::info!("Connected successfully with server!");

                let (_, mut read) = ws_stream.split();

                if let Some(Ok(Message::Text(text))) = read.next().await {
                    log::info!("Received a message from server: {}", text);
                    println!("Received a message from server: {}", text);
                }
            },
            Err(e) => {
                log::error!("Failed to connect to server: {}", e);
            }
        }
    } else {
        error!("Argument invalid");
    }
}


