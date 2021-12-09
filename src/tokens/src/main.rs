/*
file consisting of functions that the frontend will interface with
*/
mod token;
mod voting;
mod staking;

use chrono::prelude::*;
use std::collections::HashMap;
use ic_kit::{ic , Principal};
use std::collections::LinkedList;

// not sure what the "fee" argument is in the stake function
// make sure quadratic staking is working
// join as a voter + stake
fn joinVoter (
    caller: Option<Principal>,
    amount: u64,
    locktime u64,
) {
    let now = Utc::now();
    now = now.timestamp_millis();
    // put 0 for fee
    staking::stake(caller, amount, 0, locktime, now);
}

// create vote tokens, everytime local network is spun up
fn createVoteTokens() {
    // call to the token canister
    api::call::call("rrkah-fqaaa-aaaaa-aaaaq-cai", "setMetadata", (first_arg, second_arg)).await?;
}

// mint voting tokens
// not included in candid file because this should happen after staking (and be in staking file)
fn mintVoteTokens(
    amount: u64,

) {

    let (first_result, second_result) : (first_result_type, second_result_type) = 
     api::call::call(canister_id, "method", (first_arg, second_arg)).await?;

}

// cast first vote
fn castFirstVote(
    caller: Option<Principal>,
    votes: HashMap<Principal, bool>,
) {
    firstVote(caller, votes);
}

// cast second vote
// might need to edit this for multiple grant sizes 
fn castSecondVote(
    caller: Option<Principal>,
    votes: LinkedList<Principal>
) {
    secondVote(caller, vote);
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