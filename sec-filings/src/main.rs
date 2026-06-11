use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use dotenvy::dotenv;


#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    let massive = reqwest::Client::new();

    let api_key = get_api_key();

    let data = fetch_filings(&massive, &api_key).await?;

    let mut prompt = String::from("YOU ARE A STOCK TRADER ANALZYING THESE FORM 3 FILINGS, WHAT DO YOU NOTICE AND LOOKS GOOD TO BUY OR SHORT WITH A CONFIDENCE SCORE, respond in a list {company:ticker:confidence score}");

    prompt += &data;

    let client = Client::new();

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&json!({
            "model": "llama3.1:8b",
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await?
        .json::<OllamaResponse>()
        .await?;

    println!("AI says: {}", res.response);

    Ok(())
}

fn get_api_key() -> String {
    dotenv().ok();

    env::var("apikey")
        .expect("MASSIVE_API_KEY not set")
}

async fn fetch_filings(client: &Client, api_key: &str) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://api.massive.com/stocks/filings/vX/form-3?limit=1&sort=filing_date.desc&apiKey={}",
        api_key
    );

    let res = client
        .get(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}