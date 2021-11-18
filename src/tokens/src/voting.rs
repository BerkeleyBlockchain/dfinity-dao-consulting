mod main

use std::collections::HashMap;
use std::collections::LinkedList;
use ic_kit::{ic , Principal};

type Results1 = HashMap<Principal, Vote1Status>;
type Results2 = HashMap<Principal, u64>;

static BURN_ID: Principal "0x9762D80271de8fa872A2a1f770E2319c3DF643bC";

type Vote1 = HashMap<Principal, bool>;

struct Vote1Status {
    yes: u64,
    no: u64,
}

type Vote2 = HashMap<Principal, LinkedList<Principal>>


fn setRankedScores() {
    let vote2 = ic::get::<Vote2>();
    let iter_vote2 = vote2.iter();
    while (iter_vote2 != None) {
        
        iter_vote2 = iter_vote2.next();
    }
    
}

fn first_vote(
    from: Option<Principal>,
    metadata: HashMap<Principal, bool>,
) {
    u64 balance = main::balance_of(caller);
    main::transfer(grantee, BURN_ID, balance, "");
}

fn second_vote(
    from: Option<Principal>,
    metadata: HashMap<Principal, LinkedList<Principal>>,
) {
    // Consider our ranked voting implementation!
}

