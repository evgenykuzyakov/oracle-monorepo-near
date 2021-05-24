use uint::construct_uint;
use near_sdk::{
    env,
    StorageUsage,
    AccountId, 
    Balance,
    Promise,
    PromiseResult
};

const STORAGE_PRICE_PER_BYTE: Balance = 100_000_000_000_000_000_000;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct u256(4);
}

/*** operators that does not take decimals into account ***/
pub fn calc_product(a: u128, b: u128, divisor: u128) -> u128 {
    let a_u256 = u256::from(a);
    let b_u256 = u256::from(b);
    let divisor_u256 = u256::from(divisor);

    (a_u256 * b_u256 / divisor_u256).as_u128()
}

pub fn refund_storage(initial_storage: StorageUsage, sender_id: AccountId) {
    let current_storage = env::storage_usage();
    let attached_deposit = env::attached_deposit();
    let refund_amount = if current_storage > initial_storage {
        let required_deposit =
            Balance::from(current_storage - initial_storage) * STORAGE_PRICE_PER_BYTE;
        assert!(
            required_deposit <= attached_deposit,
            "The required attached deposit is {}, but the given attached deposit is is {}",
            required_deposit,
            attached_deposit,
        );
        attached_deposit - required_deposit
    } else {
        attached_deposit
            + Balance::from(initial_storage - current_storage) * STORAGE_PRICE_PER_BYTE
    };
    if refund_amount > 0 {
        Promise::new(sender_id).transfer(refund_amount);
    }
}

pub fn ns_to_ms(ns_timestamp: u64) -> u64 {
    ns_timestamp / 1_000_000
}

/**
 * @panics if the caller is not the contract itself (for promises)
 */
pub(crate) fn assert_self() {
    assert_eq!(
        env::predecessor_account_id(),
        env::current_account_id(),
        "Method is private"
    );
}

pub(crate) fn is_promise_success() -> bool {
    assert_eq!(
        env::promise_results_count(),
        1,
        "Contract expected a result on the callback"
    );
    match env::promise_result(0) {
        PromiseResult::Successful(_) => true,
        _ => false,
    }
}

pub(crate) fn assert_prev_promise_successful() {
    assert_eq!(is_promise_success(), true, "previous promise failed");
}
