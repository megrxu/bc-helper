use bchelper_lib::transaction::*;
use chrono::NaiveDate;
use std::str::FromStr;

#[test]
fn test_bean_export() {
    let t = Transaction {
        datetime: NaiveDate::from_str("2020-05-05").unwrap(),
        payee: String::from("微信"),
        note: String::from("提现"),
        tags: vec!["Nintendo".to_string(), "Fuck".to_string()],
        refs: vec!["Transaction".to_string()],
        items: vec![
            Item {
                name: "Assets:Bank:CN:CCB:UND".to_string(),
                amount: 200.0,
                unit: "CNY".to_string(),
            },
            Item {
                name: "Assets:Cash:Wechat".to_string(),
                amount: -200.0,
                unit: "CNY".to_string(),
            },
        ],
        status: TransactionStatus::Cleared,
    };
    assert_eq!(
        format!("{}", t),
        r#"2020-05-05 * "提现" "微信" #Nintendo #Fuck ^Transaction
  Assets:Bank:CN:CCB:UND                            +200.00 CNY
  Assets:Cash:Wechat                                -200.00 CNY
"#
    );
}
