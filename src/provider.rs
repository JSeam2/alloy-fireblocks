use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use alloy_core::primitives::Address;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_transport::{TransportError, TransportErrorKind};

use crate::{
    api::FireblocksClient,
    types::{Asset, FireblocksError, FireblocksProviderConfig, TransactionStatus},
};

/// A Web3 provider that integrates with Fireblocks custody
pub struct FireblocksProvider {
    /// The underlying RPC client for basic operations
    pub inner: Arc<dyn Provider>,
    /// Fireblocks SDK client
    pub fireblocks: FireblocksClient,
    /// Configuration
    pub config: FireblocksProviderConfig,
    /// Cached account addresses
    pub accounts: Arc<RwLock<HashMap<u64, Address>>>,
}

impl FireblocksProvider {
    /// Create a new Fireblocks provider
    /// Create a new Fireblocks provider
    pub async fn new(config: FireblocksProviderConfig) -> Result<Self, TransportError> {
        // Clone only necessary fields for client initialization
        let client_private_key = config.private_key.clone();
        let client_api_key = config.api_key.clone();
        let client_api_base_url = config.api_base_url.clone();

        // Initialize RPC provider using direct config access
        let inner = if let Some(rpc) = &config.rpc_url {
            Arc::new(ProviderBuilder::new().on_builtin(rpc.as_str()).await?)
        } else {
            let asset = Asset::get_by_chain_id(&config.chain_id);
            Arc::new(
                ProviderBuilder::new()
                    .on_builtin(asset.rpc_url.as_str())
                    .await?,
            )
        };

        // Initialize Fireblocks SDK with cloned values
        let fireblocks =
            FireblocksClient::new(client_private_key, client_api_key, client_api_base_url);

        // Create the provider with empty accounts
        let provider = Self {
            inner,
            fireblocks,
            config, // Original intact config
            accounts: Arc::new(RwLock::new(HashMap::new())),
        };

        provider
            .populate_accounts()
            .await
            .map_err(|_e| TransportErrorKind::custom_str("Failed to populate accounts"))?;

        Ok(provider)
    }

    /// Get the user agent for program
    pub fn get_user_agent(&self) -> String {
        if let Some(custom_ua) = &self.config.user_agent {
            format!(
                "alloy-fireblocks/{} {}",
                env!("CARGO_PKG_VERSION"),
                custom_ua
            )
        } else {
            format!("alloy-fireblocks/{}", env!("CARGO_PKG_VERSION"))
        }
    }

    /// Get Vault accounts
    pub async fn get_vault_accounts(&self) -> Result<Vec<u64>, FireblocksError> {
        // Ensure asset_id is populated
        let asset_id = self
            .config
            .asset_id
            .as_ref()
            .ok_or(FireblocksError::MissingAssetIDError())?;

        // Get paged vault accounts
        // TODO: deal with pagination
        let response = self.fireblocks.get_vaults().await?;

        // Filter and parse accounts
        let account_ids = response
            .accounts
            .into_iter()
            .filter(|account| account.assets.iter().any(|asset| asset.id == *asset_id))
            .map(|account| account.parse_id())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(account_ids)
    }

    /// Populate accounts with deposit addresses from Fireblocks
    pub async fn populate_accounts(&self) -> Result<(), FireblocksError> {
        // Get vault accounts from config or fetch them
        let vault_accounts = match &self.config.vault_account_ids {
            Some(ids) => ids.clone(),
            None => self.get_vault_accounts().await?,
        };

        // Get asset id from config
        let asset_id = self
            .config
            .asset_id
            .clone()
            .ok_or(FireblocksError::MissingAssetIDError())?;

        // Process each vault account
        let mut populated_accounts = HashMap::new();
        for vault_id in vault_accounts {
            // Get deposit addresses for this vault
            let deposit_addresses = self
                .fireblocks
                .get_deposit_address(&vault_id.to_string(), &asset_id)
                .await?;

            // If addresses exist, use the first one
            if let Some(first_address) = deposit_addresses.first() {
                // Parse the address string to Address type
                let address = first_address.address.parse::<Address>().map_err(|_| {
                    FireblocksError::InvalidAddressError(first_address.address.clone())
                })?;

                // Add to our map
                populated_accounts.insert(vault_id, address);
            }
        }

        // Update the shared accounts map
        if let Ok(mut accounts) = self.accounts.write() {
            accounts.extend(populated_accounts);
            Ok(())
        } else {
            Err(FireblocksError::SynchronizationError(
                "Failed to acquire write lock".to_string(),
            ))
        }
    }

