mod types;
use alloy_core::primitives::Address;
use types::{TransactionArguments, TransactionDetails, TransactionStatus};

mod api;
use api::FireblocksClient;

use jsonwebtoken::EncodingKey;
use std::{collections::HashMap, time::Instant};
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, FireblocksError>;

#[derive(Debug, Error)]
/// Fireblocks API related errors
pub enum FireblocksError {
    #[error(transparent)]
    /// Thrown when submitting a POST/GET request fails
    ReqwestError(#[from] reqwest::Error),

    #[error("Deserialization Error: {err}. Response: {text}")]
    /// Serde JSON Error
    SerdeJson {
        err: serde_json::Error,
        text: String,
    },

    #[error(
        "Transaction was not completed successfully. Final Status: {:?}. Sub status: {1}",
        0
    )]
    /// Thrown when a transaction submission or message signing fails
    TxError(TransactionStatus, String),

    #[error("Could not parse data: {0}")]
    /// Thrown when parsing string as Ethereum data fails
    ParseError(String),

    #[error("Timed out while waiting for user to approve transaction")]
    /// Thrown when the transaction isn't approved on time
    Timeout,
}

#[derive(Debug, Clone)]
pub struct FireblocksSigner {
    fireblocks: FireblocksClient,
    account_ids: HashMap<Address, String>,
    chain_id: u64,
    asset_id: String,
    address: Address,
    account_id: String,
    timeout: u128,
}

/// Configuration options for instantiating a [`FireblocksSigner`](FireblocksSigner)
pub struct Config {
    /// The RSA key file.
    pub key: EncodingKey,
    /// The API key which was provided to you by fireblocks support
    pub api_key: String,
    /// The chain id of the network you are connecting to
    pub chain_id: u64,
    /// Your vault's account id.
    pub account_id: String,
}

impl Config {
    /// Instantiates the config file given a path to the RSA file as well as the rest of the config
    /// args.
    pub fn new<T: AsRef<str>>(
        key: T,
        api_key: &str,
        account_id: &str,
        chain_id: u64,
    ) -> Result<Self> {
        let rsa_pem = std::fs::read(key.as_ref())?;
        let key = EncodingKey::from_rsa_pem(&rsa_pem)?;

        Ok(Self {
            key,
            chain_id,
            api_key: api_key.to_string(),
            account_id: account_id.to_string(),
        })
    }
}

impl AsRef<FireblocksClient> for FireblocksSigner {
    fn as_ref(&self) -> &FireblocksClient {
        &self.fireblocks
    }
}

impl FireblocksSigner {
    /// Instantiates a FireblocksSigner with the provided config
    pub async fn new(cfg: Config) -> Self {
        let fireblocks = FireblocksClient::new(cfg.key, &cfg.api_key);
        let asset_id = match cfg.chain_id {
            1 => "ETH",
            3 => "ETH_TEST3",
            5 => "ETH_TEST5",
            42 => "ETH_TEST2",
            _ => panic!("Unsupported chain_id"),
        };

        let res = fireblocks
            .vault_addresses(&cfg.account_id, asset_id)
            .await
            .expect("could not get vault addrs");

        Self {
            fireblocks,
            account_ids: HashMap::new(),
            chain_id: cfg.chain_id,
            asset_id: asset_id.to_owned(),
            address: res[0].address[2..]
                .parse()
                .expect("could not parse as address"),
            account_id: cfg.account_id,
            timeout: 60_000,
        }
    }

    /// Sets the timeout duration in milliseconds. If the user does not approve a
    /// transaction within this time, the transaction request throws an error.
    pub fn timeout(&mut self, timeout_ms: u128) {
        self.timeout = timeout_ms;
    }

    /// Registers an Account ID to Address mapping.
    pub fn add_account(&mut self, account_id: String, address: Address) {
        self.account_ids.insert(address, account_id);
    }

    async fn handle_action<F, R>(&self, args: TransactionArguments, func: F) -> Result<R>
    where
        F: FnOnce(TransactionDetails) -> Result<R>,
    {
        let res = self.fireblocks.create_transaction(args).await?;
        let start = Instant::now();
        loop {
            if Instant::now().duration_since(start).as_millis() >= self.timeout {
                return Err(FireblocksError::Timeout);
            }

            let details = self.fireblocks.transaction(&res.id).await?;
            use TransactionStatus::*;
            // Loops in pending signature
            match details.status {
                BROADCASTING | COMPLETED => return func(details),
                BLOCKED | CANCELLED | FAILED => {
                    return Err(FireblocksError::TxError(details.status, details.sub_status))
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
async fn test_signer() -> FireblocksSigner {
    let config = Config::new(
        std::env::var("FIREBLOCKS_API_SECRET_PATH").unwrap(),
        &std::env::var("FIREBLOCKS_API_KEY").unwrap(),
        &std::env::var("FIREBLOCKS_SOURCE_VAULT_ACCOUNT").unwrap(),
        5,
    )
    .unwrap();
    FireblocksSigner::new(config).await
}
