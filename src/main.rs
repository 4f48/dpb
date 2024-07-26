mod config;

use config::RecordType;
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::json;
use std::net::{Ipv4Addr, Ipv6Addr};

#[tokio::main]
async fn main() {
    let config = config::config().await.unwrap();
    let client = Client::new();

    match config.record.r#type {
        RecordType::A => {
            let ip = get_v4(&client, &config.config.api_key, &config.config.secret_key)
                .await
                .unwrap();
            let response = edit_v4_record(
                &client,
                &config.config.api_key,
                &config.config.secret_key,
                &ip.yourIp,
                &config.record.domain,
                &config.record.subdomain,
            )
            .await
            .unwrap();
            println!("{}", response.status);
        }
        RecordType::Aaaa => {
            let ip = get_v6(&client, &config.config.api_key, &config.config.secret_key)
                .await
                .unwrap();
            let response = edit_v6_record(
                &client,
                &config.config.api_key,
                &config.config.secret_key,
                &ip.yourIp,
                &config.record.domain,
                &config.record.subdomain,
            )
            .await
            .unwrap();
            println!("{}", response.status);
        }
        RecordType::Both => {
            let ip = get_v4(&client, &config.config.api_key, &config.config.secret_key)
                .await
                .unwrap();
            let response = edit_v4_record(
                &client,
                &config.config.api_key,
                &config.config.secret_key,
                &ip.yourIp,
                &config.record.domain,
                &config.record.subdomain,
            )
            .await
            .unwrap();
            println!("V4: {}", response.status);

            let ip = get_v6(&client, &config.config.api_key, &config.config.secret_key)
                .await
                .unwrap();
            let response = edit_v6_record(
                &client,
                &config.config.api_key,
                &config.config.secret_key,
                &ip.yourIp,
                &config.record.domain,
                &config.record.subdomain,
            )
            .await
            .unwrap();
            println!("V6: {}", response.status);
        }
    };
}
#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
struct ResV6 {
    status: String,
    yourIp: Ipv6Addr,
}
async fn get_v6(client: &Client, api_key: &str, secret_key: &str) -> Result<ResV6, Error> {
    let res: ResV6 = client
        .post("https://api.porkbun.com/api/json/v3/ping")
        .json(&json!({
            "secretapikey": secret_key,
            "apikey": api_key
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
struct ResV4 {
    status: String,
    yourIp: Ipv4Addr,
}
async fn get_v4(client: &Client, api_key: &str, secret_key: &str) -> Result<ResV4, Error> {
    let res: ResV4 = client
        .post("https://api-ipv4.porkbun.com/api/json/v3/ping")
        .json(&json!({
            "secretapikey": secret_key,
            "apikey": api_key
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}

#[derive(Deserialize)]
struct EditResp {
    status: String,
}

async fn edit_v4_record(
    client: &Client,
    api_key: &str,
    secret_key: &str,
    ip: &Ipv4Addr,
    domain: &str,
    subdomain: &str,
) -> Result<EditResp, Error> {
    let res: EditResp = client
        .post(format!(
            "https://api.porkbun.com/api/json/v3/dns/editByNameType/{domain}/A/{subdomain}"
        ))
        .json(&json!({
            "secretapikey": secret_key,
            "apikey": api_key,
            "content": ip
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}

async fn edit_v6_record(
    client: &Client,
    api_key: &str,
    secret_key: &str,
    ip: &Ipv6Addr,
    domain: &str,
    subdomain: &str,
) -> Result<EditResp, Error> {
    let res: EditResp = client
        .post(format!(
            "https://api.porkbun.com/api/json/v3/dns/editByNameType/{domain}/AAAA/{subdomain}"
        ))
        .json(&json!({
            "secretapikey": secret_key,
            "apikey": api_key,
            "content": ip
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}
