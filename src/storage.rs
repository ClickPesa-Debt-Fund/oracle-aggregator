use sep_40_oracle::Asset;
use soroban_sdk::{vec, Address, Env, Symbol, Vec};

const IS_INIT_KEY: &str = "IsInit";

const BASE_KEY: &str = "Base";
const DECIMALS_KEY: &str = "Decimals";
const USDC_KEY: &str = "USDC";
const CPYT_KEY: &str = "CPYT";
const ORACLE_KEY: &str = "Oracle";

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5 seconds per ledger on average
const LEDGER_THRESHOLD_SHARED: u32 = 30 * ONE_DAY_LEDGERS;
const LEDGER_BUMP_SHARED: u32 = 31 * ONE_DAY_LEDGERS;

//********** Storage Utils **********//

/// Bump the instance lifetime by the defined amount
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD_SHARED, LEDGER_BUMP_SHARED);
}

/********** Instance **********/

/// Check if the contract has been initialized
pub fn get_is_init(e: &Env) -> bool {
    e.storage().instance().has(&Symbol::new(e, IS_INIT_KEY))
}

/// Set the contract as initialized
pub fn set_is_init(e: &Env) {
    e.storage()
        .instance()
        .set::<Symbol, bool>(&Symbol::new(e, IS_INIT_KEY), &true);
}

/********** Persistent **********/

pub fn get_assets(e: &Env) -> Vec<Asset> {
    vec![e, Asset::Stellar(get_usdc(e)), Asset::Stellar(get_cpyt(e))]
}
/// Get the usdc address
pub fn get_usdc(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, USDC_KEY))
        .unwrap()
}

/// Set the usdc address
pub fn set_usdc(e: &Env, usdc: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, USDC_KEY), &usdc);
}

pub fn get_cpyt(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, CPYT_KEY))
        .unwrap()
}

/// Set the cpyt address
pub fn set_cpyt(e: &Env, cpyt: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, CPYT_KEY), &cpyt);
}

pub fn set_base(e: &Env, base: &Asset) {
    e.storage()
        .persistent()
        .set::<Symbol, Asset>(&Symbol::new(e, BASE_KEY), base);
}

pub fn get_base(e: &Env) -> Asset {
    e.storage().persistent().extend_ttl(
        &Symbol::new(e, BASE_KEY),
        LEDGER_THRESHOLD_SHARED,
        LEDGER_BUMP_SHARED,
    );
    e.storage()
        .persistent()
        .get::<Symbol, Asset>(&Symbol::new(e, BASE_KEY))
        .unwrap()
}

pub fn set_decimals(e: &Env, decimals: &u32) {
    e.storage()
        .persistent()
        .set::<Symbol, u32>(&Symbol::new(e, DECIMALS_KEY), decimals);
}

pub fn get_decimals(e: &Env) -> u32 {
    e.storage().persistent().extend_ttl(
        &Symbol::new(e, DECIMALS_KEY),
        LEDGER_THRESHOLD_SHARED,
        LEDGER_BUMP_SHARED,
    );
    e.storage()
        .persistent()
        .get::<Symbol, u32>(&Symbol::new(e, DECIMALS_KEY))
        .unwrap()
}

pub fn set_oracle(e: &Env, oracle: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, ORACLE_KEY), &oracle);
}

pub fn get_oracle(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, ORACLE_KEY))
        .unwrap()
}
