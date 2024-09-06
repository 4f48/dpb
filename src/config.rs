use serde::Deserialize;
use toml::value::Array;

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct Toml {
    pub config: Config,
    pub A: A,
    pub AAAA: AAAA,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub key: String,
    pub secret: String,
    pub domain: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct A {
    pub subdomains: Array,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, Deserialize)]
pub struct AAAA {
    pub subdomains: Array,
}

pub fn get_config(file_path: &str) -> Result<Toml, Box<dyn std::error::Error>> {
    Ok(toml::from_str::<Toml>(&std::fs::read_to_string(
        file_path,
    )?)?)
}
