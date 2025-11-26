use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepositSourceTriggerConditionInput {
    /// Amount conditions for the rule.
    #[serde(rename = "AmountConditions")]
    pub amount_conditions: Vec<models::AmountCondition>,
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    /// Payments network (prod/sandbox):  * Bitcoin/BitcoinTest  * Celo/CeloTestSepolia  * FlowEvm/FlowEvmTest  * Gnosis/GnosisTestChiado  * Lightning/LightningTest  * Ethereum/EthereumTestSepolia  * PolygonPos/PolygonTestAmoy  * Solana/SolanaDevnet  * OffNetwork/OffNetwork
    #[serde(rename = "Network")]
    pub network: String,
}

impl DepositSourceTriggerConditionInput {
    pub fn new(
        amount_conditions: Vec<models::AmountCondition>,
        crypto_currency: String,
        network: String,
    ) -> DepositSourceTriggerConditionInput {
        DepositSourceTriggerConditionInput {
            amount_conditions,
            crypto_currency,
            network,
        }
    }
}
