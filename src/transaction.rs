use chrono::prelude::*;

#[derive(Debug)]
pub enum TransactionStatus {
    Cleared,
    Pending,
}

#[derive(Debug)]
pub struct Transaction {
    pub datetime: Date<Utc>,
    pub payee: String,
    pub note: String,
    pub tags: Vec<String>,
    pub refs: Vec<String>,
    pub items: Vec<Item>,
    pub status: TransactionStatus,
}

#[derive(Debug)]
pub struct Amount {
    pub number: u32,
    pub currency: String,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub amount: Amount,
}
