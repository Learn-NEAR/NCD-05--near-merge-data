use near_sdk::serde_json::json;
use near_sdk_sim::to_yocto;
use near_sdk_sim::DEFAULT_GAS;

use crate::utils::init;
use crate::utils::print_burnt;
use crate::utils::to_gas;

#[test]
fn empty_data() {
    let (root, contract, _alice) = init();
    let result = root.call(
        contract.account_id(),
        "merge",
        &[].to_vec(),
        DEFAULT_GAS,
        0, // deposit
    );
    print_burnt(&result);
    assert_eq!(result.promise_errors().len(), 1);
    assert!(result.gas_burnt() <= to_gas("2.7"));
    assert!(result.tokens_burnt() <= to_yocto("0.0003"));
}

#[test]
fn empty_deposit() {
    let (root, contract, _alice) = init();
    let result = root.call(
        contract.account_id(),
        "merge",
        &json!({
            "data": [[1], [2]]
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    print_burnt(&result);
    assert_eq!(result.promise_errors().len(), 1);
    assert!(result.gas_burnt() <= to_gas("2.7"));
    assert!(result.tokens_burnt() <= to_yocto("0.0003"));
}

#[test]
fn merge() {
    let (root, contract, _alice) = init();
    let result = root.call(
        contract.account_id(),
        "merge",
        &json!({
            "data": [[2], [1]]
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        1, // deposit
    );
    let actual: Vec<u8> = result.unwrap_json();
    assert_eq!(vec![2, 1], actual);
    print_burnt(&result);
    assert!(result.gas_burnt() <= to_gas("3.3"));
    assert!(result.tokens_burnt() <= to_yocto("0.0004"));
}
