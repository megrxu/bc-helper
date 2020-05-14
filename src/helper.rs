use crate::transaction::*;
use crate::utils::*;
use chrono::prelude::*;
use chrono::{NaiveDate, TimeZone};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs::read_to_string;

struct ParseTimeError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub currency: String,
    pub timezone: String,
    pub indent: usize,
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

    pub fn parse_items(&self, line: &str) -> Vec<Item> {
        vec![]
    }

    /// format: `[date] | [payee] {note} #tags ^refs {abbr} {amount}, {abbr} {amount}, {abbr} {amount} > {abbr} {amount}, {abbr} {amount}`
    /// yestoday | #Nintendo ccb 200 CNY, > game 200 CNY
    /// inspired by [costflow](https://docs.costflow.io/syntax/) but more powerful
    pub fn parse(&self, line: &str) -> Transaction {
        // Datetime
        let datetime: Date<Utc>;
        if line.contains("|") {
            datetime = Utc::today(); // TODO: parse from string
        } else {
            datetime = Utc::today();
        }

        // Note and Payee
        let note = String::default();
        let payee = String::default();

        // Tags and Refs
        let tags = vec![];
        let refs = vec![];

        // Items
        let items = vec![];

        // Status
        let status = TransactionStatus::Cleared;

        Transaction {
            datetime,
            payee,
            note,
            tags,
            refs,
            items,
            status,
        }
    }
}
