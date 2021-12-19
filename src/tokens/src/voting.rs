// mod token;

use std::collections::HashMap;
use std::collections::LinkedList;
use ic_kit::{ic , Principal};

type Results1 = HashMap<Principal, Vote1Status>;
type Results2 = HashMap<Principal, u64>;

type Winners1 = LinkedList<Principal>;

static BURN_ID: Principal = "0x9762D80271de8fa872A2a1f770E2319c3DF643bC";
static GRANTEE: Principal = "czno4-rk7jd-ohw6i-iub4f-atz6u-nkz7y-2bzw3-lutwk-ojg6j-axjew-lae";

type GrantSizes = LinkedList<u64>;

// lists applicant and the grant size they are applying for
type Applicants = LinkedList<Principal, u64>;

#[derive(Clone, Copy)]
struct Vote1Status {
    yes: u64,
    no: u64,
}

type Vote2 = HashMap<Principal, LinkedList<Principal>>;

fn addApplicant(
    caller: Option<Principal>,
    grantSize: u64
) {
    let applicants = ic::get_mut::<Applicants>();
    applicants.push_back(caller, grantSize);
}


fn setGrantSizes(
    sizes: LinkedList<u64>
) {
    let grant_sizes = ic::get_mut::<GrantSizes>();
    grant_sizes = sizes;
}

fn secondVoteScores() {
    let vote2 = ic::get::<Vote2>();
    let count = 0;
    for (key, value) in vote2.into_iter() {
        let iter_ranked = value.iter();
        count = value.len(); // depends on how many they ranked
        let results2 = ic::get_mut::<Results2>();
        while (iter_ranked != None) {
            match results2.get(iter_ranked) {
                Some(votes) => votes + count,
                None => 0,
            }
            iter_ranked = iter_ranked.next();
            count = count - 1;
        }
    }
    
}

// list of applications they've looked at 
fn firstVote(
    from: Option<Principal>,
    metadata: HashMap<Principal, bool>
) {
    let balance: u64 = token::balance_of(from);
    // token::transfer(grantee, BURN_ID, balance, "");
    // use dfinity's account as grantee
    ic_cdk::api::call::call("rrkah-fqaaa-aaaaa-aaaaq-cai", "transfer", (grantee, BURN_ID, balance, "")).await?;
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
    metadata: LinkedList<Principal>
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
    let winners1 = ic::get_mut::<Winners1>();
    for (key, value) in results1.into_iter(){
        curr_voter_count = *value.yes + *value.no;
        if curr_voter_count >= voter_count * 0.60 {
            if *value.yes > curr_voter_count * 0.35 {
                winners1.push_back(key);
            }
        }
    }
}
// second vote winners function
// fn secondVoteWinners()

// time lock
// have a way of looking at how many voters there are 
// have a way at looking at grant sizes
// need to rank second vote according to grant sizes