    // /// Initialize account addresses
    // async fn init_accounts(&self) -> Result<(), TransportError> {
    //     let mut accounts = self.accounts.write();
    //     if !accounts.is_empty() {
    //         return Ok(());
    //     }

    //     let vault_ids = if let Some(ids) = &self.config.vault_account_ids {
    //         ids.clone()
    //     } else {
    //         // Fetch first 20 vault accounts if not specified
    //         self.fireblocks
    //             .get_vaults()
    //             .await?
    //             .accounts
    //             .into_iter()
    //             .map(|acc| acc.id.parse::<u64>().unwrap())
    //             .collect()
    //     };

    //     for vault_id in vault_ids {
    //         let addresses = self
    //             .fireblocks
    //             .get_deposit_address(vault_id.to_string(), self.config.asset_id.clone())
    //             .await?;

    //         if let Some(address) = addresses.first() {
    //             accounts.insert(vault_id, Address::from_str(&address.address)?);
    //         }
    //     }

    //     Ok(())
    // }

    // /// Create and submit a transaction via Fireblocks
    // async fn create_fireblocks_transaction(
    //     &self,
    //     tx: &TransactionRequest,
    // ) -> Result<B256, TransportError> {
    //     // Validate the transaction's chain ID
    //     if let Some(chain_id) = tx.chain_id {
    //         if chain_id != self.config.chain_id {
    //             return Err(TransportError::Custom("Chain ID mismatch".to_string()));
    //         }
    //     }

    //     // Get the source vault account
    //     let vault_id = self.get_vault_account_id(tx.from.unwrap_or_default())?;

    //     // Prepare transaction arguments
    //     let args = TransactionArguments {
    //         operation: if tx.data.is_some() {
    //             TransactionOperation::CONTRACT_CALL
    //         } else {
    //             TransactionOperation::TRANSFER
    //         },
    //         asset_id: self.config.asset_id.clone().unwrap(),
    //         source: vault_id.to_string(),
    //         destination: tx.to.map(|addr| addr.to_string()).unwrap_or_default(),
    //         amount: tx.value.unwrap_or_default().to_string(),
    //         note: self.config.note.clone(),
    //         external_tx_id: None,
    //         extra_parameters: tx.data.as_ref().map(|data| {
    //             json!({
    //                 "contractCallData": format!("0x{}", hex::encode(data))
    //             })
    //         }),
    //     };

    //     // Submit transaction and wait for completion
    //     let response = self.fireblocks.create_transaction(args).await?;

    //     // Poll for transaction completion
    //     let mut status = response.status;
    //     while !is_final_status(&status) {
    //         tokio::time::sleep(Duration::from_secs(1)).await;
    //         let info = self.fireblocks.get_transaction_by_id(response.id).await?;
    //         status = info.status;
    //     }

    //     if !is_successful_status(&status) {
    //         return Err(TransportError::Custom(format!(
    //             "Transaction failed with status: {:?}",
    //             status
    //         )));
    //     }

    //     // Return transaction hash
    //     Ok(B256::from_str(&response.tx_hash.unwrap_or_default())?)
    // }
}

// Helper functions
pub fn is_final_status(status: &TransactionStatus) -> bool {
    matches!(
        status,
        TransactionStatus::COMPLETED | TransactionStatus::FAILED | TransactionStatus::REJECTED
    )
}

pub fn is_successful_status(status: &TransactionStatus) -> bool {
    matches!(status, TransactionStatus::COMPLETED)
}
