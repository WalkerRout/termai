use std::collections::HashMap;
use reqwest::header::*; // AUTHORIZATION, CONTENT_TYPE
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIAnswer {
  text: String,
  index: i32,
  logprobs: Option<f32>,
  finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIUsage {
  prompt_tokens: i32,
  completion_tokens: i32,
  total_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponse {
  id: String,
  object: String,
  created: i64,
  model: String,
  choices: Vec<OpenAIAnswer>,
  usage: OpenAIUsage,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  let client = reqwest::Client::builder().build()?;
  
  let host = "https://api.openai.com/v1/completions";
  let model = "text-davinci-003";
  let temperature = std::env::var("OPEN_AI_TEMPERATURE").unwrap_or(String::from("1.00"));
  let prompt = std::env::args()
    .nth(1)
    .unwrap_or("Repeat after me: I forgot to type in a prompt".to_string());
  let body = format!("{{\"model\": \"{}\", \"prompt\": \"{}\", \"temperature\": {}, \"max_tokens\": 2048}}", 
    model, prompt, temperature);
  
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

  println!("\n{:?}\n", response.choices[0].text.trim());

  if let Some(path) = std::env::args().nth(2) {
    use std::io::Write;

    let mut file = std::fs::File::create(path)?;
    file.write_all(response.choices[0].text.as_bytes())?;
  }

  Ok(())
}
