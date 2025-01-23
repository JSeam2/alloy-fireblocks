#![allow(dead_code, non_camel_case_types)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultAccountPaginatedResponse {
    accounts: Vec<VaultAccountResponse>,
    paging: Paging,
    previous_url: Option<String>,
    next_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultAccountResponse {
    pub id: String,
    pub name: String,
    #[serde(rename = "hiddenOnUI")]
    pub hidden_on_ui: bool,
    pub assets: Vec<AssetResponse>,
    pub customer_ref_id: Option<String>,
    pub auto_fuel: bool,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagedVaultAccountsResponse {
    pub accounts: Vec<VaultAccountResponse>,
    pub paging: Option<Paging>,
    pub previous_url: Option<String>,
    pub next_url: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
    before: Option<String>,
    after: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVaultRequest {
    pub name: String,
    #[serde(rename = "hiddenOnUI")]
    pub hidden_on_ui: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_ref_id: Option<String>,
    #[serde(rename = "autoFuel")]
    pub auto_fuel: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVaultResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetTypeResponse {
    id: String,
    name: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(rename = "contractAddress")]
    contract_address: String,
    #[serde(rename = "nativeAsset")]
    native_asset: String,
    decimals: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponse {
    id: String,
    total: String,
    /// DEPRECATED
    balance: Option<String>,
    #[serde(rename = "lockedAmount")]
    locked_amount: Option<String>,
    available: Option<String>,
    pending: Option<String>,
    self_staked_cpu: Option<String>,
    self_staked_network: Option<String>,
    pending_refund_cpu: Option<String>,
    pending_refund_network: Option<String>,
    total_staked_cpu: Option<String>,
    total_staked_network: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetWalletsResponse {
    #[serde(rename = "vaultId")]
    pub vault_id: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub total: String,
    pub available: String,
    pub pending: String,
    pub staked: String,
    pub frozen: String,
    #[serde(rename = "lockedAmount")]
    pub locked_amount: String,
    #[serde(rename = "blockHeight")]
    pub block_height: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "creationTime")]
    pub creation_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnspentInputsResponse {
    pub address: String,
    pub input: Input,
    pub amount: String,
    pub confirmations: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub number: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetWalletsResponse {
    #[serde(rename = "assetWallets")]
    pub asset_wallets: Vec<AssetWalletsResponse>,
    pub paging: Paging,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddressResponse {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub address: String,
    pub tag: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "legacyAddress")]
    pub legacy_address: Option<String>,
    #[serde(rename = "customerRefId")]
    pub customer_ref_id: Option<String>,
    #[serde(rename = "addressFormat")]
    pub address_format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionArguments {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub operation: TransactionOperation,
    pub source: TransferPeerPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DestinationTransferPeerPath>,
    pub amount: String,
    //pub extra_parameters: Option<ExtraParameters>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub gas_price: Option<String>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub gas_limit: Option<String>,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtraParameters {
    ContractCallData(String),
    RawMessageData(RawMessageData),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferPeerPath {
    #[serde(rename = "type")]
    pub peer_type: PeerType,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinationTransferPeerPath {
    #[serde(rename = "type")]
    pub peer_type: PeerType,
    //  #[serde(skip_serializing_if = "Option::is_none")]
    pub id: String,
    //  #[serde(skip_serializing_if = "Option::is_none")]
    //  pub one_time_address: Option<OneTimeAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeAddress {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum TransactionOperation {
    TRANSFER,
    RAW,
    CONTRACT_CALL,

    MINT,
    BURN,
    SUPPLY_TO_COMPOUND,
    REDEEM_FROM_COMPOUND,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum PeerType {
    VAULT_ACCOUNT,
    EXCHANGE_ACCOUNT,
    INTERNAL_WALLET,
    EXTERNAL_WALLET,
    ONE_TIME_ADDRESS,
    NETWORK_CONNECTION,
    FIAT_ACCOUNT,
    COMPOUND,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionResponse {
    pub id: String,
    pub status: TransactionStatus,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionStatus {
    SUBMITTED,
    QUEUED,
    PENDING_SIGNATURE,
    PENDING_AUTHORIZATION,
    PENDING_3RD_PARTY_MANUAL_APPROVAL,
    PENDING_3RD_PARTY,
    PENDING,
    BROADCASTING,
    CONFIRMING,
    CONFIRMED,
    COMPLETED,
    PENDING_AML_SCREENING,
    PARTIALLY_COMPLETED,
    CANCELLING,
    CANCELLED,
    REJECTED,
    FAILED,
    TIMEOUT,
    BLOCKED,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetails {
    pub id: String,
    pub asset_id: String,

    pub tx_hash: String,
    pub status: TransactionStatus,
    pub sub_status: String,

    pub signed_messages: Vec<SignedMessageResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedMessageResponse {
    content: String,
    algorithm: String,
    derivation_path: Vec<usize>,
    pub signature: SignatureResponse,
    public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureResponse {
    pub full_sig: String,
    pub r: String,
    pub s: String,
    pub v: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawMessageData {
    pub messages: Vec<UnsignedMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsignedMessage {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestOptions {
    #[serde(rename = "idempotencyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ncw: Option<NCW>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NCW {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainId {
    MAINNET = 1,
    ROPSTEN = 3,
    KOVAN = 42,
    GOERLI = 5,
    RINKEBY = 4,
    SEPOLIA = 11155111,
    HOLESKY = 17000,
    BSC = 56,
    BSC_TEST = 97,
    POLYGON = 137,
    POLYGON_TEST = 80001,
    POLYGON_AMOY = 80002,
    AVALANCHE = 43114,
    AVALANCHE_TEST = 43113,
    MOONRIVER = 1285,
    MOONBEAM = 1284,
    SONGBIRD = 19,
    ARBITRUM = 42161,
    ARBITRUM_SEPOLIA = 421614,
    ARBITRUM_RIN = 421611,
    FANTOM = 250,
    RSK = 30,
    RSK_TEST = 31,
    CELO = 42220,
    CELO_BAK = 62320,
    CELO_ALF = 44787,
    OPTIMISM = 10,
    OPTIMISM_SEPOLIA = 11155420,
    OPTIMISM_KOVAN = 69,
    RONIN = 2020,
    CANTO = 7700,
    CANTO_TEST = 7701,
    POLYGON_ZKEVM_TEST = 1442,
    POLYGON_ZKEVM = 1101,
    KAVA = 2222,
    SMARTBCH = 10000,
    SMARTBCH_TEST = 10001,
    HECO = 128,
    AURORA = 1313161554,
    RISEOFTHEWARBOTSTESTNET = 7777,
    EVMOS = 9001,
    ASTAR = 592,
    VELAS = 106,
    ARB_GOERLI = 421613,
    XDC = 50,
    BASE = 8453,
    BASE_SEPOLIA = 84532,
    IVAR = 88888,
    JOC = 81,
    OASYS = 248,
    SHIMMEREVM = 148,
    LINEA = 59144,
    LINEA_TEST = 59140,
    FLARE = 14,
    MANTLE = 5000,
    MANTLE_TEST = 5001,
    BLAST = 81457,
    SONEIUM_MINATO = 1946,
    LACHAIN = 274,
}

/// Fireblocks Base Api Url
/// Documentation https://developers.fireblocks.com/reference/signing-a-request-jwt-structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiBaseUrl {
    Production,
    Sandbox,
    EU,
    EU2,
}

impl ApiBaseUrl {
    /// Returns the API Base URL value
    pub fn value(&self) -> &str {
        match self {
            ApiBaseUrl::Production => "https://api.fireblocks.io",
            ApiBaseUrl::Sandbox => "https://sandbox-api.fireblocks.io",
            ApiBaseUrl::EU => "https://eu-api.fireblocks.io/v1",
            ApiBaseUrl::EU2 => "https://eu2-api.fireblocks.io/v1",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Asset data
pub struct Asset {
    pub asset_id: String,
    pub rpc_url: String,
}

impl Asset {
    /// Get Asset via ChainId
    pub fn get_by_chain_id(chain_id: ChainId) -> Self {
        match chain_id {
            ChainId::MAINNET => Asset {
                asset_id: "ETH".to_string(),
                rpc_url: "https://cloudflare-eth.com".to_string(),
            },
            ChainId::ROPSTEN => Asset {
                asset_id: "ETH_TEST".to_string(),
                rpc_url: "https://rpc.ankr.com/eth_ropsten".to_string(),
            },
            ChainId::KOVAN => Asset {
                asset_id: "ETH_TEST2".to_string(),
                rpc_url: "https://kovan.poa.network".to_string(),
            },
            ChainId::GOERLI => Asset {
                asset_id: "ETH_TEST3".to_string(),
                rpc_url: "https://rpc.ankr.com/eth_goerli".to_string(),
            },
            ChainId::RINKEBY => Asset {
                asset_id: "ETH_TEST4".to_string(),
                rpc_url: "https://rpc.ankr.com/eth_rinkeby".to_string(),
            },
            ChainId::SEPOLIA => Asset {
                asset_id: "ETH_TEST5".to_string(),
                rpc_url: "https://rpc.sepolia.org".to_string(),
            },
            ChainId::HOLESKY => Asset {
                asset_id: "ETH_TEST6".to_string(),
                rpc_url: "https://ethereum-holesky-rpc.publicnode.com".to_string(),
            },
            ChainId::BSC => Asset {
                asset_id: "BNB_BSC".to_string(),
                rpc_url: "https://bsc-dataseed.binance.org".to_string(),
            },
            ChainId::BSC_TEST => Asset {
                asset_id: "BNB_TEST".to_string(),
                rpc_url: "https://data-seed-prebsc-1-s1.binance.org:8545".to_string(),
            },
            ChainId::POLYGON => Asset {
                asset_id: "MATIC_POLYGON".to_string(),
                rpc_url: "https://polygon-rpc.com".to_string(),
            },
            ChainId::POLYGON_TEST => Asset {
                asset_id: "MATIC_POLYGON_MUMBAI".to_string(),
                rpc_url: "https://rpc-mumbai.maticvigil.com".to_string(),
            },
            ChainId::POLYGON_AMOY => Asset {
                asset_id: "AMOY_POLYGON_TEST".to_string(),
                rpc_url: "https://rpc-amoy.polygon.technology".to_string(),
            },
            ChainId::AVALANCHE => Asset {
                asset_id: "AVAX".to_string(),
                rpc_url: "https://api.avax.network/ext/bc/C/rpc".to_string(),
            },
            ChainId::AVALANCHE_TEST => Asset {
                asset_id: "AVAXTEST".to_string(),
                rpc_url: "https://api.avax-test.network/ext/bc/C/rpc".to_string(),
            },
            ChainId::MOONRIVER => Asset {
                asset_id: "MOVR_MOVR".to_string(),
                rpc_url: "https://rpc.moonriver.moonbeam.network".to_string(),
            },
            ChainId::MOONBEAM => Asset {
                asset_id: "GLMR_GLMR".to_string(),
                rpc_url: "https://rpc.api.moonbeam.network".to_string(),
            },
            ChainId::SONGBIRD => Asset {
                asset_id: "SGB".to_string(),
                rpc_url: "https://songbird.towolabs.com/rpc".to_string(),
            },
            ChainId::ARBITRUM => Asset {
                asset_id: "ETH-AETH".to_string(),
                rpc_url: "https://rpc.ankr.com/arbitrum".to_string(),
            },
            ChainId::ARBITRUM_SEPOLIA => Asset {
                asset_id: "ETH-AETH_SEPOLIA".to_string(),
                rpc_url: "https://sepolia-rollup.arbitrum.io/rpc".to_string(),
            },
            ChainId::ARBITRUM_RIN => Asset {
                asset_id: "ETH-AETH-RIN".to_string(),
                rpc_url: "https://rinkeby.arbitrum.io/rpc".to_string(),
            },
            ChainId::FANTOM => Asset {
                asset_id: "FTM_FANTOM".to_string(),
                rpc_url: "https://rpc.ftm.tools/".to_string(),
            },
            ChainId::RSK => Asset {
                asset_id: "RBTC".to_string(),
                rpc_url: "https://public-node.rsk.co".to_string(),
            },
            ChainId::RSK_TEST => Asset {
                asset_id: "RBTC_TEST".to_string(),
                rpc_url: "https://public-node.testnet.rsk.co".to_string(),
            },
            ChainId::CELO => Asset {
                asset_id: "CELO".to_string(),
                rpc_url: "https://rpc.ankr.com/celo".to_string(),
            },
            ChainId::CELO_BAK => Asset {
                asset_id: "CELO_BAK".to_string(),
                rpc_url: "https://baklava-blockscout.celo-testnet.org/api/eth-rpc".to_string(),
            },
            ChainId::CELO_ALF => Asset {
                asset_id: "CELO_ALF".to_string(),
                rpc_url: "https://alfajores-forno.celo-testnet.org/api/eth-rpc".to_string(),
            },
            ChainId::OPTIMISM => Asset {
                asset_id: "ETH-OPT".to_string(),
                rpc_url: "https://rpc.ankr.com/optimism".to_string(),
            },
            ChainId::OPTIMISM_KOVAN => Asset {
                asset_id: "ETH-OPT_KOV".to_string(),
                rpc_url: "https://kovan.optimism.io/".to_string(),
            },
            ChainId::OPTIMISM_SEPOLIA => Asset {
                asset_id: "ETH-OPT_SEPOLIA".to_string(),
                rpc_url: "https://sepolia.optimism.io/".to_string(),
            },
            ChainId::RONIN => Asset {
                asset_id: "RON".to_string(),
                rpc_url: "https://api.roninchain.com/rpc".to_string(),
            },
            ChainId::CANTO => Asset {
                asset_id: "CANTO".to_string(),
                rpc_url: "https://canto.gravitychain.io".to_string(),
            },
            ChainId::CANTO_TEST => Asset {
                asset_id: "CANTO_TEST".to_string(),
                rpc_url: "https://testnet-archive.plexnode.wtf".to_string(),
            },
            ChainId::POLYGON_ZKEVM => Asset {
                asset_id: "ETH_ZKEVM".to_string(),
                rpc_url: "https://zkevm-rpc.com".to_string(),
            },
            ChainId::POLYGON_ZKEVM_TEST => Asset {
                asset_id: "ETH_ZKEVM_TEST".to_string(),
                rpc_url: "https://rpc.public.zkevm-test.net".to_string(),
            },
            ChainId::KAVA => Asset {
                asset_id: "KAVA_KAVA".to_string(),
                rpc_url: "https://evm.kava.io".to_string(),
            },
            ChainId::SMARTBCH => Asset {
                asset_id: "SMARTBCH".to_string(),
                rpc_url: "https://smartbch.greyh.at".to_string(),
            },
            ChainId::SMARTBCH_TEST => Asset {
                asset_id: "ETHW".to_string(),
                rpc_url: "https://rpc-testnet.smartbch.org".to_string(),
            },
            ChainId::HECO => Asset {
                asset_id: "HT_CHAIN".to_string(),
                rpc_url: "https://http-mainnet.hecochain.com".to_string(),
            },
            ChainId::AURORA => Asset {
                asset_id: "AURORA_DEV".to_string(),
                rpc_url: "https://mainnet.aurora.dev".to_string(),
            },
            ChainId::RISEOFTHEWARBOTSTESTNET => Asset {
                asset_id: "TKX".to_string(),
                rpc_url: "https://testnet1.rotw.games".to_string(),
            },
            ChainId::EVMOS => Asset {
                asset_id: "EVMOS".to_string(),
                rpc_url: "https://eth.bd.evmos.org".to_string(),
            },
            ChainId::ASTAR => Asset {
                asset_id: "ASTR_ASTR".to_string(),
                rpc_url: "https://evm.astar.network".to_string(),
            },
            ChainId::VELAS => Asset {
                asset_id: "VLX_VLX".to_string(),
                rpc_url: "https://evmexplorer.velas.com/rpc".to_string(),
            },
            ChainId::ARB_GOERLI => Asset {
                asset_id: "ETH-AETH_GOERLI".to_string(),
                rpc_url: "https://endpoints.omniatech.io/v1/arbitrum/goerli/public".to_string(),
            },
            ChainId::XDC => Asset {
                asset_id: "XDC".to_string(),
                rpc_url: "https://rpc.xdcrpc.com".to_string(),
            },
            ChainId::BASE => Asset {
                asset_id: "BASECHAIN_ETH".to_string(),
                rpc_url: "https://mainnet.base.org".to_string(),
            },
            ChainId::BASE_SEPOLIA => Asset {
                asset_id: "BASECHAIN_ETH_TEST5".to_string(),
                rpc_url: "https://sepolia.base.org".to_string(),
            },
            ChainId::IVAR => Asset {
                asset_id: "CHZ_CHZ2".to_string(),
                rpc_url: "https://mainnet-rpc.ivarex.com".to_string(),
            },
            ChainId::JOC => Asset {
                asset_id: "ASTR_TEST".to_string(),
                rpc_url: "https://rpc-1.japanopenchain.org:8545".to_string(),
            },
            ChainId::OASYS => Asset {
                asset_id: "OAS".to_string(),
                rpc_url: "https://oasys.blockpi.network/v1/rpc/public".to_string(),
            },
            ChainId::SHIMMEREVM => Asset {
                asset_id: "SMR_SMR".to_string(),
                rpc_url: "https://json-rpc.evm.shimmer.network".to_string(),
            },
            ChainId::LINEA => Asset {
                asset_id: "LINEA".to_string(),
                rpc_url: "https://rpc.linea.build".to_string(),
            },
            ChainId::LINEA_TEST => Asset {
                asset_id: "LINEA_TEST".to_string(),
                rpc_url: "https://rpc.goerli.linea.build".to_string(),
            },
            ChainId::FLARE => Asset {
                asset_id: "FLR".to_string(),
                rpc_url: "https://flare-api.flare.network/ext/C/rpc".to_string(),
            },
            ChainId::MANTLE => Asset {
                asset_id: "MANTLE".to_string(),
                rpc_url: "https://rpc.mantle.xyz".to_string(),
            },
            ChainId::MANTLE_TEST => Asset {
                asset_id: "MANTLE_TEST".to_string(),
                rpc_url: "https://rpc.testnet.mantle.xyz".to_string(),
            },
            ChainId::BLAST => Asset {
                asset_id: "BLAST".to_string(),
                rpc_url: "https://rpc.ankr.com/blast".to_string(),
            },
            ChainId::SONEIUM_MINATO => Asset {
                asset_id: "SONEIUM_MINATO_TEST".to_string(),
                rpc_url: "https://rpc.minato.soneium.org/".to_string(),
            },
            ChainId::LACHAIN => Asset {
                asset_id: "LAC".to_string(),
                rpc_url: "https://rpc1.mainnet.lachain.network".to_string(),
            },
        }
    }

    /// Method to update the RPC URL in case of custom RPCs
    pub fn set_rpc_url(&mut self, new_url: String) {
        self.rpc_url = new_url;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RawMessageType {
    EIP712,
    ETH_MESSAGE,
}

impl RawMessageType {
    /// Returns the str Value for FeeLevel
    pub fn value(&self) -> &str {
        match self {
            RawMessageType::EIP712 => "EIP712",
            RawMessageType::ETH_MESSAGE => "ETH_MESSAGE",
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeLevel {
    HIGH,
    MEDIUM,
    LOW,
}

impl FeeLevel {
    /// Returns the str Value for FeeLevel
    pub fn value(&self) -> &str {
        match self {
            FeeLevel::HIGH => "HIGH",
            FeeLevel::MEDIUM => "MEDIUM",
            FeeLevel::LOW => "LOW",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireblocksProviderConfig {
    // Mandatory fields
    /// API Key provided by Fireblocks
    pub api_key: String,
    /// Private key provided by Fireblocks
    pub private_key: String,
    /// ApiBaseUrl enum
    pub api_base_url: ApiBaseUrl,
    /// Chain Id
    pub chain_id: ChainId,

    // Optional fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_account_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_fee_level: Option<FeeLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub polling_interval: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_addresses_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_tx_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_transaction_status_changes: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_requests_and_responses: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_error_handling: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gasless_gas_tank_vault_id: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_path: Option<String>,
}

impl FireblocksProviderConfig {
    pub fn new(
        api_key: String,
        private_key: String,
        api_base_url: ApiBaseUrl,
        chain_id: ChainId,
    ) -> Self {
        let asset = Asset::get_by_chain_id(chain_id.clone());
        Self {
            // Required fields
            api_key,
            private_key,
            api_base_url,
            chain_id,
            // Optional fields with defaults
            rpc_url: Some(asset.rpc_url),
            vault_account_ids: None,
            fallback_fee_level: Some(FeeLevel::MEDIUM),
            note: Some("alloy-fireblocks Provider".into()),
            polling_interval: Some(1000),
            one_time_addresses_enabled: Some(true),
            external_tx_id: None,
            user_agent: None,
            asset_id: Some(asset.asset_id),
            log_transaction_status_changes: Some(false),
            log_requests_and_responses: Some(false),
            enhanced_error_handling: Some(true),
            gasless_gas_tank_vault_id: None,
            proxy_path: None,
        }
    }
    /// Builder pattern for rpc url
    pub fn with_rpc_url(mut self, rpc_url: String) -> Self {
        self.rpc_url = Some(rpc_url);
        self
    }

    /// Builder pattern for vault_account_ids
    pub fn with_vault_account_ids(mut self, ids: Vec<String>) -> Self {
        self.vault_account_ids = Some(ids);
        self
    }

    /// Builder pattern for fallback fee level
    pub fn with_fallback_fee_level(mut self, level: FeeLevel) -> Self {
        self.fallback_fee_level = Some(level);
        self
    }

    /// Builder pattern for note
    pub fn with_note(mut self, note: String) -> Self {
        self.note = Some(note);
        self
    }

    /// Builder pattern for polling interval
    pub fn with_polling_interval(mut self, polling_interval: u64) -> Self {
        self.polling_interval = Some(polling_interval);
        self
    }

    /// Builder pattern for one time addresses enabled
    pub fn with_one_time_addresses_enabled(mut self, one_time_addresses_enabled: bool) -> Self {
        self.one_time_addresses_enabled = Some(one_time_addresses_enabled);
        self
    }

    /// Builder pattern for external tx id
    pub fn with_external_tx_id(mut self, external_tx_id: String) -> Self {
        self.external_tx_id = Some(external_tx_id);
        self
    }

    /// Builder pattern for user agent
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    /// Builder pattern for asset id and RPC URL. Setting this will override defaults inferred from chain id
    pub fn with_asset_id(mut self, asset_id: String, rpc_url: String) -> Self {
        self.asset_id = Some(asset_id);
        self.rpc_url = Some(rpc_url);
        self
    }

    /// Builder pattern for log transaction status changes
    pub fn with_log_transaction_status_changes(
        mut self,
        log_transaction_status_changes: bool,
    ) -> Self {
        self.log_transaction_status_changes = Some(log_transaction_status_changes);
        self
    }

    /// Builder pattern for log requests and responses
    pub fn with_log_requests_and_responses(mut self, log_requests_and_responses: bool) -> Self {
        self.log_requests_and_responses = Some(log_requests_and_responses);
        self
    }

    /// Builder pattern for enhanced error handling
    pub fn with_enhanced_error_handling(mut self, enhanced_error_handling: bool) -> Self {
        self.enhanced_error_handling = Some(enhanced_error_handling);
        self
    }

    /// Builder pattern for gasless gas tank vault id
    pub fn with_gasless_gas_tank_vault_id(mut self, gasless_gas_tank_vault_id: u64) -> Self {
        self.gasless_gas_tank_vault_id = Some(gasless_gas_tank_vault_id);
        self
    }

    /// Builder pattern for proxy path
    pub fn with_proxy_path(mut self, proxy_path: String) -> Self {
        self.proxy_path = Some(proxy_path);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestArguments<T> {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<T>,
}

#[derive(Debug)]
pub struct ProviderRpcError {
    pub message: String,
    pub code: i32,
    pub data: Option<String>,
    pub payload: RequestArguments<serde_json::Value>,
}

impl std::error::Error for ProviderRpcError {}

impl std::fmt::Display for ProviderRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RPC Error {}: {}", self.code, self.message)
    }
}
