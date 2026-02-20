use sqlx::{PgPool, Result, Row};
use crate::db::models::Transaction;
use uuid::Uuid;

pub async fn insert_transaction(pool: &PgPool, tx: &Transaction) -> Result<Transaction> {
    let row = sqlx::query(
        "INSERT INTO transactions (id, stellar_account, amount, asset_code, status, created_at, updated_at, anchor_transaction_id, callback_type, callback_status) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
         RETURNING id, stellar_account, amount, asset_code, status, created_at, updated_at, anchor_transaction_id, callback_type, callback_status"
    )
    .bind(tx.id)
    .bind(&tx.stellar_account)
    .bind(&tx.amount)
    .bind(&tx.asset_code)
    .bind(&tx.status)
    .bind(tx.created_at)
    .bind(tx.updated_at)
    .bind(&tx.anchor_transaction_id)
    .bind(&tx.callback_type)
    .bind(&tx.callback_status)
    .fetch_one(pool)
    .await?;

    Ok(Transaction {
        id: row.get("id"),
        stellar_account: row.get("stellar_account"),
        amount: row.get("amount"),
        asset_code: row.get("asset_code"),
        status: row.get("status"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        anchor_transaction_id: row.get("anchor_transaction_id"),
        callback_type: row.get("callback_type"),
        callback_status: row.get("callback_status"),
    })
}

pub async fn get_transaction(pool: &PgPool, id: Uuid) -> Result<Transaction> {
    let row = sqlx::query(
        "SELECT id, stellar_account, amount, asset_code, status, created_at, updated_at, anchor_transaction_id, callback_type, callback_status 
         FROM transactions WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(Transaction {
        id: row.get("id"),
        stellar_account: row.get("stellar_account"),
        amount: row.get("amount"),
        asset_code: row.get("asset_code"),
        status: row.get("status"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        anchor_transaction_id: row.get("anchor_transaction_id"),
        callback_type: row.get("callback_type"),
        callback_status: row.get("callback_status"),
    })
}

pub async fn list_transactions(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Transaction>> {
    let rows = sqlx::query(
        "SELECT id, stellar_account, amount, asset_code, status, created_at, updated_at, anchor_transaction_id, callback_type, callback_status 
         FROM transactions ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let mut transactions = Vec::new();
    for row in rows {
        transactions.push(Transaction {
            id: row.get("id"),
            stellar_account: row.get("stellar_account"),
            amount: row.get("amount"),
            asset_code: row.get("asset_code"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            anchor_transaction_id: row.get("anchor_transaction_id"),
            callback_type: row.get("callback_type"),
            callback_status: row.get("callback_status"),
        });
    }

    Ok(transactions)
}
