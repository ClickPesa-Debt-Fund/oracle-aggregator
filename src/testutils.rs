#![cfg(test)]

use crate::contract::OracleAggregatorClient;
use sep_40_oracle::{
    testutils::{Asset as MockAsset, MockPriceOracleClient, MockPriceOracleWASM},
    Asset,
};
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Env, Symbol, Vec,
};
pub mod oracle_aggregator {
    soroban_sdk::contractimport!(
        file = "./target/wasm32-unknown-unknown/optimized/oracle_aggregator.wasm"
    );
}

// use crate::mock_oracle::{MockOracle, MockPriceOracleClient};

const ONE_DAY_LEDGERS: u32 = 24 * 60 * 60 / 5;

pub trait EnvTestUtils {
    /// Jump the env by the given amount of ledgers. Assumes 5 seconds per ledger.
    fn jump(&self, ledgers: u32);

    /// Set the ledger to the default LedgerInfo
    ///
    /// Time -> 1441065600 (Sept 1st, 2015 12:00:00 AM UTC)
    /// Sequence -> 100
    fn set_default_info(&self);
}

impl EnvTestUtils for Env {
    fn jump(&self, ledgers: u32) {
        self.ledger().set(LedgerInfo {
            timestamp: self.ledger().timestamp().saturating_add(ledgers as u64 * 5),
            protocol_version: 20,
            sequence_number: self.ledger().sequence().saturating_add(ledgers),
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 50 * ONE_DAY_LEDGERS,
            min_persistent_entry_ttl: 50 * ONE_DAY_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_LEDGERS,
        });
    }

    fn set_default_info(&self) {
        self.ledger().set(LedgerInfo {
            timestamp: 1441065600, // Sept 1st, 2015 12:00:00 AM UTC
            protocol_version: 20,
            sequence_number: 100,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 50 * ONE_DAY_LEDGERS,
            min_persistent_entry_ttl: 50 * ONE_DAY_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_LEDGERS,
        });
    }
}

/// Deploy an oracle aggreator contract
pub fn create_oracle_aggregator<'a>(e: &Env) -> (Address, OracleAggregatorClient<'a>) {
    let oracle_aggregator_address = e.register_contract_wasm(None, oracle_aggregator::WASM);
    let oracle_aggregator_client: OracleAggregatorClient<'a> =
        OracleAggregatorClient::new(&e, &oracle_aggregator_address);
    return (oracle_aggregator_address, oracle_aggregator_client);
}

/// Setup an oracle aggregator with default test setttings based on the current env timestamp.
///
/// ### Returns
/// Two oracle aggegator clients:
/// - (0 and 1 oracle, 2 oracle)
pub fn setup_default_aggregator<'a>(
    e: &Env,
    aggregator: &Address,
    admin: &Address,
    base: &Asset,
    address_0: &Address,
    address_1: &Address,
) -> (MockPriceOracleClient<'a>) {
    // setup oracle with XLM and USDC proce
    let oracle_0_1_id = e.register_contract_wasm(None, MockPriceOracleWASM);
    let oracle_0_1 = MockPriceOracleClient::new(&e, &oracle_0_1_id);
    oracle_0_1.set_data(
        &Address::generate(&e),
        &MockAsset::Other(Symbol::new(&e, "BASE")),
        &Vec::from_array(
            &e,
            [
                MockAsset::Stellar(address_1.clone()),
                MockAsset::Other(Symbol::new(&e, "USDC")),
            ],
        ),
        &9,
        &300,
    );
    oracle_0_1.set_price(
        &Vec::from_array(&e, [0_120000000, 1_010000000]),
        &(e.ledger().timestamp() - 300 * 2),
    );
    oracle_0_1.set_price(
        &Vec::from_array(&e, [0_100000000, 0_990000000]),
        &(e.ledger().timestamp() - 300),
    );
    oracle_0_1.set_price(
        &Vec::from_array(&e, [0_110000000, 1_000000000]),
        &e.ledger().timestamp(),
    );

    return (oracle_0_1);
}

pub fn assert_assets_equal(a: Asset, b: Asset) -> bool {
    match (a, b) {
        (Asset::Stellar(a), Asset::Stellar(b)) => a == b,
        (Asset::Other(a), Asset::Other(b)) => a == b,
        _ => false,
    }
}
