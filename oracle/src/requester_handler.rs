use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ext_contract, Gas, Promise, PromiseOrValue};

const GAS_BASE_SET_OUTCOME: Gas = 250_000_000_000_000;

#[ext_contract]
pub trait RequesterContractExtern {
    fn set_outcome(
        requester: AccountId,
        outcome: Outcome,
        tags: Vec<String>,
        final_arbitrator_triggered: bool,
    );
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct Requester {
    pub contract_name: String,
    pub account_id: AccountId, // Change to account_id
    pub stake_multiplier: Option<u16>,
    pub code_base_url: Option<String>,
}

#[ext_contract(ext_self)]
trait SelfExt {
    fn proceed_dr_new(&mut self, sender: AccountId, amount: Balance, payload: NewDataRequestArgs);
}

impl Requester {
    pub fn new_no_whitelist(account_id: &AccountId) -> Self {
        Self {
            contract_name: "".to_string(),
            account_id: account_id.to_string(),
            stake_multiplier: None,
            code_base_url: None,
        }
    }
    pub fn set_outcome(
        &self,
        outcome: Outcome,
        tags: Vec<String>,
        final_arbitrator_triggered: bool,
    ) -> Promise {
        // AUDIT: Suggestions:
        //     - No need to pass `requester`, since it will be env::current_account_id() for the receiver.
        //     - Maybe get some unique `request_id`, but I guess it can be part of the `tags`.
        //     - `1` yoctoNEAR is not necessary, since this callback can only be received from the oracle and not from the user.
        //     - Gas limit is a bit tight. Ideally there is larger amount of gas that can be configured.
        requester_contract_extern::set_outcome(
            self.account_id.to_string(),
            outcome,
            tags,
            final_arbitrator_triggered,
            // NEAR params
            &self.account_id,
            1,
            GAS_BASE_SET_OUTCOME / 10,
        )
    }
}

#[near_bindgen]
impl Contract {
    /**
     * @notice called in ft_on_transfer to chain together fetching of TVL and data request creation
     */
    #[private]
    pub fn ft_dr_new_callback(
        &mut self,
        sender: AccountId,
        amount: Balance,
        payload: NewDataRequestArgs,
    ) -> PromiseOrValue<WrappedBalance> {
        PromiseOrValue::Value(U128(self.dr_new(sender.clone(), amount.into(), payload)))
    }
}
