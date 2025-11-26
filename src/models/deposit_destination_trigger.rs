use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepositDestinationTrigger {
    #[serde(rename = "Type")]
    pub r#type: Type,
    #[serde(rename = "Amount")]
    pub amount: String,
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    /// Payments network (prod/sandbox):  * Bitcoin/BitcoinTest  * Celo/CeloTestSepolia  * FlowEvm/FlowEvmTest  * Gnosis/GnosisTestChiado  * Lightning/LightningTest  * Ethereum/EthereumTestSepolia  * PolygonPos/PolygonTestAmoy  * Solana/SolanaDevnet  * OffNetwork/OffNetwork
    #[serde(rename = "Network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "RefundAddress", skip_serializing_if = "Option::is_none")]
    pub refund_address: Option<Box<models::DestinationAddress>>,
    #[serde(rename = "DepositAddress")]
    pub deposit_address: Box<models::DestinationAddress>,
}

impl DepositDestinationTrigger {
    pub fn new(
        r#type: Type,
        amount: String,
        crypto_currency: String,
        deposit_address: models::DestinationAddress,
    ) -> DepositDestinationTrigger {
        DepositDestinationTrigger {
            r#type,
            amount,
            crypto_currency,
            network: None,
            customer_id: None,
            refund_address: None,
            deposit_address: Box::new(deposit_address),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "DepositDestinationTrigger")]
    DepositDestinationTrigger,
}

impl Default for Type {
    fn default() -> Type {
        Self::DepositDestinationTrigger
    }
}
