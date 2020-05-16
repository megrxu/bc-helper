use crate::transaction::*;
use crate::utils::*;
use chrono::prelude::*;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::read_to_string;

#[derive(Debug)]
pub enum ParseError {
    ParseTimeError,
    ParseItemError,
    ParseNotePayeeError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub currency: String,
    pub timezone: i64,
    pub operating_file: String,
    pub account_files: Vec<String>,
}

#[derive(Debug)]
pub struct HelperInstance {
    pub default_config: Config,
    pub namelist: Vec<String>,
    pub addr_map: HashMap<String, String>,
}

impl HelperInstance {
    pub fn new(config_path: &str) -> Self {
        let default_config: Config =
            serde_yaml::from_str(&(read_to_string(config_path).unwrap())).unwrap();
        let namelist = extract_namelist(&default_config.account_files);
        let addr_map = build_abbreviation_map(&namelist);
        HelperInstance {
            default_config,
            namelist,
            addr_map,
        }
    }

    pub fn parse_items(&self, line: &str) -> Result<Vec<Item>, ParseError> {
        let tokens = line.trim().split(",").collect::<Vec<&str>>();
        let mut items: Vec<Item> = vec![];
        let mut iter = tokens.iter();

        while let Some(item_str) = iter.next() {
            let tokens = item_str.trim().split(" ").collect::<Vec<&str>>();
            match tokens.len() {
                1 => {
                    if let Some(name) = self.addr_map.get(&tokens[0].to_lowercase()) {
                        items.push(Item {
                            name: name.to_string(),
                            amount: None,
                            unit: self.default_config.currency.to_string(),
                        })
                    }
                }
                2 => {
                    if let (Some(name), Ok(amount)) = (
                        self.addr_map.get(&tokens[0].to_lowercase()),
                        tokens[1].parse::<f32>(),
                    ) {
                        items.push(Item {
                            name: name.to_string(),
                            amount: Some(amount),
                            unit: self.default_config.currency.to_string(),
                        })
                    }
                }
                3 => {
                    if let (Some(name), Ok(amount)) = (
                        self.addr_map.get(&tokens[0].to_lowercase()),
                        tokens[1].parse::<f32>(),
                    ) {
                        items.push(Item {
                            name: name.to_string(),
                            amount: Some(amount),
                            unit: tokens[2].to_string(),
                        })
                    }
                }
                _ => return Err(ParseError::ParseItemError),
            }
        }
        Ok(items)
    }

    /// format: `[date |] [*/!] ["payee"] {"note"} [#tags] [^refs] {abbr} [amount] [unit], {abbr} [amount] [unit], {abbr} [amount] [unit]`
    /// 2020-02-20 | #Nintendo ccb 200 CNY > game 200 CNY
    /// inspired by [costflow](https://docs.costflow.io/syntax/) but more powerful
    pub fn parse(&self, line: &str) -> Result<Transaction, ParseError> {
        let unparsed_line: String = line.to_string();

        // Datetime
        let datetime: String;
        let tokens = unparsed_line.split("|").collect::<Vec<&str>>();
        let unparsed_line: String;
        if tokens.len() == 2 {
            datetime = match tokens[0] {
                _ => tokens[0].trim().to_string(),
            };
            unparsed_line = tokens[1].trim().to_string();
        } else {
            // default is the current time, based on the timezone in config file
            datetime = format!(
                "{}",
                (Utc::now() + Duration::hours(self.default_config.timezone)).format("%Y-%m-%d")
            );
            unparsed_line = tokens[0].trim().to_string();
        }

        // Status
        let status: TransactionStatus;
        // check for the first letter
        let unparsed_line = match unparsed_line.chars().nth(0) {
            Some('!') => {
                status = TransactionStatus::Pending;
                unparsed_line.chars().skip(1).collect::<String>()
            }
            Some('*') => {
                status = TransactionStatus::Cleared;
                unparsed_line.chars().skip(1).collect::<String>()
            }
            _ => {
                // default is cleared
                status = TransactionStatus::Cleared;
                unparsed_line
            }
        };

        // Note and Payee are included in '"'
        let tokens = unparsed_line.split("\"").collect::<Vec<&str>>();
        let note: String;
        let payee: String;
        let unparsed_line: String;

        if tokens.len() == 3 {
            // no note
            note = tokens.iter().nth(1).unwrap().to_string();
            payee = String::default();
            unparsed_line = tokens.iter().last().unwrap().trim().to_string();
        } else if tokens.len() == 5 {
            // with note
            payee = tokens.iter().nth(1).unwrap().to_string();
            note = tokens.iter().nth(3).unwrap().to_string();
            unparsed_line = tokens.iter().last().unwrap().trim().to_string();
        } else {
            return Err(ParseError::ParseNotePayeeError);
        }

        // Tags and Refs
        let mut tags = vec![];
        let mut refs = vec![];
        let tokens = unparsed_line.split(" ").collect::<Vec<&str>>();
        let mut token_iter = tokens.iter();
        let mut unparsed_line = String::default();
        while let Some(token) = token_iter.next() {
            if token.len() == 0 {
                break;
            }
            if token.chars().nth(0).unwrap() == '#' {
                tags.push(token.to_string());
            } else if token.chars().nth(0).unwrap() == '^' {
                refs.push(token.to_string());
            } else {
                unparsed_line += token;
                unparsed_line += " ";
                break;
            }
        }
        while let Some(token) = token_iter.next() {
            unparsed_line += token;
            unparsed_line += " ";
        }

        // Items
        let items = self.parse_items(&unparsed_line);
        if let Ok(items) = items {
            Ok(Transaction {
                datetime,
                payee,
                note,
                tags,
                refs,
                items,
                status,
            })
        } else {
            Err(ParseError::ParseItemError)
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ParseItemError => write!(f, "Parse item error."),
            ParseError::ParseNotePayeeError => write!(f, "Parse note or payee error."),
            ParseError::ParseTimeError => write!(f, "Parse time error."),
        }
    }
}
