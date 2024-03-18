use reqwest::{header::USER_AGENT, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "Narven",
        repo = "sublime-artisan"
    );

    println!("{}", request_url);

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "rust web-api-client demo")
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);

    Ok(())
}
