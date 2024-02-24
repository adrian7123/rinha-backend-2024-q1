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

    pub fn push(&mut self, item: T) {
        if self.0.len() >= self.0.capacity() {
            self.0.pop_back();
            self.0.push_front(item)
        } else {
            self.0.push_front(item)
        }
    }
}

#[derive(Default)]
pub struct Customer {
    pub balance: i64,
    pub limit: i64,
    pub transactions: RingBuffer<Transaction>,
}

impl Customer {
    pub fn new(limit: i64) -> Self {
        Self {
            balance: 0,
            limit,
            transactions: RingBuffer::default(),
        }
    }

    pub fn transact(&mut self, transaction: Transaction) -> Result<(), String> {
        match transaction.transaction_type {
            TransactionType::Credit => {
                self.balance += transaction.value;
                self.transactions.push(transaction);
                Ok(())
            }
            TransactionType::Debit => {
                if (self.balance + transaction.value).abs() > self.limit {
                    self.balance -= transaction.value;
                    self.transactions.push(transaction);
                    Ok(())
                } else {
                    Err(String::from("Não tem limite suficiente"))
                }
            }
        }
    }
}
