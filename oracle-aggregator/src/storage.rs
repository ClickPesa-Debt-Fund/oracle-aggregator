use soroban_sdk::{
    contracttype, unwrap::UnwrapOptimized, Address, Env, IntoVal, Symbol, TryFromVal, Val, Vec,
};

use crate::types::{Asset, OracleConfig, PriceData};

const ADMIN_KEY: &str = "Admin";
const IS_INIT_KEY: &str = "IsInit";
const ASSETS_KEY: &str = "Assets";
const BASE_KEY: &str = "Base";
const DECIMALS_KEY: &str = "Decimals";

const CIRCUIT_BREAKER_KEY: &str = "CircuitBreaker";
const VELOCITY_THRESHOLD_KEY: &str = "VelocityThreshold";
const STATUS_KEY: &str = "Status";
const TIMEOUT_KEY: &str = "Timeout";
const CIRCUIT_BREAKER_TIMEOUT_KEY: &str = "CircuitBreakerTimeout";

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5 seconds per ledger on average
const LEDGER_THRESHOLD_SHARED: u32 = 14 * ONE_DAY_LEDGERS;
const LEDGER_BUMP_SHARED: u32 = 15 * ONE_DAY_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub enum AggregatorDataKey {
    AssetConfig(Asset),
}

//********** Storage Utils **********//

/// Bump the instance lifetime by the defined amount
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD_SHARED, LEDGER_BUMP_SHARED);
}

/// Fetch an entry in persistent storage that has a default value if it doesn't exist
fn get_persistent_default<K: IntoVal<Env, Val>, V: TryFromVal<Env, Val>>(
    e: &Env,
    key: &K,
    default: V,
    bump_threshold: u32,
    bump_amount: u32,
) -> V {
    if let Some(result) = e.storage().persistent().get::<K, V>(key) {
        e.storage()
            .persistent()
            .extend_ttl(key, bump_threshold, bump_amount);
        result
    } else {
        default
    }
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

/// Get the admin address
pub fn get_admin(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, ADMIN_KEY))
        .unwrap()
}

/// Set the admin address
pub fn set_admin(e: &Env, admin: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, ADMIN_KEY), &admin);
}

/********** Persistent **********/

pub fn set_assets(e: &Env, assets: &Vec<Asset>) {
    e.storage()
        .persistent()
        .set::<Symbol, Vec<Asset>>(&Symbol::new(e, ASSETS_KEY), assets);
}

pub fn get_assets(e: &Env) -> Vec<Asset> {
    e.storage().persistent().extend_ttl(
        &Symbol::new(e, ASSETS_KEY),
        LEDGER_THRESHOLD_SHARED,
        LEDGER_BUMP_SHARED,
    );
    e.storage()
        .persistent()
        .get::<Symbol, Vec<Asset>>(&Symbol::new(e, ASSETS_KEY))
        .unwrap()
}

pub fn set_asset_config(e: &Env, asset: &Asset, config: &OracleConfig) {
    let key = AggregatorDataKey::AssetConfig(asset.clone());
    e.storage()
        .persistent()
        .set::<AggregatorDataKey, OracleConfig>(&key, config);
}

pub fn get_asset_config(e: &Env, asset: &Asset) -> OracleConfig {
    let key = AggregatorDataKey::AssetConfig(asset.clone());
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD_SHARED, LEDGER_BUMP_SHARED);
    e.storage().persistent().get(&key).unwrap_optimized()
}

pub fn has_asset_config(e: &Env, asset: &Asset) -> bool {
    let key = AggregatorDataKey::AssetConfig(asset.clone());
    e.storage().persistent().has(&key)
}

pub fn remove_asset_config(e: &Env, asset: &Asset) {
    let key = AggregatorDataKey::AssetConfig(asset.clone());
    e.storage().persistent().remove(&key);
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

pub fn set_circuit_breaker(e: &Env, enable: &bool) {
    e.storage()
        .persistent()
        .set::<Symbol, bool>(&Symbol::new(&e, CIRCUIT_BREAKER_KEY), &enable);
}

pub fn has_circuit_breaker(e: &Env) -> bool {
    e.storage().persistent().extend_ttl(
        &Symbol::new(&e, CIRCUIT_BREAKER_KEY),
        LEDGER_THRESHOLD_SHARED,
        LEDGER_BUMP_SHARED,
    );
    e.storage()
        .persistent()
        .get::<Symbol, bool>(&Symbol::new(&e, CIRCUIT_BREAKER_KEY))
        .unwrap_optimized()
}

pub fn set_velocity_threshold(e: &Env, threshold: &u32) {
    e.storage()
        .persistent()
        .set::<Symbol, u32>(&Symbol::new(&e, VELOCITY_THRESHOLD_KEY), threshold);
}

pub fn get_velocity_threshold(e: &Env) -> u32 {
    get_persistent_default(
        e,
        &Symbol::new(e, VELOCITY_THRESHOLD_KEY),
        0,
        LEDGER_THRESHOLD_SHARED,
        LEDGER_BUMP_SHARED,
    )
}

pub fn set_last_price(e: &Env, asset: &Asset, price: &PriceData) {
    e.storage()
        .persistent()
        .set::<Asset, PriceData>(asset, price);
}

pub fn get_last_price(e: &Env, asset: &Asset) -> Option<PriceData> {
    e.storage()
        .persistent()
        .extend_ttl(asset, LEDGER_THRESHOLD_SHARED, LEDGER_BUMP_SHARED);
    e.storage().persistent().get::<Asset, PriceData>(asset)
}

pub fn set_circuit_breaker_status(e: &Env, status: &bool) {
    e.storage()
        .persistent()
        .set::<Symbol, bool>(&Symbol::new(&e, STATUS_KEY), status);
}

pub fn get_circuit_breaker_status(e: &Env) -> bool {
    get_persistent_default(
        &e,
        &Symbol::new(&e, STATUS_KEY),
        false,
        LEDGER_THRESHOLD_SHARED,
        LEDGER_BUMP_SHARED,
    )
}

pub fn set_timeout(e: &Env, timeout: &u64) {
    e.storage()
        .persistent()
        .set::<Symbol, u64>(&Symbol::new(&e, TIMEOUT_KEY), timeout);
}

pub fn get_timeout(e: &Env) -> u64 {
    e.storage().persistent().extend_ttl(
        &Symbol::new(&e, TIMEOUT_KEY),
        LEDGER_THRESHOLD_SHARED,
        LEDGER_BUMP_SHARED,
    );
    e.storage()
        .persistent()
        .get::<Symbol, u64>(&Symbol::new(&e, CIRCUIT_BREAKER_TIMEOUT_KEY))
        .unwrap()
}

pub fn set_circuit_breaker_timeout(e: &Env, timeout: &u64) {
    e.storage()
        .persistent()
        .set::<Symbol, u64>(&Symbol::new(&e, CIRCUIT_BREAKER_TIMEOUT_KEY), timeout);
}

pub fn get_circuit_breaker_timeout(e: &Env) -> u64 {
    e.storage().persistent().extend_ttl(
        &Symbol::new(&e, CIRCUIT_BREAKER_TIMEOUT_KEY),
        LEDGER_THRESHOLD_SHARED,
        LEDGER_BUMP_SHARED,
    );
    e.storage()
        .persistent()
        .get::<Symbol, u64>(&Symbol::new(&e, CIRCUIT_BREAKER_TIMEOUT_KEY))
        .unwrap()
}
