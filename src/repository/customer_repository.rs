use std::sync::Arc;

use chrono::Utc;
use db::PrismaClient;

use crate::models::{
    customer_model::Customer,
    transaction_model::{Description, Transaction, TransactionType},
};

pub struct CustomerRepository {
    client: Arc<PrismaClient>,
}

impl CustomerRepository {
    pub fn new(client: &Arc<PrismaClient>) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn get_one(&self, id: i32) -> Result<Option<Customer>, Box<dyn std::error::Error>> {
        let customer_data = self
            .client
            .customer()
            .find_unique(db::customer::id::equals(id))
            .with(db::customer::transactions::fetch(vec![]))
            .exec()
            .await?;

        if customer_data.is_none() {
            return Ok(None);
        }

        let customer_data = customer_data.unwrap();

        let mut transactions: Vec<Transaction> = vec![];

        if !customer_data
            .clone()
            .transactions
            .unwrap_or(vec![])
            .is_empty()
        {
            for transaction_data in customer_data.transactions.unwrap() {
                transactions.push(Transaction {
                    created_at: Utc::now(),
                    description: Description(transaction_data.description),
                    id: transaction_data.id,
                    transaction_type: match transaction_data.transaction_type {
                        db::TransactionType::Credit => TransactionType::Credit,
                        db::TransactionType::Debit => TransactionType::Debit,
                    },
                    value: transaction_data.value,
                });
            }
        }

        Ok(Some(Customer {
            balance: customer_data.balance,
            limit: customer_data.limit,
            id: customer_data.id,
            transactions,
        }))
    }

    pub async fn update(
        &self,
        id: i32,
        update_params: Vec<db::customer::SetParam>,
    ) -> Result<Customer, Box<dyn std::error::Error>> {
        let customer_data = self
            .client
            .customer()
            .update(db::customer::id::equals(id), update_params)
            .with(db::customer::transactions::fetch(vec![]))
            .exec()
            .await?;

        let mut transactions: Vec<Transaction> = vec![];

        if !customer_data
            .transactions
            .clone()
            .unwrap_or(vec![])
            .is_empty()
        {
            for transaction_data in customer_data.transactions.unwrap() {
                transactions.push(Transaction {
                    created_at: Utc::now(),
                    description: Description(transaction_data.description),
                    id: transaction_data.id,
                    transaction_type: match transaction_data.transaction_type {
                        db::TransactionType::Credit => TransactionType::Credit,
                        db::TransactionType::Debit => TransactionType::Debit,
                    },
                    value: transaction_data.value,
                });
            }
        }

        Ok(Customer {
            balance: customer_data.balance,
            limit: customer_data.limit,
            id: customer_data.id,
            transactions,
        })
    }
}
