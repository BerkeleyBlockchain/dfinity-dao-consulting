mod voting;
mod staking;

// use chrono::prelude::*;
use std::collections::HashMap;
use candid::candid_method;
use ic_kit::{ic, Principal};
use ic_kit::macros::*;
use std::collections::LinkedList;

// Submits a grant application for a user
#[update]
#[candid_method(update)]
pub fn submitApp(
    application: String, 
    grantSize: u64
) {
    // TODO: make a transfer to DFINITY for applciation fee

    // add caller to hashmap
    voting::addApplicant(ic::caller(), grantSize);
}

// not sure what the "fee" argument is in the stake function
// make sure quadratic staking is working
// join as a voter + stake
#[candid_method(update)]
pub fn joinVoter(
    amount: u64,
    locktime: u64,
) {
    let caller = ic::caller();
    let now = ic::time();

    // put 0 for fee
    staking::stake(caller, amount, 0, locktime, now);
}

// create vote tokens, everytime local network is spun up
// TODO: idk how to make this run when spun up.
// async fn createVoteTokens() {
//     // call to the token canister
//     // Principal::from_str("0x9762D80271de8fa872A2a1f770E2319c3DF643bC").unwrap()
//     ic_cdk::api::call::call(
//         TOKEN_CANISTER,
//         "init",
//         ("logo", "name", "symbol", 18, 10, "czno4-rk7jd-ohw6i-iub4f-atz6u-nkz7y-2bzw3-lutwk-ojg6j-axjew-lae", 1, "fee_to")
//     ).await;
//     //ic_cdk::api::call(candid::Principal::management_canister(), "create_canister", ()).await?;
// }

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

const VOTE_EXPONENT: u32 = 2;
// Prints out something idk what this does.
#[query]
fn getNumVotes(icpAdded: u32, currVotes: u32) {
    //new votes: currvotes ^ vote_exp + icpAdded
    let num_icp = currVotes.pow(VOTE_EXPONENT) + icpAdded;
    let conv_ratio = 1 / VOTE_EXPONENT;
    let num_votes = (num_icp as f64).powf(conv_ratio as f64);
    ic_cdk::print(num_votes.to_string());
}

// cast first vote
#[update]
fn castFirstVote(
    votes: HashMap<Principal, bool>,
) {
    voting::firstVote(ic::caller(), votes);
}

// cast second vote
// might need to edit this for multiple grant sizes 
#[update]
fn castSecondVote(
    votes: LinkedList<Principal>
) {
    voting::secondVote(ic::caller(), votes);
}

// delegate voting tokens to others
// setting grant sizes
fn grantSizes(
    sizes:LinkedList<u64>
) {
    voting::setGrantSizes(sizes);
}
// get voters (from staking.rs)

// TODO in main
// create internet identity
// start voting period - probably want a portal for dfinity


#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}