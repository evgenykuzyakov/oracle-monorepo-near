use near_sdk::{
    env,
    json_types::{U128, U64},
    serde_json::json,
    AccountId, Balance,
};

use crate::{
    data_request::DataRequest, helpers::ns_to_ms, oracle_config::OracleConfig,
    requester_handler::Requester, resolution_window::ResolutionWindow, types::*,
};

pub fn log_new_data_request(request: &DataRequest) {
    env::log(
        json!({
            "type": "data_requests",
            "action": "update",
            "cap_id": format!("dr_{}", request.id),
            "params": {
                "id": U64(request.id),
                "sources": request.sources,
                "description": request.description,
                "outcomes": request.outcomes,
                "requester": request.requester,
                "requester_account_id": request.requester.account_id,
                "finalized_outcome": request.finalized_outcome,
                "initial_challenge_period": U64(request.initial_challenge_period),
                "final_arbitrator_triggered": request.final_arbitrator_triggered,
                "paid_fee": U128(request.request_config.paid_fee),
                "stake_multiplier": request.request_config.stake_multiplier,
                "global_config_id": U64(request.global_config_id),
                "tags": request.tags,
                "date": U64(ns_to_ms(env::block_timestamp())),
                "block_height": U64(env::block_index()),
                "data_type": request.data_type,
                "creator": request.creator,
            }
        })
        .to_string()
        .as_bytes(),
    );
}

pub fn log_update_data_request(request: &DataRequest) {
    env::log(
        json!({
            "type": "data_requests",
            "action": "update",
            "cap_id": format!("dr_{}", request.id),
            "params": {
                "id": U64(request.id),
                "sources": request.sources,
                "outcomes": request.outcomes,
                "requester": request.requester,
                "finalized_outcome": request.finalized_outcome,
                "initial_challenge_period": U64(request.initial_challenge_period),
                "final_arbitrator_triggered": request.final_arbitrator_triggered,
            }
        })
        .to_string()
        .as_bytes(),
    );
}

pub fn log_oracle_config(config: &OracleConfig, id: u64) {
    env::log(
        json!({
            "type": "oracle_configs",
            "action": "update",
            "cap_id": format!("oc_{}", id),
            "params": {
                "id": U64(id),
                "gov": config.gov,
                "final_arbitrator": config.final_arbitrator,
                "stake_token": config.stake_token,
                "payment_token": config.payment_token,
                "validity_bond": config.validity_bond,
                "max_outcomes": config.max_outcomes,
                "default_challenge_window_duration": config.default_challenge_window_duration,
                "min_initial_challenge_window_duration": config.min_initial_challenge_window_duration,
                "final_arbitrator_invoke_amount": config.final_arbitrator_invoke_amount,

                "fee": {
                    "flux_market_cap": config.fee.flux_market_cap,
                    "total_value_staked": config.fee.total_value_staked,
                    "resolution_fee_percentage": config.fee.resolution_fee_percentage,
                },

                "date": U64(ns_to_ms(env::block_timestamp())),
                "block_height": U64(env::block_index()),
            }
        })
        .to_string()
        .as_bytes()
    );
}

pub fn log_resolution_window(window: &ResolutionWindow) {
    env::log(
        json!({
            "type": "resolution_windows",
            "action": "update",
            "cap_id": format!("rw_{}_{}", window.dr_id, window.round),
            "params": {
                "id": format!("rw_{}_{}", window.dr_id, window.round),
                "dr_id": U64(window.dr_id),
                "round": window.round,
                "start_time": U64(window.start_time),
                "end_time": U64(window.end_time),
                "bond_size": U128(window.bond_size),
                "bonded_outcome": window.bonded_outcome,

                "date": U64(ns_to_ms(env::block_timestamp())),
                "block_height": U64(env::block_index()),
            }
        })
        .to_string()
        .as_bytes(),
    );
}

fn outcome_to_id(outcome: &Outcome) -> String {
    // We append ans_ infront of an answer to avoid malicous fake invalids
    // that would overwrite a real invalid outcome
    match outcome {
        Outcome::Answer(answer) => match answer {
            AnswerType::String(str_ans) => format!("ans_str_{}", str_ans),
            AnswerType::Number(num_ans) => format!(
                "ans_num_{}_{}_{}",
                num_ans.value.0, num_ans.multiplier.0, num_ans.negative
            ),
        },
        Outcome::Invalid => "invalid".to_string(),
    }
}

