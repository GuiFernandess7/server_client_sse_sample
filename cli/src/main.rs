use futures::{Stream, TryStreamExt};
use std::{env, process, time::Duration};
use tokio::main;
use env_logger;

use eventsource_client as es;

mod args;

use args::ScraperArgs;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), es::Error>{
    env_logger::init();
    let args = ScraperArgs::parse();
    let auth_header = "";

    if &args.action_type == "start" {
        let client = es::ClientBuilder::for_url("http://localhost:8000/events")?
                                        .header("Authorization", auth_header)?
                                        .reconnect(
                                            es::ReconnectOptions::reconnect(true)
                                                .retry_initial(false)
                                                .delay(Duration::from_secs(1))
                                                .backoff_factor(2)
                                                .delay_max(Duration::from_secs(60))
                                                .build(),
                                        )
                                        .build();
        let mut stream = tail_events(client);

        while let Ok(Some(_)) = stream.try_next().await {}

        ()
    }
    Ok(())
    //Err(es::Error::StreamClosed)
}

fn tail_events(client: impl es::Client) -> impl Stream<Item = Result<(), ()>> {
    client
        .stream()
        .map_ok(|event| match event {
            es::SSE::Event(ev) => {
                println!("got an event: {}\n{}", ev.event_type, ev.data)
            }
            es::SSE::Comment(comment) => {
                println!("got a comment: \n{}", comment)
            }
        })
        .map_err(|err| eprintln!("error streaming events: {:?}", err))
}
