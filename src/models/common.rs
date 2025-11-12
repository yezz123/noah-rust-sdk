//! Common types and enums

use serde::{Deserialize, Serialize};

/// Positive decimal number (string representation)
pub type PositiveDecimal = String;

/// Date in ISO format (YYYY-MM-DD)
pub type Date = String;

/// DateTime in ISO 8601 format
pub type DateTime = String;

/// Country code (ISO 3166-1 alpha-2)
pub type CountryCode = String;

/// Fiat currency code (ISO 4217)
pub type FiatCurrencyCode = String;

/// Cryptocurrency code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum CryptoCurrencyCode {
    /// Bitcoin (production)
    #[serde(rename = "BTC")]
    Btc,
    /// Bitcoin (testnet)
    #[serde(rename = "BTC_TEST")]
    BtcTest,
    /// USDC (production)
    #[serde(rename = "USDC")]
    Usdc,
    /// USDC (testnet)
    #[serde(rename = "USDC_TEST")]
    UsdcTest,
}

impl std::fmt::Display for CryptoCurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoCurrencyCode::Btc => write!(f, "BTC"),
            CryptoCurrencyCode::BtcTest => write!(f, "BTC_TEST"),
            CryptoCurrencyCode::Usdc => write!(f, "USDC"),
            CryptoCurrencyCode::UsdcTest => write!(f, "USDC_TEST"),
        }
    }
}

/// Network identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Network {
    /// Bitcoin mainnet
    Bitcoin,
    /// Bitcoin testnet
    BitcoinTest,
    /// Ethereum mainnet
    Ethereum,
    /// Ethereum testnet (Sepolia)
    EthereumTestSepolia,
    /// Celo mainnet
    Celo,
    /// Celo testnet (Sepolia)
    CeloTestSepolia,
    /// Flow EVM mainnet
    FlowEvm,
    /// Flow EVM testnet
    FlowEvmTest,
    /// Gnosis mainnet
    Gnosis,
    /// Gnosis testnet (Chiado)
    GnosisTestChiado,
    /// Lightning mainnet
    Lightning,
    /// Lightning testnet
    LightningTest,
    /// Polygon PoS mainnet
    PolygonPos,
    /// Polygon PoS testnet (Amoy)
    PolygonTestAmoy,
    /// Solana mainnet
    Solana,
    /// Solana devnet
    SolanaDevnet,
    /// Off-network
    OffNetwork,
}

/// Address format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum AddressFormat {
    /// EVM-compatible address
    EvmCompatible,
    /// Segwit address
    Segwit,
    /// Tron Base58 address
    TronBase58,
    /// Solana Base58 address
    SolanaBase58,
}

/// Blockchain address
pub type Address = String;

/// Destination address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationAddress {
    /// The address
    #[serde(rename = "Address")]
    pub address: Address,
}

/// Fiat amount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiatAmount {
    /// Amount
    #[serde(rename = "Amount")]
    pub amount: PositiveDecimal,
    /// Fiat currency
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: FiatCurrencyCode,
}

/// Account type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum AccountType {
    /// Current account
    Current,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Current => write!(f, "Current"),
        }
    }
}

/// Payment method category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentMethodCategory {
    /// Bank payment methods
    Bank,
    /// Card payment methods
    Card,
    /// Identifier-based payment methods
    Identifier,
}

/// Payment method type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentMethodType {
    /// SEPA bank transfer
    BankSepa,
    /// Local bank transfer
    BankLocal,
    /// Fedwire bank transfer
    BankFedwire,
    /// Tokenized card
    TokenizedCard,
    /// PIX identifier
    IdentifierPix,
}

/// Processing tier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ProcessingTier {
    /// Standard processing
    Standard,
    /// Priority processing
    Priority,
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum TransactionStatus {
    /// Transaction is pending
    Pending,
    /// Transaction failed
    Failed,
    /// Transaction settled
    Settled,
}

/// Transaction direction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum TransactionDirection {
    /// Incoming transaction
    In,
    /// Outgoing transaction
    Out,
}

/// Sort direction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortDirection {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// Customer ID
pub type CustomerID = String;

/// Channel ID (UUID)
pub type ChannelID = String;

/// Payment method ID
pub type PaymentMethodID = String;

/// External ID
pub type ExternalID = String;

/// Form session ID (UUID)
pub type FormSessionID = String;

/// Nonce (unique transaction identifier)
pub type Nonce = String;

/// Phone number (E.164 format)
pub type PhoneNumber = String;

/// Return URL
pub type ReturnURL = String;
