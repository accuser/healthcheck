use clap::Parser;
use reqwest::{Client, Url};
use tokio::task::JoinSet;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// URLs to check
    #[arg(required = true)]
    urls: Vec<Url>,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let args = Args::parse();
    let mut set = JoinSet::new();

    for url in args.urls {
        if let Ok(client) = Client::builder()
            .user_agent("healthcheck".to_string())
            .build()
        {
            set.spawn(async move {
                match client.head(url.to_string()).send().await {
                    Ok(_) => println!("🟢 {}", url),
                    Err(err) => println!("🔴 {} ({})", url, err.without_url()),
                }
            });
        };
    }

    while let Some(_) = set.join_next().await {}
}
