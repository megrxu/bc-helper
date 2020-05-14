use bchelper_lib::helper::*;

#[test]
fn test_config() {
    let instance = HelperInstance::new("data/config.yml");
    assert_eq!(instance.default_config.currency, "CNY")
}
