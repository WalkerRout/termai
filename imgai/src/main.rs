use std::collections::HashMap;
use reqwest::header::*; // AUTHORIZATION, CONTENT_TYPE
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIAnswer {
  url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponse {
  created: i64,
  data: Vec<OpenAIAnswer>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  let client = reqwest::Client::builder().build()?;
  
  let host = "https://api.openai.com/v1/images/generations";
  let prompt = std::env::args()
    .nth(1)
    .unwrap_or("White siamese cat".to_owned());
  let body = format!("{{\"prompt\": \"{}\"}}", prompt);
  let api_key = std::env::var("OPEN_AI_API_KEY")
    .expect("Unable to locate environment variable 'OPEN_AI_API_KEY'");

  let res = client
    .post(host)
    .header(AUTHORIZATION, format!("Bearer {}", api_key))
    .header(CONTENT_TYPE, "application/json")
    .body(body)
    .send().await?;

  let json_string = res.text().await?;

  let response: OpenAIResponse = serde_json::from_str(&json_string).unwrap();
  let url = response.data[0].url.trim();

  println!("\n{}\n", url);

  if let Some(path) = std::env::args().nth(2) {
    use std::io::Write;

    let mut file = std::fs::File::create(path)?;
    file.write_all(url.as_bytes())?;
  }

  Ok(())
}
