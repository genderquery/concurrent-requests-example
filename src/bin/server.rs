use std::{error::Error, time::Duration};

use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use clap::Parser;
use reqwest::StatusCode;
use tokio::{net::TcpListener, time::sleep};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "localhost")]
    pub host: String,
    #[arg(long, default_value_t = 0)]
    pub port: u16,
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}

async fn root() -> &'static str {
    "/delay/:millis"
}

async fn delay(Path(millis): Path<u64>) {
    sleep(Duration::from_millis(millis)).await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let listener = TcpListener::bind((args.host, args.port)).await?;
    println!("Listening at http://{}", listener.local_addr()?);

    let app = Router::new()
        .route("/", get(root))
        .route("/delay/:millis", get(delay))
        .fallback(fallback);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
