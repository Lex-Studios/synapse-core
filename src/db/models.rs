use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub stellar_account: String,
    pub amount: BigDecimal,
    pub asset_code: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub anchor_transaction_id: Option<String>,
    pub callback_type: Option<String>,
    pub callback_status: Option<String>,
}

impl Transaction {
    pub fn new(
        stellar_account: String,
        amount: BigDecimal,
        asset_code: String,
        anchor_transaction_id: Option<String>,
        callback_type: Option<String>,
        callback_status: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            stellar_account,
            amount,
            asset_code,
            status: "pending".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            anchor_transaction_id,
            callback_type,
            callback_status,
        }
    }
}
