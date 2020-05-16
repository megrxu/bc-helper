use std::fmt::Display;

#[derive(Debug)]
pub enum TransactionStatus {
    Cleared,
    Pending,
}

#[derive(Debug)]
pub struct Transaction {
    pub datetime: String,
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
    pub amount: Option<f32>,
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

impl Transaction {
    pub fn export(&self) -> String {
        let mut res: String;
        res = format!(
            "\n{} {} \"{}\" \"{}\"",
            self.datetime, self.status, self.payee, self.note
        );
        for tag in self.tags.iter() {
            res += &format!(" {}", tag);
        }
        for _ref in self.refs.iter() {
            res += &format!(" {}", _ref);
        }
        res += "\n";
        for item in self.items.iter() {
            res += &format!("  {}\n", item.export());
        }
        res
    }

    pub fn markdown(&self) -> String {
        let mut res = String::default();
        res += &format!(
            "**{} {} \"{}\" \"{}\"",
            self.datetime,
            match self.status {
                TransactionStatus::Cleared => "Cleared",
                TransactionStatus::Pending => "Pending",
            },
            self.payee,
            self.note
        );
        for tag in self.tags.iter() {
            res += &format!(" {}", tag);
        }
        for _ref in self.refs.iter() {
            res += &format!(" {}", _ref);
        }
        res += "**\n```\n";
        for item in self.items.iter() {
            res += &format!("{}\n", item.markdown());
        }
        res += "```";
        res
    }
}

impl Item {
    pub fn markdown(&self) -> String {
        match self.amount {
            Some(amount) => format!("{:<30}{:+>7.2} {}", self.name, amount, self.unit),
            _ => format!("{}", self.name),
        }
    }

    pub fn export(&self) -> String {
        match self.amount {
            Some(amount) => format!("{:<50}{:+>7.2} {}", self.name, amount, self.unit),
            _ => format!("{}", self.name),
        }
    }
}
