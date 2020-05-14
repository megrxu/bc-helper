use bchelper_lib::utils::*;

#[test]
fn test_abbr() {
    let nl = extract_namelist(&vec![
        "data/account.bean".to_string(),
        "data/categories.bean".to_string(),
    ]);
    let map = build_abbreviation_map(&nl);
    assert_eq!(
        map.get(&"liang".to_string()).unwrap(),
        "Expenses:Gift:Liang"
    );
    assert_eq!(
        map.get(&"ee".to_string()).unwrap(),
        "Expenses:Entertainment"
    );
    assert_eq!(
        map.get(&"lccc".to_string()).unwrap(),
        "Liabilities:CreditCard:CN:CCB"
    );
}
