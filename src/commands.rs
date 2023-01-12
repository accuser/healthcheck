use const_format::formatcp;
use reqwest::{Client, Error, Response, Url};

const USER_AGENT: &'static str =
    formatcp!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

/// Perform a simple HEAD request to check the health of an endpoint.
pub async fn healthcheck(url: Url) -> Result<Response, Error> {
    Client::builder()
        .user_agent(USER_AGENT)
        .build()?
        .head(url.to_string())
        .send()
        .await
}

/// Report a healthcheck result.
pub fn healthcheck_report(result: Result<Response, Error>, verbose: bool) {
    match result {
        Ok(res) => println!("🟢 {}", res.url()),
        Err(err) => match (err.url(), verbose) {
            (Some(url), false) => println!("🔴 {}", url),
            _ => println!("🔴 {}", err.to_string()),
        },
    }
}
