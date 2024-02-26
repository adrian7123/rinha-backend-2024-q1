use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use super::transaction_model::{Transaction, TransactionType};

#[derive(Serialize, Deserialize)]
pub struct RingBuffer<T>(VecDeque<T>);

impl<T> Default for RingBuffer<T> {
    fn default() -> Self {
        Self::with_capacity(10)
    }
}

impl<T> RingBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(VecDeque::with_capacity(capacity))
    }

    #[allow(dead_code)]
    pub fn push(&mut self, item: T) {
        if self.0.len() >= self.0.capacity() {
            self.0.pop_back();
            self.0.push_front(item)
        } else {
            self.0.push_front(item)
        }
    }
}
fn default() -> i32 {
    0
}

#[derive(Default, Serialize, Deserialize)]
pub struct Customer {
    #[serde(default = "default")]
    pub id: i32,
    pub balance: i32,
    pub limit: i32,
    pub transactions: Vec<Transaction>,
}

impl Customer {
    pub fn transact(&self, transaction: Transaction) -> Result<Self, String> {
        let mut transactions = self.transactions.clone();
        match transaction.transaction_type {
            TransactionType::Credit => {
                transactions.push(transaction.clone());
                Ok(Self {
                    id: self.id,
                    limit: self.limit,
                    balance: self.balance + transaction.value,
                    transactions,
                })
            }
            TransactionType::Debit => {
                if self.balance + self.limit >= transaction.value {
                    transactions.push(transaction.clone());
                    Ok(Self {
                        id: self.id,
                        limit: self.limit,
                        balance: self.balance - transaction.value,
                        transactions,
                    })
                } else {
                    Err(String::from("Não tem limite suficiente"))
                }
            }
        }
    }
}
