use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FiatPaymentRefund {
    #[serde(rename = "RefundID")]
    pub refund_id: uuid::Uuid,
    #[serde(rename = "RefundedAmount")]
    pub refunded_amount: Box<models::FiatAmount>,
    #[serde(rename = "RequestedTime")]
    pub requested_time: String,
    #[serde(rename = "Status")]
    pub status: models::FiatPaymentStatus,
}

impl FiatPaymentRefund {
    pub fn new(
        refund_id: uuid::Uuid,
        refunded_amount: models::FiatAmount,
        requested_time: String,
        status: models::FiatPaymentStatus,
    ) -> FiatPaymentRefund {
        FiatPaymentRefund {
            refund_id,
            refunded_amount: Box::new(refunded_amount),
            requested_time,
            status,
        }
    }
}
