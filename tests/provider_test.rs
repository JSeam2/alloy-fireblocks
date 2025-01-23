use std::sync::Arc;

use alloy_core::primitives::Address;
use alloy_provider::ProviderBuilder;
use alloy_signer::k256::elliptic_curve::PrimeCurveArithmetic;
use alloy_transport::TransportError;

use alloy_fireblocks::{
    api::FireblocksClient,
    provider::FireblocksProvider,
    types::{ApiBaseUrl, ChainId, FireblocksProviderConfig},
};
use tokio::fs;

// Helper to create test configuration
async fn test_config() -> FireblocksProviderConfig {
    // Read API key from file
    let api_key_content = fs::read_to_string("API_KEY").await.unwrap();
    let api_key_trimmed = api_key_content.trim().to_string();

    // Read private key from file
    let private_key_content = fs::read_to_string("PRIVATE_KEY").await.unwrap();
    let private_key_trimmed = private_key_content.trim().to_string();

    FireblocksProviderConfig::new(
        api_key_trimmed,
        private_key_trimmed,
        ApiBaseUrl::Sandbox,
        ChainId::SEPOLIA,
    )
}

#[tokio::test]
async fn test_provider_creation() -> Result<(), TransportError> {
    let config = test_config().await;
    let provider = FireblocksProvider::new(config).await?;

    assert!(Arc::strong_count(&provider.inner) == 1);
    Ok(())
}

#[tokio::test]
async fn test_user_agent() {
    let test_cases = vec![
        (
            Some("test"),
            format!("alloy-fireblocks/{} test", env!("CARGO_PKG_VERSION")),
        ),
        (
            None,
            format!("alloy-fireblocks/{}", env!("CARGO_PKG_VERSION")),
        ),
    ];

    let config = test_config().await;
    let provider = FireblocksProvider::new(config.clone()).await.unwrap();
    assert_eq!(
        provider.get_user_agent(),
        format!("alloy-fireblocks/{}", env!("CARGO_PKG_VERSION"))
    );

    let config = config.with_user_agent("test".to_string());
    let provider = FireblocksProvider::new(config).await.unwrap();
    assert_eq!(
        provider.get_user_agent(),
        format!("alloy-fireblocks/{} test", env!("CARGO_PKG_VERSION"))
    );
}

#[tokio::test]
async fn test_account_caching() {
    let config = test_config().await;
    let provider = FireblocksProvider::new(config).await.unwrap();

    // Test account caching functionality
    let account_id = 1;
    let address =
        Address::parse_checksummed("0x52908400098527886E0F7030069857D2E4169EE7", None).unwrap();

    {
        let mut accounts = provider.accounts.write().unwrap();
        accounts.insert(account_id, address);
    }

    {
        let accounts = provider.accounts.read().unwrap();
        assert_eq!(accounts.get(&account_id), Some(&address));
    }
}
