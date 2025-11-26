use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnchainDepositToPaymentMethodHostedRequest {
    /// A unique ID which identifies the customer in the Business' internal system and in NOAH.
    #[serde(rename = "CustomerID")]
    pub customer_id: String,
    /// The URL to which the user is redirected at the end of the Hosted Checkout session. We will include the CheckoutSessionID, ExternalID and Status of the session, ie `?CheckoutSessionID={CheckoutSessionID}&ExternalID={ExternalID}&Status={Status}`
    #[serde(rename = "ReturnURL")]
    pub return_url: String,
    /// Unique identifier for the channel.
    #[serde(rename = "ChannelID")]
    pub channel_id: uuid::Uuid,
    #[serde(rename = "FiatAmount", skip_serializing_if = "Option::is_none")]
    pub fiat_amount: Option<String>,
    /// Payments network (prod/sandbox):  * Bitcoin/BitcoinTest  * Celo/CeloTestSepolia  * FlowEvm/FlowEvmTest  * Gnosis/GnosisTestChiado  * Lightning/LightningTest  * Ethereum/EthereumTestSepolia  * PolygonPos/PolygonTestAmoy  * Solana/SolanaDevnet  * OffNetwork/OffNetwork
    #[serde(rename = "Network")]
    pub network: String,
    /// Cryptocurrency (prod/sandbox):  * BTC/BTC_TEST  * USDC/USDC_TEST
    #[serde(rename = "CryptoCurrency")]
    pub crypto_currency: String,
    /// A unique identifier used in the business system to store a reference for the transaction. This field allows businesses to track and manage transactions within their internal systems.
    #[serde(rename = "ExternalID", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Custom user defined key value pairs used for storing additional information.
    #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl OnchainDepositToPaymentMethodHostedRequest {
    pub fn new(
        customer_id: String,
        return_url: String,
        channel_id: uuid::Uuid,
        network: String,
        crypto_currency: String,
    ) -> OnchainDepositToPaymentMethodHostedRequest {
        OnchainDepositToPaymentMethodHostedRequest {
            customer_id,
            return_url,
            channel_id,
            fiat_amount: None,
            network,
            crypto_currency,
            external_id: None,
            metadata: None,
        }
    }
}
