use std::{error::Error, time::Instant};

use clap::Parser;
use futures::{stream, StreamExt};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "localhost")]
    pub host: String,
    #[arg(long, default_value_t = 0)]
    pub port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let client = reqwest::Client::new();

    let urls = vec![format!("http://{}:{}/delay/1000", args.host, args.port); 10];

    let responses = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move { (Instant::now(), client.get(url).send().await) }
        })
        .buffer_unordered(2);

    let total_time = Instant::now();

    responses
        .for_each(|(time, resp)| async move {
            match resp {
                Ok(resp) => {
                    println!(
                        "{} ({} ms)",
                        resp.url().as_str(),
                        time.elapsed().as_millis()
                    );
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        })
        .await;

    println!("Took {} ms", total_time.elapsed().as_millis());

    Ok(())
}