pub fn log_outcome_to_stake(
    data_request_id: u64,
    round: u16,
    outcome: &Outcome,
    total_stake: Balance,
) {
    let outcome_id = outcome_to_id(outcome);

    env::log(
        json!({
            "type": "outcome_stakes",
            "action": "update",
            "cap_id": format!("ots_{}_{}_{}", data_request_id, round, outcome_id),
            "params": {
                "id": format!("ots_{}_{}_{}", data_request_id, round, outcome_id),
                "data_request_id": U64(data_request_id),
                "round": round,
                "outcome": outcome,
                "total_stake": U128(total_stake),
            }
        })
        .to_string()
        .as_bytes(),
    );
}

pub fn log_user_stake(
    data_request_id: u64,
    round: u16,
    account_id: &AccountId,
    outcome: &Outcome,
    total_stake: Balance,
) {
    let outcome_id = outcome_to_id(outcome);

    env::log(
        json!({
            "type": "user_stakes",
            "action": "update",
            "cap_id": format!("us_{}_{}_{}_{}", data_request_id, round, outcome_id, account_id),
            "params": {
                "id": format!("us_{}_{}_{}_{}", data_request_id, round, outcome_id, account_id),
                "data_request_id": U64(data_request_id),
                "round": round,
                "outcome": outcome,
                "account_id": account_id,
                "total_stake": U128(total_stake),
            }
        })
        .to_string()
        .as_bytes(),
    );
}

pub fn log_claim(
    account_id: &AccountId,
    data_request_id: u64,
    total_correct_bonded_staked: u128,
    total_incorrect_staked: u128,
    user_correct_stake: u128,
    stake_profit: u128,
    fee_profit: u128,
) {
    env::log(
        json!({
            "type": "data_requests",
            "action": "update",
            "cap_id": format!("dr_{}", data_request_id),
            "params": {
                "id": U64(data_request_id),
                "total_correct_bonded_staked": U128(total_correct_bonded_staked),
                "total_incorrect_staked": U128(total_incorrect_staked),
            }
        })
        .to_string()
        .as_bytes(),
    );

    env::log(
        json!({
            "type": "claims",
            "action": "update",
            "cap_id": format!("c_{}_{}", account_id, data_request_id),
            "params": {
                "id": format!("c_{}_{}", account_id, data_request_id),
                "account_id": account_id,
                "data_request_id": U64(data_request_id),
                "total_correct_bonded_staked": U128(total_correct_bonded_staked),
                "total_incorrect_staked": U128(total_incorrect_staked),
                "user_correct_stake": U128(user_correct_stake),
                "payout": U128(stake_profit),
                "fee_profit": U128(fee_profit),
                "date": U64(ns_to_ms(env::block_timestamp())),
                "block_height": U64(env::block_index()),
            }
        })
        .to_string()
        .as_bytes(),
    );
}

pub fn log_whitelist(requester: &Requester, active: bool) {
    env::log(
        json!({
            "type": "whitelist",
            "action": "update",
            "cap_id": format!("wl_{}", requester.account_id),
            "params": {
                "id": format!("wl_{}", requester.account_id),
                "contract_name": requester.contract_name,
                "account_id": requester.account_id,
                "stake_multiplier": requester.stake_multiplier,
                "code_base_url": requester.code_base_url,
                "active": active,
                "date": U64(ns_to_ms(env::block_timestamp())),
                "block_height": U64(env::block_index()),
            }
        })
        .to_string()
        .as_bytes(),
    );
}

#[derive(serde::Serialize)]
pub enum TransactionType {
    Stake,
    Unstake,
}

pub fn log_transaction(
    tx_type: TransactionType,
    account_id: &AccountId,
    data_request_id: u64,
    round: Option<u16>,
    input: u128,
    output: u128,
    extra_info: Option<String>,
) {
    env::log(
        json!({
            "type": "transactions",
            "params": {
                "account_id": account_id,
                "input": U128(input),
                "output": U128(output),
                "data_request_id": U64(data_request_id),
                "round": round,
                "date": U64(ns_to_ms(env::block_timestamp())),
                "block_height": U64(env::block_index()),
                "extra_info": extra_info,
                "type": tx_type,
            }
        })
        .to_string()
        .as_bytes(),
    );
}

pub fn log_stake_transaction(
    account_id: &AccountId,
    window: &ResolutionWindow,
    amount_in: Balance,
    amount_out: Balance,
    outcome: &Outcome,
) {
    log_transaction(
        TransactionType::Stake,
        account_id,
        window.dr_id,
        Some(window.round),
        amount_in,
        amount_out,
        Some(outcome_to_id(outcome)),
    );
}

pub fn log_unstake_transaction(
    account_id: &AccountId,
    window: &ResolutionWindow,
    amount_out: Balance,
    outcome: &Outcome,
) {
    log_transaction(
        TransactionType::Unstake,
        account_id,
        window.dr_id,
        Some(window.round),
        0,
        amount_out,
        Some(outcome_to_id(outcome)),
    );
}
