use std::collections::VecDeque;

use crate::shared::utils::ToDocument;

use super::transaction_model::Transaction;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub _id: Option<ObjectId>,
    pub id: i32,
    pub balance: i32,
    pub limit: i32,
    pub transactions: RingBuffer<Transaction>,
}

impl Default for Customer {
    fn default() -> Self {
        Self {
            id: 0,
            _id: Some(ObjectId::new()),
            balance: 0,
            limit: 0,
            transactions: RingBuffer::with_capacity(10),
        }
    }
}

impl ToDocument for Customer {}
