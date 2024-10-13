use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Token {
    pub secret: String,
    pub expire: String,
}
