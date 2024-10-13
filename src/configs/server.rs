use std::net::IpAddr;

use serde::{Deserialize, Deserializer};
use tracing::Level;

#[derive(Deserialize, Debug)]
pub struct Server {
    pub local_ip: IpAddr,
    pub local_port: u16,
    pub timeout: u16,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub log_file_name: Option<String>,
    #[serde(deserialize_with = "deserialize_level_u8")]
    pub log_level: Level,
}

fn deserialize_level_u8<'de, D>(deserializer: D) -> Result<Level, D::Error>
where
    D: Deserializer<'de>,
{
    let level_num: u8 = u8::deserialize(deserializer)?;
    match level_num {
        0 => Ok(Level::TRACE),
        1 => Ok(Level::DEBUG),
        2 => Ok(Level::INFO),
        3 => Ok(Level::WARN),
        4 => Ok(Level::ERROR),

        _ => Ok(Level::INFO),
        // _ => Err(de::Error::custom(format!("Invalid level: {}", level_num))),
    }
}

fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.and_then(|s| if s.is_empty() { None } else { Some(s) }))
}
