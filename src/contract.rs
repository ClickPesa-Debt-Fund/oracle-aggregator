use crate::{
    errors::OracleAggregatorErrors,
    storage::{self},
};
use sep_40_oracle::{Asset, PriceData, PriceFeedClient, PriceFeedTrait};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, Symbol, Vec};

#[contract]
pub struct OracleAggregator;

#[contractimpl]
impl PriceFeedTrait for OracleAggregator {
    fn resolution(e: Env) -> u32 {
        panic_with_error!(e, OracleAggregatorErrors::NotImplemented);
    }

    fn price(e: Env, _asset: Asset, _timestamp: u64) -> Option<PriceData> {
        panic_with_error!(e, OracleAggregatorErrors::NotImplemented);
    }

    fn prices(e: Env, _asset: Asset, _records: u32) -> Option<Vec<PriceData>> {
        panic_with_error!(e, OracleAggregatorErrors::NotImplemented);
    }

    fn base(e: Env) -> Asset {
        storage::get_base(&e)
    }

    fn decimals(e: Env) -> u32 {
        storage::get_decimals(&e)
    }

    fn assets(e: Env) -> Vec<Asset> {
        storage::get_assets(&e)
    }

    fn lastprice(e: Env, asset: Asset) -> Option<PriceData> {
        storage::extend_instance(&e);

        let usdc = storage::get_usdc(&e);
        let cpyt = storage::get_cpyt(&e);
        match asset {
            Asset::Stellar(ref a) if a.clone() == usdc || a.clone() == cpyt => {
                let oracle = PriceFeedClient::new(&e, &storage::get_oracle(&e));
                oracle.lastprice(&Asset::Other(Symbol::new(&e, "USDC")))
            }
            _ => {
                panic_with_error!(&e, OracleAggregatorErrors::AssetNotFound);
            }
        }
    }
}

#[contractimpl]
impl OracleAggregator {
    /// Initialize the contract with the admin and the oracle configurations
    ///
    /// ### Arguments
    /// * `admin` - The address of the admin
    /// * `base` - The base asset
    /// * `assets` - The list of supported assets
    /// * `asset_configs` - The list of oracle configurations for each asset
    ///
    /// ### Errors
    /// * `AlreadyInitialized` - The contract has already been initialized
    /// * `InvalidAssets` - The asset array is invalid
    /// * `InvalidOracleConfig` - The oracle config array is invalid
    pub fn initialize(e: Env, usdc: Address, cpyt: Address, usdc_oracle: Address) {
        if storage::get_is_init(&e) {
            panic_with_error!(&e, OracleAggregatorErrors::AlreadyInitialized);
        }

        storage::extend_instance(&e);
        storage::set_is_init(&e);
        storage::set_usdc(&e, &usdc);
        storage::set_cpyt(&e, &cpyt);
        storage::set_oracle(&e, &usdc_oracle);

        let usdc_oracle = PriceFeedClient::new(&e, &usdc_oracle);
        let base = usdc_oracle.base();
        let decimals = usdc_oracle.decimals();

        storage::set_base(&e, &base);
        storage::set_decimals(&e, &decimals);
    }
}
