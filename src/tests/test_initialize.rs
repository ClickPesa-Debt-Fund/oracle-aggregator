#![cfg(test)]
use crate::testutils::{
    assert_assets_equal, create_oracle_aggregator, setup_default_aggregator, EnvTestUtils,
};
use sep_40_oracle::Asset;
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

#[test]
fn test_initalize() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    e.budget().reset_unlimited();
    let admin = Address::generate(&e);
    let base = Asset::Other(Symbol::new(&e, "BASE"));
    let address_0 = Address::generate(&e);
    let address_1 = Address::generate(&e);
    let asset_0 = Asset::Stellar(address_0.clone());
    let asset_1 = Asset::Stellar(address_1.clone());

    let (aggregator, oracle_aggregator_client) = create_oracle_aggregator(&e);
    let oracle_0_1 =
        setup_default_aggregator(&e, &aggregator, &admin, &base, &address_0, &address_1);
    oracle_aggregator_client.initialize(&address_0, &address_1, &oracle_0_1.address);
    assert!(assert_assets_equal(oracle_aggregator_client.base(), base));

    assert_eq!(oracle_aggregator_client.decimals(), 9);

    let assets = oracle_aggregator_client.assets();
    assert_eq!(oracle_aggregator_client.assets().len(), 2);
    assert_assets_equal(assets.get(0).unwrap(), asset_0.clone());
    assert_assets_equal(assets.get(1).unwrap(), asset_1.clone());
    let decimals = oracle_aggregator_client.decimals();
    assert_eq!(decimals, 9);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_already_initialized() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let base = Asset::Other(Symbol::new(&e, "BASE"));
    let address_0 = Address::generate(&e);
    let address_1 = Address::generate(&e);

    let (aggregator, oracle_aggregator_client) = create_oracle_aggregator(&e);
    let oracle_client =
        setup_default_aggregator(&e, &aggregator, &admin, &base, &address_0, &address_1);

    oracle_aggregator_client.initialize(&address_0, &address_1, &oracle_client.address);
    oracle_aggregator_client.initialize(&address_0, &address_1, &oracle_client.address);
}
