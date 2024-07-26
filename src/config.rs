use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigFile {
    pub config: Config,
    pub record: Record,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub secret_key: String,
}

#[derive(Deserialize, Debug)]
pub struct Record {
    pub domain: String,
    pub subdomain: String,
    pub r#type: RecordType,
}

#[derive(Deserialize, Debug)]
pub enum RecordType {
    A,
    Aaaa,
    Both,
}

pub async fn config() -> Result<ConfigFile, Box<dyn std::error::Error>> {
    let config = std::fs::read_to_string("dpb.toml")?;
    Ok(toml::from_str::<ConfigFile>(&config)?)
}
