use crate::utils::*;
use oracle::whitelist::CustomFeeStakeArgs;
use near_sdk::json_types::U128;
use oracle::data_request::PERCENTAGE_DIVISOR;

#[test]
fn dr_resolution_flow_test() {
    let stake_amount = to_yocto("250"); 
    let stake_cost = 200;
    let dr_cost = 100;
    let init_res = TestUtils::init(None);
    let init_balance_alice = init_res.alice.get_token_balance(None);

    let _res = init_res.alice.dr_new();
    let _post_new_balance_oracle = init_res.alice.get_token_balance(Some(ORACLE_CONTRACT_ID.to_string()));
    
    let dr_exist = init_res.alice.dr_exists(0);
    assert!(dr_exist, "something went wrong during dr creation");
    let outcome = data_request::Outcome::Answer(
        data_request::AnswerType::String("test".to_string())
    );
    let _res = init_res.alice.stake(0, outcome, stake_amount);

    let _post_stake_balance_oracle = init_res.alice.get_token_balance(Some(ORACLE_CONTRACT_ID.to_string()));
    let post_stake_balance_alice = init_res.alice.get_token_balance(None);
    assert_eq!(post_stake_balance_alice, init_balance_alice - stake_cost - dr_cost);
    
    init_res.bob.ft_transfer(&TARGET_CONTRACT_ID, 100_000);
    init_res.alice.finalize(0);
    init_res.alice.claim(0);
    
    let post_claim_balance_alice = init_res.alice.get_token_balance(None);
    assert_eq!(post_claim_balance_alice, init_balance_alice);
}

#[test]
fn dr_fixed_fee_flow() {
    let custom_fee_amount = 100;
    let custom_fee = CustomFeeStakeArgs::Fixed(U128(custom_fee_amount));
    let stake_amount = to_yocto("250");
    let dr_cost = 100;
    let init_res = TestUtils::init(Some(custom_fee));
    let init_balance_alice = init_res.alice.get_token_balance(None);

    let _res = init_res.alice.dr_new();
    let _post_new_balance_oracle = init_res.alice.get_token_balance(Some(ORACLE_CONTRACT_ID.to_string()));
    
    let dr_exist = init_res.alice.dr_exists(0);
    assert!(dr_exist, "something went wrong during dr creation");
    let outcome = data_request::Outcome::Answer(
        data_request::AnswerType::String("test".to_string())
    );
    let _res = init_res.alice.stake(0, outcome, stake_amount);

    let _post_stake_balance_oracle = init_res.alice.get_token_balance(Some(ORACLE_CONTRACT_ID.to_string()));
    let post_stake_balance_alice = init_res.alice.get_token_balance(None);
    assert_eq!(post_stake_balance_alice, init_balance_alice - dr_cost - custom_fee_amount*2);
    
    init_res.bob.ft_transfer(&TARGET_CONTRACT_ID, 100_000);

    init_res.alice.finalize(0);
    init_res.alice.claim(0);
    
    // let post_claim_balance_alice = init_res.alice.get_token_balance(None);
    // assert_eq!(post_claim_balance_alice, init_balance_alice);
}

#[test]
fn dr_multiplier_flow() {
    let stake_cost = 200;
    let multiplier_amount = 10500; // 105%
    let custom_fee = CustomFeeStakeArgs::Multiplier(multiplier_amount);
    let stake_amount = to_yocto("250");
    let dr_cost = 100;
    let init_res = TestUtils::init(Some(custom_fee));
    let init_balance_alice = init_res.alice.get_token_balance(None);

    let _res = init_res.alice.dr_new();
    let _post_new_balance_oracle = init_res.alice.get_token_balance(Some(ORACLE_CONTRACT_ID.to_string()));
    
    let dr_exist = init_res.alice.dr_exists(0);
    assert!(dr_exist, "something went wrong during dr creation");
    let outcome = data_request::Outcome::Answer(
        data_request::AnswerType::String("test".to_string())
    );
    let _res = init_res.alice.stake(0, outcome, stake_amount);

    let _post_stake_balance_oracle = init_res.alice.get_token_balance(Some(ORACLE_CONTRACT_ID.to_string()));
    let post_stake_balance_alice = init_res.alice.get_token_balance(None);
    let weighted_stake_cost = u128::from(stake_cost * u64::from(multiplier_amount) / PERCENTAGE_DIVISOR as u64);
    assert_eq!(post_stake_balance_alice, init_balance_alice - dr_cost - weighted_stake_cost);
    
    init_res.bob.ft_transfer(&TARGET_CONTRACT_ID, 100_000);

    init_res.alice.finalize(0);
    init_res.alice.claim(0);
    
    let post_claim_balance_alice = init_res.alice.get_token_balance(None);
    assert_eq!(post_claim_balance_alice, init_balance_alice);
}
