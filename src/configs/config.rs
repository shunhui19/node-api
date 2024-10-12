use core::panic;
use std::net::IpAddr;

use serde::{Deserialize, Deserializer};
use tracing::error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub token: Token,
    // pub db: Db,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub local_ip: IpAddr,
    pub local_port: u16,
    pub timeout: u16,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub log_file_name: Option<String>,
}

fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.and_then(|s| if s.is_empty() { None } else { Some(s) }))
}

#[derive(Deserialize, Debug)]
pub struct Token {
    pub secret: String,
    pub expire: String,
}

// #[derive(Deserialize, Debug)]
// pub struct Db {
//     connect: String,
//     db_name: String,
//     username: String,
//     password: String,
// }

impl Config {
    pub fn init_config() -> Self {
        // TODO add self defined config file path.
        let config_str = include_str!("../etc/server_local.toml");
        match toml::from_str::<Config>(config_str) {
            Ok(config) => config,
            Err(e) => {
                error!("Cannot load the config: {}", e);
                panic!("Load config file failed.");
            }
        }
    }
}

/// Parse the str of time's type, like
/// "1s" is second type,
/// "1m" is min type,
/// "1d" is day type.
/// Currently support second, min, hour and day types.
// TODO 后续调整返回值类型为Duration,并且调用的这个函数的地方统一类型为
// std::time::Duration,而不是chrono::Duration
pub fn parse_str_to_num(s: &str) -> Result<u64, String> {
    let s = s.trim();
    let (value, unit) = if let Some(value) = s.strip_suffix("s") {
        (value, "s")
    } else if let Some(value) = s.strip_suffix("m") {
        (value, "m")
    } else if let Some(value) = s.strip_suffix("h") {
        (value, "h")
    } else if let Some(value) = s.strip_suffix("d") {
        (value, "d")
    } else {
        return Err(format!("Invalid number format: {}", s));
    };

    match value.parse::<u64>() {
        Ok(val) => match unit {
            "s" => Ok(val),
            "m" => Ok(val * 60),
            "h" => Ok(val * 60 * 60),
            "d" => Ok(val * 60 * 60 * 24),
            _ => Err(format!("Unsupported time unit: {}", unit)),
        },
        Err(_) => Err(format!("Invalid number format in duration: {}", value)),
    }
}

#[cfg(test)]
mod test {

    use super::{parse_str_to_num, Config};

    #[test]
    fn parse_str_to_num_test() {
        // Failed cases.
        assert_eq!(
            parse_str_to_num("ss "),
            Err("Invalid number format in duration: s".to_string())
        );
        assert_eq!(
            parse_str_to_num("s"),
            Err("Invalid number format in duration: ".to_string())
        );
        assert_eq!(
            parse_str_to_num("m"),
            Err("Invalid number format in duration: ".to_string())
        );

        assert_eq!(parse_str_to_num("1s "), Ok(1));
        assert_eq!(parse_str_to_num("1m"), Ok(60));
        assert_eq!(parse_str_to_num("1h"), Ok(60 * 60));
        assert_eq!(parse_str_to_num("1d"), Ok(60 * 60 * 24));
    }

    #[test]
    fn validate_config_test() {
        let config_str = r#"
        [server]
        local_ip = "127.0.0.1"
        local_port = 3000
        # support second and min types.
        timeout = 1
        log_file_name = ""

        [token]
        secret = "hell, rust"
        # support second, min, hour and day types.
        expire = "1d"


        [db]
        connect = ""
        db_name = ""
        username = ""
        password = ""
        "#;
        let config: Config = toml::from_str(config_str).unwrap();
        assert_eq!(
            parse_str_to_num(config.token.expire.as_str()).unwrap(),
            86400
        );
    }
}
