mod config;
mod http;

use crate::config::{get_config, Toml};
use crate::http::{edit_a, edit_aaaa, get_ipv4, get_ipv6};

use clap::Parser;
use log::{error, info};
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interval: Option<u64>,

    #[arg(short, long, default_value_t = String::from("dpb.toml"))]
    config: String,

    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(2)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();

    let config = match get_config(&args.config) {
        Ok(config) => config,
        Err(error) => {
            error!("failed to get configuration");
            error!("{error}");
            return;
        }
    };

    let client = reqwest::blocking::Client::new();

    match args.interval {
        Some(i) => loop {
            edit_a_records(&config, &client);
            edit_aaaa_records(&config, &client);
            info!("sleeping for {i} seconds...");
            sleep(Duration::from_secs(i));
        },
        None => {
            edit_a_records(&config, &client);
            edit_aaaa_records(&config, &client);
        }
    }
}

fn edit_a_records(config: &Toml, client: &reqwest::blocking::Client) {
    if !config.A.subdomains.is_empty() {
        let ip = match get_ipv4(client, &config.config) {
            Ok(res) => res.yourIp,
            Err(error) => {
                error!("failed to get your ipv4 address");
                error!("{error}");
                return;
            }
        };
        for subdomain in config.A.subdomains.clone() {
            match edit_a(
                client,
                &config.config,
                subdomain.to_string().trim_matches('"'),
                &ip,
            ) {
                Ok(_) => {
                    info!(
                        "updated {}.{}",
                        subdomain.to_string().trim_matches('"'),
                        config.config.domain
                    );
                }
                Err(error) => {
                    error!(
                        "failed to update {}.{}",
                        subdomain.to_string().trim_matches('"'),
                        config.config.domain
                    );
                    error!("{error}");
                    continue;
                }
            };
        }
    }
}

fn edit_aaaa_records(config: &Toml, client: &reqwest::blocking::Client) {
    if !config.AAAA.subdomains.is_empty() {
        let ip = match get_ipv6(client, &config.config) {
            Ok(res) => res.yourIp,
            Err(error) => {
                error!("failed to get your ipv4 address");
                error!("{error}");
                return;
            }
        };
        for subdomain in config.AAAA.subdomains.clone() {
            match edit_aaaa(
                client,
                &config.config,
                subdomain.to_string().trim_matches('"'),
                &ip,
            ) {
                Ok(_) => {
                    info!(
                        "updated {}.{}",
                        subdomain.to_string().trim_matches('"'),
                        config.config.domain
                    );
                }
                Err(error) => {
                    error!(
                        "failed to update {}.{}",
                        subdomain.to_string().trim_matches('"'),
                        config.config.domain
                    );
                    error!("{error}");
                    continue;
                }
            };
        }
    }
}
