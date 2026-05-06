use reqwest::{Client, StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
  login: String,
  name: Option<String>,
  public_repos: u32,
  followers: u32,
  following: u32,
}

#[derive(Debug)]
enum FetchError {
  Request(reqwest::Error),
  NotFound,
  UnexpectedStatus(StatusCode),
}

impl From<reqwest::Error> for FetchError {
  fn from(err: reqwest::Error) -> Self {
    FetchError::Request(err)
  }
}

async fn fetch_user(client: &Client, username: &str) -> Result<User, FetchError> {
  let url = format!("https://api.github.com/users/{}", username);
  let response = client
    .get(&url)
    .header("User-Agent", "rust-client")
    .send()
    .await?;

  match response.status() {
    StatusCode::OK => Ok(response.json::<User>().await?),
    StatusCode::NOT_FOUND => Err(FetchError::NotFound),
    status => Err(FetchError::UnexpectedStatus(status)),
  }
}

#[tokio::main]
async fn main() {
  let client = Client::new();
  let username = "DevPedroHB";

  match fetch_user(&client, username).await {
    Ok(user) => {
      println!("Usuário: {user:?}");
    }
    Err(err) => {
      eprintln!("Erro: {err:?}");
    }
  }
}
