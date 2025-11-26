use crate::models;
use serde::{Deserialize, Serialize};

/// TransactionOrchestration : Orchestration details for the transaction.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionOrchestration {
    /// Unique identifier for the rule that is matched for the transaction.
    #[serde(rename = "RuleID")]
    pub rule_id: String,
    /// Unique identifier for the execution of the rules that is matched for the transaction.
    #[serde(rename = "RuleExecutionID")]
    pub rule_execution_id: String,
}

impl TransactionOrchestration {
    /// Orchestration details for the transaction.
    pub fn new(rule_id: String, rule_execution_id: String) -> TransactionOrchestration {
        TransactionOrchestration {
            rule_id,
            rule_execution_id,
        }
    }
}
