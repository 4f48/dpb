use crate::config::Config;

use reqwest::header::USER_AGENT;
use serde::Deserialize;
use serde_json::json;
use std::net::{Ipv4Addr, Ipv6Addr};

const DPB_USERAGENT: &str = "4f48/dpb@1.0.0-rc.2 (0x4f48@proton.me)";

#[allow(non_snake_case, unused)]
#[derive(Deserialize, Debug)]
pub struct GetIpV4 {
    pub status: String,
    pub yourIp: Ipv4Addr,
}

#[allow(non_snake_case, unused)]
#[derive(Deserialize)]
pub struct GetIpV6 {
    pub status: String,
    pub yourIp: Ipv6Addr,
}

pub fn get_ipv4(
    client: &reqwest::blocking::Client,
    config: &Config,
) -> Result<GetIpV4, reqwest::Error> {
    let res: GetIpV4 = client
        .post("https://api-ipv4.porkbun.com/api/json/v3/ping")
        .header(USER_AGENT, DPB_USERAGENT)
        .json(&json!({
            "secretapikey": config.secret,
            "apikey": config.key
        }))
        .send()?
        .json()?;
    Ok(res)
}

pub fn get_ipv6(
    client: &reqwest::blocking::Client,
    config: &Config,
) -> Result<GetIpV6, reqwest::Error> {
    let res: GetIpV6 = client
        .post("https://api.porkbun.com/api/json/v3/ping")
        .header(USER_AGENT, DPB_USERAGENT)
        .json(&json!({
            "secretapikey": config.secret,
            "apikey": config.key
        }))
        .send()?
        .json()?;
    Ok(res)
}

pub fn edit_a(
    client: &reqwest::blocking::Client,
    config: &Config,
    subdomain: &str,
    ip: &Ipv4Addr,
) -> Result<(), reqwest::Error> {
    client
        .post(format!(
            "https://api.porkbun.com/api/json/v3/dns/editByNameType/{}/A/{}",
            config.domain, subdomain
        ))
        .header(USER_AGENT, DPB_USERAGENT)
        .json(&json!({
            "secretapikey": config.secret,
            "apikey": config.key,
            "content": ip
        }))
        .send()?;
    Ok(())
}

pub fn edit_aaaa(
    client: &reqwest::blocking::Client,
    config: &Config,
    subdomain: &str,
    ip: &Ipv6Addr,
) -> Result<(), reqwest::Error> {
    client
        .post(format!(
            "https://api.porkbun.com/api/json/v3/dns/editByNameType/{}/AAAA/{}",
            config.domain, subdomain
        ))
        .header(USER_AGENT, DPB_USERAGENT)
        .json(&json!({
            "secretapikey": config.secret,
            "apikey": config.key,
            "content": ip
        }))
        .send()?;
    Ok(())
}
