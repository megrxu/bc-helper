use bchelper_lib::transaction::*;

#[test]
fn test_bean_export() {
    let t = Transaction {
        datetime: "2020-05-05".to_string(),
        payee: String::from("微信"),
        note: String::from("提现"),
        tags: vec!["#Wechat".to_string(), "#Cash".to_string()],
        refs: vec!["^Transaction".to_string()],
        items: vec![
            Item {
                name: "Assets:Bank:CN:CCB:UND".to_string(),
                amount: Some(200.0),
                unit: "CNY".to_string(),
            },
            Item {
                name: "Assets:Cash:Wechat".to_string(),
                amount: None,
                unit: "CNY".to_string(),
            },
        ],
        status: TransactionStatus::Cleared,
    };
    assert_eq!(
        t.export(),
        r#"
2020-05-05 * "微信" "提现" #Wechat #Cash ^Transaction
  Assets:Bank:CN:CCB:UND                            +200.00 CNY
  Assets:Cash:Wechat
"#
    );
}
