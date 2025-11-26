use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankDepositToOnchainAddressHostedRequest {
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID")]
    pub customer_id: String,
    /// The URL to which the user is redirected at the end of the Hosted Checkout session. We will include the CheckoutSessionID, ExternalID and Status of the session, ie `?CheckoutSessionID={CheckoutSessionID}&ExternalID={ExternalID}&Status={Status}`
    #[serde(rename = "ReturnURL")]
    pub return_url: String,
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    /// Payments network (prod/sandbox):  * Bitcoin/BitcoinTest  * Celo/CeloTestSepolia  * FlowEvm/FlowEvmTest  * Gnosis/GnosisTestChiado  * Lightning/LightningTest  * Ethereum/EthereumTestSepolia  * PolygonPos/PolygonTestAmoy  * Solana/SolanaDevnet  * OffNetwork/OffNetwork
    #[serde(rename = "Network")]
    pub network: String,
    #[serde(rename = "DestinationAddress")]
    pub destination_address: Box<models::DestinationAddress>,
    /// List of fiat options to be supported by the customer.
    #[serde(rename = "FiatOptions")]
    pub fiat_options: Vec<models::FiatOption>,
    /// Custom user defined key value pairs used for storing additional information.
    #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl BankDepositToOnchainAddressHostedRequest {
    pub fn new(
        customer_id: String,
        return_url: String,
        crypto_currency: String,
        network: String,
        destination_address: models::DestinationAddress,
        fiat_options: Vec<models::FiatOption>,
    ) -> BankDepositToOnchainAddressHostedRequest {
        BankDepositToOnchainAddressHostedRequest {
            customer_id,
            return_url,
            crypto_currency,
            network,
            destination_address: Box::new(destination_address),
            fiat_options,
            metadata: None,
        }
    }
}
