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
    for (key, value) in vote2.into_iter() {
        let iter_ranked = value.iter();
        count = value.len(); // depends on how many they ranked
        let results2 = ic::get_mut::<Results2>();
        while (iter_ranked != None) {
            match results.get(iter_ranked) {
                Some(votes) => votes + count,
                None => 0,
            }
            iter_ranked = iter_ranked.next();
            count --;
        }
    }
    
}

fn firstVote(
    from: Option<Principal>,
    metadata: HashMap<Principal, bool>,
) {
    u64 balance = main::balance_of(caller);
    main::transfer(grantee, BURN_ID, balance, "");
}

fn secondVote(
    from: Option<Principal>,
    metadata: HashMap<Principal, LinkedList<Principal>>,
) {
    // Consider our ranked voting implementation!
}

// time lock
// deciding on winner