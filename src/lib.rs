//use std::convert::TryInto;
//use std::ops::Index;

use near_sdk::collections::{
    LookupMap,
    //Vector,
};

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    //log,
    //serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault
    //, Promise,
};
//use near_sdk::env::{self, signer_account_id};
use near_sdk::{near_bindgen};
//use near_sdk::{near_bindgen, serde};
use String;

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Tournament {
    // SETUP CONTRACT STATE
    player_stats: LookupMap<AccountId, u8>,
    comp: bool,
    round: u8,

}
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Stats{
    wins: u128,
    losses: u128,
    championships: u128,

}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // SETUP CONTRACT STATE
    tournament_keys: LookupMap<AccountId, String>,
    tournaments: LookupMap<String, Tournament>,
    user_stats: LookupMap<AccountId, Stats>

}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
