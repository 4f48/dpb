mod config;
mod http;

use crate::config::{get_config, Toml};
use crate::http::{edit_a, edit_aaaa, get_ipv4, get_ipv6};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interval: Option<u32>,

    #[arg(short, long, default_value_t = String::from("dpb.toml"))]
    config: String,
}

fn main() {
    let args = Args::parse();
    let config = get_config(&args.config).expect("Failed to get configuration file.");

    let client = reqwest::blocking::Client::new();

    match args.interval {
        Some(_interval) => {
            todo!()
        }
        None => {
            edit_a_records(&config, &client);
            edit_aaaa_records(&config, &client);
        }
    }
}

fn edit_a_records(config: &Toml, client: &reqwest::blocking::Client) {
    if !config.A.subdomains.is_empty() {
        let ip = get_ipv4(client, &config.config)
            .expect("couldn't retreive ipv4 address")
            .yourIp;
        for subdomain in config.A.subdomains.clone() {
            match edit_a(client, &config.config, subdomain.to_string(), &ip) {
                Ok(_) => (),
                Err(error) => {
                    println!(
                        "failed to update {}.{}: {}",
                        config.config.domain, subdomain, error
                    );
                    continue;
                }
            };
        }
    }
}

fn edit_aaaa_records(config: &Toml, client: &reqwest::blocking::Client) {
    if !config.AAAA.subdomains.is_empty() {
        let ip = get_ipv6(client, &config.config)
            .expect("couldn't retreive ipv6 address")
            .yourIp;
        for subdomain in config.AAAA.subdomains.clone() {
            match edit_aaaa(client, &config.config, subdomain.to_string(), &ip) {
                Ok(_) => (),
                Err(error) => {
                    println!(
                        "failed to update {}.{}: {}",
                        config.config.domain, subdomain, error
                    );
                    continue;
                }
            };
        }
    }
}
