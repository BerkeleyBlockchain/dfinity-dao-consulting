// account balance
// total_supply()
// send
// main

use candid::{candid_method, CandidType, Deserialize};
use ic_kit::{ic , Principal};
use ic_cdk_macros::*;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::string::String;

// global variables
// The derive attribute allows new items to be automatically generated for data structures.
#[derive(Deserialize, CandidType, Clone, Debug)]
struct Metadata {
    logo: String,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u64,
    owner: Principal,
    fee: u64,
    fee_to: Principal,
}

#[derive(Deserialize, CandidType, Clone, Debug)]
struct TokenInfo {
    metadata: Metadata,
    fee_to: Principal,
    // status info
    history_size: usize,
    deploy_time: u64,
    holder_number: usize,
    cycles: u64,
}

// initializes an erc20 token
#[init]
#[candid_method(init)]
// TODO: not sure if we need to add the cdk stuff
fn init(
    logo: String,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u64,
    owner: Principal,
    fee: u64,
) {
    let metadata = ic::get_mut::<Metadata>();
    metadata.logo = logo;
    metadata.name = name;
    metadata.symbol = symbol;
    metadata.decimals = decimals;
    metadata.total_supply = total_supply;
    metadata.owner = owner;
    metadata.fee = fee;
    ic_cdk::print(metadata);
    // let balances = ic::get_mut::<Balances>();
    // balances.insert(owner, total_supply);
    // let _ = add_record(
    //     Some(owner),
    //     Operation::Mint,
    //     Principal::from_text("aaaaa-aa").unwrap(),
    //     owner,
    //     total_supply,
    //     0,
    //     ic::time(),
    // );
}