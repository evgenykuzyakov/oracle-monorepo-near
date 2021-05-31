use crate::utils::*;
use oracle::oracle_config::OracleConfig;
use oracle::whitelist::RegistryEntry;

pub struct OracleUtils {
    pub contract: ContractAccount<OracleContract>
}

fn new_registry_entry() -> RegistryEntry {
    RegistryEntry {
        callback: "resolve".to_string(),
        code_base_url: None,
        contract_entry: "test.near".to_string(),
        interface_name: "test".to_string(),
    }
}

impl OracleUtils {
    pub fn new(master_account: &TestAccount) -> Self {
        let config = OracleConfig {
            gov: "alice".to_string(),
            final_arbitrator: "alice".to_string(),
            bond_token: TOKEN_CONTRACT_ID.to_string(),
            stake_token: TOKEN_CONTRACT_ID.to_string(),
            validity_bond: U128(100),
            max_outcomes: 8,
            default_challenge_window_duration: U64(1000),
            min_initial_challenge_window_duration: U64(1000),
            final_arbitrator_invoke_amount: U128(250),
            resolution_fee_percentage: 10_000,
        };

        // deploy token
        let contract = deploy!(
            // Contract Proxy
            contract: OracleContract,
            // Contract account id
            contract_id: ORACLE_CONTRACT_ID,
            // Bytes of contract
            bytes: &ORACLE_WASM_BYTES,
            // User deploying the contract,
            signer_account: master_account.account,
            deposit: to_yocto("1000"),
            // init method
            init_method: new(
                Some(vec![
                    new_registry_entry(), 
                    new_registry_entry()
                    // "amm".to_string().try_into().expect("invalid acc id"), 
                    // "alice".to_string().try_into().expect("invalid acc id")
                    ]
                ), 
                config
            )
        );

        // storage_deposit(TOKEN_CONTRACT_ID, &master_account.account, SAFE_STORAGE_AMOUNT, Some(ORACLE_CONTRACT_ID.to_string()));
        // storage_deposit(ORACLE_CONTRACT_ID, &master_account.account, SAFE_STORAGE_AMOUNT, Some(TOKEN_CONTRACT_ID.to_string()));
        // storage_deposit(ORACLE_CONTRACT_ID, &master_account.account, SAFE_STORAGE_AMOUNT, Some(AMM_CONTRACT_ID.to_string()));


        Self {
            contract
        }
    }
}