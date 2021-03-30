use crate::*;
use near_sdk::serde::{ Serialize, Deserialize };

const MAX_SOURCES: u8 = 8;

#[derive(Serialize, Deserialize)]
pub struct NewDataRequestArgs {
    pub sources: Vec<data_request::Source>,
    pub outcomes: Option<Vec<String>>,
    pub settlement_time: Timestamp, // Can be in the past
    pub challenge_period: Timestamp,
    pub target_contract: AccountId
}

impl Contract {
    pub fn dr_validate(&self, data_request: &NewDataRequestArgs) {
        assert_eq!(data_request.sources.len() as u8, MAX_SOURCES, "Source vector length exceeds max");
        assert!(data_request.challenge_period >= self.config.min_initial_challenge_window_duration, "Challenge period exceeds maximum challenge period");
        assert!(data_request.challenge_period <= self.config.default_challenge_window_duration * 3, "Challenge shorter than minimum challenge period");
        assert!(
            data_request.outcomes.is_none() || 
            data_request.outcomes.as_ref().unwrap().len() as u8 <= self.config.max_outcomes,
            "Source vector length exceeds max"
        );
    }
}

#[derive(Serialize, Deserialize)]
pub struct StakeDataRequestArgs {
    pub id: U64,
    pub outcome: data_request::Outcome,
}

#[derive(Serialize, Deserialize)]
pub struct ChallengeDataRequestArgs {
    pub id: U64,
    pub answer: data_request::Outcome,
}