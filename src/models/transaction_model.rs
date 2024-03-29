use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TransactionType {
    #[serde(rename = "c")]
    Credit,
    #[serde(rename = "d")]
    Debit,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(try_from = "String")]
pub struct Description(pub String);

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    #[serde(rename = "valor")]
    pub value: i32,
    #[serde(rename = "tipo")]
    pub transaction_type: TransactionType,
    #[serde(rename = "descricao")]
    pub description: Description,
    #[serde(rename = "realizada_em", default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}
