use std::sync::Arc;

use alloy_core::primitives::Address;
use alloy_provider::ProviderBuilder;
use alloy_transport::TransportError;

use alloy_fireblocks::{
    api::FireblocksClient, provider::FireblocksProvider, types::FireblocksProviderConfig,
};

// Helper to create test configuration
fn test_config(user_agent: Option<&str>) -> FireblocksProviderConfig {
    FireblocksProviderConfig {
        private_key: "test-key".into(),
        api_key: "test-api-key".into(),
        rpc_url: "http://localhost:8545".parse().unwrap(),
        api_base_url: "https://api.fireblocks.io".into(),
        user_agent: user_agent.map(Into::into),
    }
}

#[tokio::test]
async fn test_provider_creation() -> Result<(), TransportError> {
    let config = test_config(None);
    let provider = FireblocksProvider::new(config).await?;

    assert!(Arc::strong_count(&provider.inner) == 1);
    Ok(())
}

#[tokio::test]
async fn test_user_agent() {
    let test_cases = vec![
        (
            Some("custom-ua"),
            format!("alloy-fireblocks/{} custom-ua", env!("CARGO_PKG_VERSION")),
        ),
        (
            None,
            format!("alloy-fireblocks/{}", env!("CARGO_PKG_VERSION")),
        ),
    ];

    for (input, expected) in test_cases {
        let config = test_config(input);
        let provider = FireblocksProvider::new(config).await.unwrap();
        assert_eq!(provider.get_user_agent(), expected);
    }
}

#[tokio::test]
async fn test_account_caching() {
    let config = test_config(None);
    let provider = FireblocksProvider::new(config).await.unwrap();

    // Test account caching functionality
    let account_id = 1;
    let address = Address::parse_checksummed("0x52908400098527886E0F7030069857D2E4169EE7", None);

    {
        let mut accounts = provider.accounts.write().unwrap();
        accounts.insert(account_id, address);
    }

    {
        let accounts = provider.accounts.read().unwrap();
        assert_eq!(accounts.get(&account_id), Some(&address));
    }
}
