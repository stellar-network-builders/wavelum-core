use soroban_sdk::{Address, Env, Vec};

use crate::storage::get_claim_history;
use crate::types::ClaimEvent;

#[allow(dead_code)]
pub fn export_all_claims(e: &Env) -> Vec<ClaimEvent> {
    get_claim_history(e)
}

#[allow(dead_code)]
pub fn export_claims_by_user(e: &Env, user: Address) -> Vec<ClaimEvent> {
    let history = get_claim_history(e);
    let mut filtered = Vec::new(e);

    for event in history.iter() {
        if event.beneficiary == user {
            filtered.push_back(event);
        }
    }

    filtered
}
