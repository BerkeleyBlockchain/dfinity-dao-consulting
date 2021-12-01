mod token;

use std::collections::HashMap;
use std::collections::LinkedList;
use ic_kit::{ic , Principal};

type Results1 = HashMap<Principal, Vote1Status>;
type Results2 = HashMap<Principal, u64>;

type Winners1 = LinkedList<Principal>;

static BURN_ID: Principal "0x9762D80271de8fa872A2a1f770E2319c3DF643bC";

type GrantSizes = LinkedList<u64>;

#[derive(Clone, Copy)]
struct Vote1Status {
    yes: u64,
    no: u64,
}

type Vote2 = HashMap<Principal, LinkedList<Principal>>

fn setGrantSizes(
    sizes: LinkedList<u64>
) {
    let grant_sizes = ic::get_mut::<GrantSizes>();
    grant_sizes = sizes;
}

fn secondVoteScores() {
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

// list of applications they've looked at 
fn firstVote(
    from: Option<Principal>,
    metadata: HashMap<Principal, bool>,
) {
    u64 balance = token::balance_of(caller);
    token::transfer(grantee, BURN_ID, balance, "");
    let results1 = ic::get_mut::<Results1>();
    for (key, value) in metadata.into_iter() {
        if value {
            // not sure if this is how you do it
            *results1.get_mut(key).unwrap().yes += 1;
            // match vote1.get(&key) {
            //     Some(status) => status->yes + 1,
            //     None => 0
            // }
        } else {
            *results1.get_mut(key).unwrap().yes += 1;
        }
    }
}

fn secondVote(
    from: Option<Principal>,
    metadata: LinkedList<Principal>,
) {
    let vote2 = ic::get_mut::<Vote2>();
    match vote2.get(&from) {
        Some(ranks) => metadata,
        None => None
    }
    // what to do with token?
}

fn firstRoundWinners(
    voter_count: u64
) {
    let results1 = ic::get_mut::<Results1>();
    let winners1 = ic::get_mut::<Winner1>();
    for (key, value) in results1.into_iter(){
        curr_voter_count = *value.yes + *value.no;
        if curr_voter_count >= voter_count * 0.60 {
            if *value.yes > curr_voter_count * 0.35 {
                winners1.push_back(key);
            }
        }
    }
}

fn secondVoteWinners()

// time lock
// have a way of looking at how many voters there are 
// have a way at looking at grant sizes
// need to rank second vote according to grant sizes