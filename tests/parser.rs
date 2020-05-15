use bchelper_lib::helper::*;

#[test]
fn test_parser() {
    let instance = HelperInstance::new("data/config.yml");
    assert_eq!(
        format!("{}", instance.parse("2020-01-01 | * \"支付宝\" \"收款\" ^Time #transaction ar 100 CNY, isr, alipay -200").unwrap()),
        r#"2020-01-01 * "支付宝" "收款" #transaction ^Time
  Assets:Receivables                                +100.00 CNY
  Income:Salary:Research
  Assets:Cash:Alipay                                -200.00 CNY
"#)
}
