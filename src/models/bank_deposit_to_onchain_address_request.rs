use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankDepositToOnchainAddressRequest {
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID")]
    pub customer_id: String,
    /// Supported fiat ISO_4217 3 letter currency codes.
    #[serde(rename = "FiatCurrency")]
    pub fiat_currency: String,
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    /// Payments network (prod/sandbox):  * Bitcoin/BitcoinTest  * Celo/CeloTestSepolia  * FlowEvm/FlowEvmTest  * Gnosis/GnosisTestChiado  * Lightning/LightningTest  * Ethereum/EthereumTestSepolia  * PolygonPos/PolygonTestAmoy  * Solana/SolanaDevnet  * OffNetwork/OffNetwork
    #[serde(rename = "Network")]
    pub network: String,
    #[serde(rename = "DestinationAddress")]
    pub destination_address: Box<models::DestinationAddress>,
}

impl BankDepositToOnchainAddressRequest {
    pub fn new(
        customer_id: String,
        fiat_currency: String,
        crypto_currency: String,
        network: String,
        destination_address: models::DestinationAddress,
    ) -> BankDepositToOnchainAddressRequest {
        BankDepositToOnchainAddressRequest {
            customer_id,
            fiat_currency,
            crypto_currency,
            network,
            destination_address: Box::new(destination_address),
        }
    }
}
