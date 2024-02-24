use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename(deserialize = "c"))]
    Credit,
    #[serde(rename(deserialize = "d"))]
    Debit,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(try_from = "String")]
pub struct Description(String);

impl TryFrom<String> for Description {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 10 {
            Err("Descricao invalida")
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename(deserialize = "valor"))]
    pub value: i64,
    #[serde(rename(deserialize = "tipo"))]
    pub transaction_type: TransactionType,
    #[serde(rename(deserialize = "descricao"))]
    pub description: Description,
    #[serde(rename(deserialize = "realizada_em"), default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}
