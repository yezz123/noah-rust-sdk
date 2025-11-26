use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    /// NOAH's unique identifier for the transaction.
    #[serde(rename = "ID")]
    pub id: uuid::Uuid,
    /// The public blockchain transaction id or hash. This identifier is only available for transactions that are broadcast to a public network as well as some private networks where available, e.g. Lightning
    #[serde(rename = "PublicID", skip_serializing_if = "Option::is_none")]
    pub public_id: Option<String>,
    /// Payments network (prod/sandbox):  * Bitcoin/BitcoinTest  * Celo/CeloTestSepolia  * FlowEvm/FlowEvmTest  * Gnosis/GnosisTestChiado  * Lightning/LightningTest  * Ethereum/EthereumTestSepolia  * PolygonPos/PolygonTestAmoy  * Solana/SolanaDevnet  * OffNetwork/OffNetwork
    #[serde(rename = "Network")]
    pub network: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "Status")]
    pub status: models::TransactionStatus,
    #[serde(rename = "Direction")]
    pub direction: models::TransactionDirection,
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// A unique identifier used in the business system to store a reference for the transaction. This field allows businesses to track and manage transactions within their internal systems.
    #[serde(rename = "ExternalID", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "NetworkFee", skip_serializing_if = "Option::is_none")]
    pub network_fee: Option<String>,
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    #[serde(rename = "FiatPayment", skip_serializing_if = "Option::is_none")]
    pub fiat_payment: Option<Box<models::FiatPayment>>,
    #[serde(rename = "Orchestration", skip_serializing_if = "Option::is_none")]
    pub orchestration: Option<Box<models::TransactionOrchestration>>,
    #[serde(rename = "FiatPaymentMethod", skip_serializing_if = "Option::is_none")]
    pub fiat_payment_method: Option<Box<models::PaymentMethod>>,
    /// This list explains how the transaction amount was calculated.
    #[serde(rename = "Breakdown", skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Vec<models::TransactionBreakdownItem>>,
    #[serde(rename = "AdjustmentFor", skip_serializing_if = "Option::is_none")]
    pub adjustment_for: Option<Box<models::TransactionAdjustment>>,
}

impl Transaction {
    pub fn new(
        id: uuid::Uuid,
        network: String,
        created: String,
        status: models::TransactionStatus,
        direction: models::TransactionDirection,
        crypto_currency: String,
    ) -> Transaction {
        Transaction {
            id,
            public_id: None,
            network,
            created,
            status,
            direction,
            customer_id: None,
            external_id: None,
            amount: None,
            network_fee: None,
            crypto_currency,
            fiat_payment: None,
            orchestration: None,
            fiat_payment_method: None,
            breakdown: None,
            adjustment_for: None,
        }
    }
}
