use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Serialize, Deserialize};

const EXPIRY: u64 = 55;

#[derive(Debug, Deserialize, Serialize)]
pub struct FireblocksClaims<'a> {
    uri: &'a str,
    nonce: u64,
    iat: u64,
    exp: u64,
    sub: &'a str,
    #[serde(rename = "bodyHash")]
    body_hash: String,
}


impl<'a> FireblocksClaims<'a> {
    pub fn new(uri: &'a str, sub: &'a str, body_hash: &'a str) -> Self {
        // use millisecond precision to ensure that it's not reused
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let nonce = now;
        let now = now / 1000;

        Self {
            uri,
            sub,
            body_hash: body_hash.to_string(),
            nonce,
            iat: now,
            exp: now + EXPIRY,
        }
    }
}
