#![allow(clippy::too_many_arguments)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, near_bindgen, AccountId, Balance};

near_sdk::setup_alloc!();

pub mod callback_args;
pub mod data_request;
pub mod fee_config;
mod fungible_token_receiver;
mod helpers;
mod logger;
pub mod oracle_config;
mod requester_handler;
mod resolution_window;
mod storage_manager;
pub mod types;
mod upgrade;
pub mod whitelist;

/// Mocks
mod fungible_token;

pub use callback_args::*;

pub use data_request::{DataRequest, Source};
pub use requester_handler::Requester;
use storage_manager::AccountStorageBalance;
use types::*;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    pub whitelist: whitelist::Whitelist,
    pub configs: Vector<oracle_config::OracleConfig>,
    pub data_requests: Vector<DataRequest>,
    pub accounts: LookupMap<AccountId, AccountStorageBalance>, // storage map
}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"Contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        initial_whitelist: Option<Vec<Requester>>,
        config: oracle_config::OracleConfig,
    ) -> Self {
        let mut configs = Vector::new(b"c".to_vec());
        configs.push(&config);
        logger::log_oracle_config(&config, 0);

        Self {
            whitelist: whitelist::Whitelist::new(initial_whitelist),
            configs,
            data_requests: Vector::new(b"dr".to_vec()),
            accounts: LookupMap::new(b"a".to_vec()),
        }
    }
}

impl Contract {
    pub fn assert_gov(&self) {
        // AUDIT: .iter().last() might be slower than .get(len() - 1)
        let config = self.configs.iter().last().unwrap();
        assert_eq!(
            config.gov,
            env::predecessor_account_id(),
            "This method is only callable by the governance contract {}",
            config.gov
        );
    }
}
