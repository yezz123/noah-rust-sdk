use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepositSourceTriggerCondition {
    /// Amount conditions for the rule.
    #[serde(rename = "AmountConditions")]
    pub amount_conditions: Vec<models::AmountCondition>,
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    /// Payments network (prod/sandbox):  * Bitcoin/BitcoinTest  * Celo/CeloTestSepolia  * FlowEvm/FlowEvmTest  * Gnosis/GnosisTestChiado  * Lightning/LightningTest  * Ethereum/EthereumTestSepolia  * PolygonPos/PolygonTestAmoy  * Solana/SolanaDevnet  * OffNetwork/OffNetwork
    #[serde(rename = "Network")]
    pub network: String,
    #[serde(rename = "DestinationAddress")]
    pub destination_address: Box<models::DestinationAddress>,
}

impl DepositSourceTriggerCondition {
    pub fn new(
        amount_conditions: Vec<models::AmountCondition>,
        crypto_currency: String,
        network: String,
        destination_address: models::DestinationAddress,
    ) -> DepositSourceTriggerCondition {
        DepositSourceTriggerCondition {
            amount_conditions,
            crypto_currency,
            network,
            destination_address: Box::new(destination_address),
        }
    }
}
