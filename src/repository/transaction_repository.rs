use std::sync::Arc;

use chrono::Utc;
use db::PrismaClient;

use crate::models::transaction_model::{Description, Transaction, TransactionType};

pub struct TransactionRepository {
    client: Arc<PrismaClient>,
}

impl TransactionRepository {
    pub fn new(client: &Arc<PrismaClient>) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn create(
        &self,
        transaction: Transaction,
        customer_id: i32,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        let data = self
            .client
            .transaction()
            .create(
                transaction.value,
                match transaction.transaction_type {
                    TransactionType::Credit => db::TransactionType::Credit,
                    TransactionType::Debit => db::TransactionType::Debit,
                },
                transaction.description.0,
                db::customer::id::equals(customer_id),
                vec![],
            )
            .exec()
            .await?;

        Ok(Transaction {
            created_at: Utc::now(),
            description: Description(data.description),
            id: data.id,
            transaction_type: match data.transaction_type {
                db::TransactionType::Credit => TransactionType::Credit,
                db::TransactionType::Debit => TransactionType::Debit,
            },
            value: data.value,
        })
    }
}
