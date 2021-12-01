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
}

// mint voting tokens
// not included in candid file because this should happen after staking (and be in staking file)
fn mintVoteTokens(
    amount: u64,

) {

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

) {

}

// delegate voting tokens to others
// setting grant sizes
// get voters (from staking.rs)

// might add this do main.rs here, or to the rust_starter canister?
// create internet identity
// start voting period - probably want a portal for dfinity