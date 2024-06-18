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
                //log::info!("Connected successfully with server!");

                let (mut write, mut read) = ws_stream.split();

                while let Some(msg) = read.next().await {
                    let start_message = Message::Text(args.action_type.clone());
                    match msg {
                        Ok(Message::Text(text)) => {
                            println!("Received from server: {}", text);
                            write.send(start_message).await.expect("Failed to send message");
                        }
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error receiving message form server: {:?}", e);
                            break;
                        }
                    }
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


