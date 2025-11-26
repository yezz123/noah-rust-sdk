use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnchainDepositSourceTriggerConditionInput {
    /// Amount conditions for the rule.
    #[serde(rename = "AmountConditions")]
    pub amount_conditions: Vec<models::AmountCondition>,
    /// Payments network (prod/sandbox):  * Bitcoin/BitcoinTest  * Celo/CeloTestSepolia  * FlowEvm/FlowEvmTest  * Gnosis/GnosisTestChiado  * Lightning/LightningTest  * Ethereum/EthereumTestSepolia  * PolygonPos/PolygonTestAmoy  * Solana/SolanaDevnet  * OffNetwork/OffNetwork
    #[serde(rename = "Network")]
    pub network: String,
}

impl OnchainDepositSourceTriggerConditionInput {
    pub fn new(
        amount_conditions: Vec<models::AmountCondition>,
        network: String,
    ) -> OnchainDepositSourceTriggerConditionInput {
        OnchainDepositSourceTriggerConditionInput {
            amount_conditions,
            network,
        }
    }
}
