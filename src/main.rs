use reqwest::{Client, Error, Response};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct ConfigFile {
    config: Config,
    record: Record,
}

#[derive(Deserialize, Debug)]
struct Config {
    api_key: String,
    secret_key: String,
}

#[derive(Deserialize, Debug)]
struct Record {
    domain: String,
    rtype: RecordType,
}

#[derive(Deserialize, Debug)]
enum RecordType {
    A,
    AAAA,
    Both,
}

#[tokio::main]
async fn main() {
    let config =
        std::fs::read_to_string("dpb.toml").expect("Couldn't read dpb.toml configuration file.");
    let config: ConfigFile = toml::from_str(&config).expect("Failed to parse config.");

    let client = Client::new();
    let response = ping(&client, config.config.api_key, config.config.secret_key)
        .await
        .expect("Failed to ping.");
    dbg!(response);
}

async fn ping(client: &Client, api_key: String, secret_key: String) -> Result<Response, Error> {
    client
        .post("https://api.porkbun.com/api/json/v3/ping")
        .json(&json!({
            "secretapikey": secret_key,
            "apikey": api_key
        }))
        .send()
        .await
}
