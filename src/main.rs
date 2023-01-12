mod commands;

use clap::Parser;
use commands::*;
use reqwest::Url;
use tokio::task::JoinSet;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// URLs to check
    #[arg(required = true)]
    urls: Vec<Url>,

    /// Use verbose output
    #[arg(long, short)]
    verbose: bool,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let args = Args::parse();

    let mut futures = JoinSet::new();

    for url in args.urls {
        futures.spawn(healthcheck(url));
    }

    while let Some(result) = futures.join_next().await {
        if let Ok(response) = result {
            healthcheck_report(response, args.verbose)
        }
    }
}
