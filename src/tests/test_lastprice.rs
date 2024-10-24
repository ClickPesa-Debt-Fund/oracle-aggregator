#![cfg(test)]

use crate::testutils::{create_oracle_aggregator, setup_default_aggregator, EnvTestUtils};
use sep_40_oracle::Asset;
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

#[test]
fn test_lastprice() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let base = Asset::Other(Symbol::new(&e, "BASE"));
    let address_0 = Address::generate(&e);
    let address_1 = Address::generate(&e);
    let asset_0 = Asset::Stellar(address_0.clone());
    let asset_1 = Asset::Stellar(address_1.clone());

    let (aggregator, oracle_aggregator_client) = create_oracle_aggregator(&e);
    let oracle_client =
        setup_default_aggregator(&e, &aggregator, &admin, &base, &address_0, &address_1);
    oracle_aggregator_client.initialize(&address_0, &address_1, &oracle_client.address);
    let price_0 = oracle_aggregator_client.lastprice(&asset_0);
    match price_0 {
        Some(price) => {
            assert_eq!(price.price, 1_000000000);
            assert_eq!(price.timestamp, e.ledger().timestamp());
        }
        None => {
            assert!(false)
        }
    }

    let price_1 = oracle_aggregator_client.lastprice(&asset_1);
    match price_1 {
        Some(price) => {
            assert_eq!(price.price, 1_000000000);
            assert_eq!(price.timestamp, e.ledger().timestamp());
        }
        None => {
            assert!(false)
        }
    }
}

#[test]
#[should_panic(expected = "Error(Contract, #105)")]
fn test_lastprice_asset_not_found() {
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

    oracle_aggregator_client.lastprice(&Asset::Other(Symbol::new(&e, "NOT_FOUND")));
}

#[test]
#[should_panic(expected = "Error(Contract, #105)")]
fn test_lastprice_asset_not_found_stellar() {
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

    oracle_aggregator_client.lastprice(&Asset::Stellar(Address::generate(&e)));
}
