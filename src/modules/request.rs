use super::dotenvy::get_api_token;
use serde_json::{json, Value};

pub async fn get_price() -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let token = get_api_token();

    let req = client
        .get("https://api.api-ninjas.com/v1/cryptoprice?symbol=BTCUSDT")
        .header("X-Api-Key", token)
        .send()
        .await?
        .json::<Value>() 
        .await?; 
    return Ok(req);
}
