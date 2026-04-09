use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub pack_id: String,
    pub user_id: String,
    pub amount: i64,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_payment_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorShare {
    pub share_rate: f64,
    pub packs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingData {
    pub transactions: Vec<Transaction>,
    pub creator_shares: std::collections::HashMap<String, CreatorShare>,
}
