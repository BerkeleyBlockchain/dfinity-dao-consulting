/*
file consisting of functions that the frontend will interface with
*/
// mod token;
mod voting;
mod staking;

// use chrono::prelude::*;
use std::collections::HashMap;
use ic_kit::{ic , Principal};
use std::collections::LinkedList;
use ic_cdk::api::*;

// not sure what the "fee" argument is in the stake function
// make sure quadratic staking is working
// join as a voter + stake
fn joinVoter (
    caller: Option<Principal>,
    amount: u64,
    locktime: u64,
) {
    // let now = Utc::now();
    // now = now.timestamp_millis();
    // put 0 for fee
    staking::stake(caller, amount, 0, locktime, 0);
}

// create vote tokens, everytime local network is spun up
fn createVoteTokens() {
    // call to the token canister
    ic_cdk::api::call::call("rrkah-fqaaa-aaaaa-aaaaq-cai", "init", ("logo", "name", "symbol", 18, 10, "czno4-rk7jd-ohw6i-iub4f-atz6u-nkz7y-2bzw3-lutwk-ojg6j-axjew-lae", 1, "fee_to")).await?;
    //ic_cdk::api::call(candid::Principal::management_canister(), "create_canister", ()).await?;
}

// mint voting tokens
// not included in candid file because this should happen after staking (and be in staking file)
// https://forum.dfinity.org/t/rust-create-canister-inter-canister-calls/2016
// https://github.com/dfinity/cdk-rs/blob/ee145313fa9feae4ae0ab7602d8fca39374c3fb8/src/ic-cdk/src/api/call.rs#L140
// https://docs.rs/ic-agent/latest/ic_agent/
// fn mintVoteTokens(
//     amount: u64,

// ) {

//     let (first_result, second_result) : (first_result_type, second_result_type) = ic_cdk::api::call::call("rrkah-fqaaa-aaaaa-aaaaq-cai", "method", (first_arg, second_arg)).await?;

// }

// cast first vote
fn castFirstVote(
    caller: Option<Principal>,
    votes: HashMap<Principal, bool>,
) {
    voting::firstVote(caller, votes);
}

// cast second vote
// might need to edit this for multiple grant sizes 
fn castSecondVote(
    caller: Option<Principal>,
    votes: LinkedList<Principal>
) {
    voting::secondVote(caller, votes);
}

// submit application
fn submitApp(
    caller: Option<Principal>,
    application: String, 
    grantSize: u64
) {
    // make a transfer to DFINITY for applciation fee

    // add caller to hashmap
    voting::addApplicant(caller, grantSize);

}

// delegate voting tokens to others
// setting grant sizes
fn grantSizes(
    sizes:LinkedList<u64>
) {
    voting::setGrantSizes(sizes);
}
// get voters (from staking.rs)

// might add this do main.rs here, or to the rust_starter canister?
// create internet identity
// start voting period - probably want a portal for dfinity