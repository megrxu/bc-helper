use chrono::prelude::*;
use std::fmt::Display;

#[derive(Debug)]
pub enum TransactionStatus {
    Cleared,
    Pending,
}

#[derive(Debug)]
pub struct Transaction {
    pub datetime: NaiveDate,
    pub payee: String,
    pub note: String,
    pub tags: Vec<String>,
    pub refs: Vec<String>,
    pub items: Vec<Item>,
    pub status: TransactionStatus,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub amount: f32,
    pub unit: String,
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Cleared => write!(f, "*"),
            TransactionStatus::Pending => write!(f, "!"),
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} \"{}\" \"{}\"",
            self.datetime, self.status, self.note, self.payee
        )?;
        for tag in self.tags.iter() {
            write!(f, " #{}", tag)?;
        }
        for _ref in self.refs.iter() {
            write!(f, " ^{}", _ref)?;
        }
        write!(f, "\n")?;
        for item in self.items.iter() {
            writeln!(f, "  {}", item)?;
        }
        write!(f, "")
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<50}{:+>7.2} {}", self.name, self.amount, self.unit)
    }
}
