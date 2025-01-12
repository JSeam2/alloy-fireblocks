use crate::types::*;
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

const EXPIRY: u64 = 55;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Claims {
    uri: String,
    nonce: String,
    iat: i64,
    exp: i64,
    sub: String,
    #[serde(rename = "bodyHash")]
    body_hash: String,
}

#[derive(Debug, Clone)]
pub struct JwtSigner {
    // TODO: Make this work with Zeroize/Secrecy
    pub key: EncodingKey,
    pub api_key: String,
}

impl JwtSigner {
    pub fn new(key: EncodingKey, api_key: &str) -> Self {
        Self {
            key,
            api_key: api_key.to_string(),
        }
    }

    pub fn sign<S: Serialize>(&self, path: &str, body: S) -> Result<String, JwtError> {
        let header = Header::new(Algorithm::RS256);
        let claims = Claims::new(path, &self.api_key, body)?;
        Ok(jsonwebtoken::encode(&header, &claims, &self.key)?)
    }
}

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Could not serialize JWT body: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Could not create JWT time: {0}")]
    Time(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    Jwt(#[from] jwterrors::Error),
}

impl<'a> Claims<'a> {
    fn new<S: Serialize>(uri: &'a str, sub: &'a str, body: S) -> Result<Self, JwtError> {
        // use millisecond precision to ensure that it's not reused
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
        let mut rng = rand::thread_rng();
        let nonce = rng.gen::<u64>();
        let now = now / 1000;

        let body_hash = {
            let mut digest = Sha256::new();
            digest.update(serde_json::to_vec(&body)?);
            digest.finalize().to_vec()
        };

        Ok(Self {
            uri,
            sub,
            body_hash: body_hash.to_hex::<String>(),
            nonce,
            iat: now,
            exp: now + EXPIRY,
        })
    }
}

// TODO; tests
