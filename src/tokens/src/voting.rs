use std::collections::{HashMap, BTreeMap, LinkedList};
use std::vec::{Vec};
use std::str::FromStr;
use ic_cdk_macros::import;
use ic_kit::{ic, Principal};
use candid::{CandidType, Deserialize};

type Results1 = BTreeMap<Principal, VoteStatus>;
type Results2 = BTreeMap<Principal, u64>;
type Winners1 = LinkedList<Principal>;
type GrantSizes = LinkedList<u64>;
type VotingPeriods = LinkedList<Vec<u64>>;

type Vote2 = BTreeMap<Principal, LinkedList<Principal>>;

// lists applicant and the grant size they are applying for
pub type Applications = BTreeMap<Principal, Application>;

#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct Proposal {
    name: String,
    description: String,
    grant_size: u64
}

#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct Application {
    pub proposal: Proposal,
    pub create_timestamp: u64,
    pub principal: Principal,
}

#[derive(Clone, Copy)]
struct VoteStatus {
    yes: u64,
    no: u64,
}

pub fn get_applications() -> &'static Applications {
    // TODO: add pagination or something.
    ic::get_mut::<Applications>()
}

pub fn add_application(
    caller: Principal,
    application: Application
) {
    let applicants = ic::get_mut::<Applications>();
    applicants.insert(caller, application);
}

pub fn setGrantSizes(
    sizes: LinkedList<u64>
) {
    let grant_sizes = ic::get_mut::<GrantSizes>();
    for value in sizes.iter_mut() {
        grant_sizes.push_back(*value);
    }
}

fn secondVoteScores() {
    let vote2 = ic::get::<Vote2>();
    let results2 = ic::get_mut::<Results2>();
    let count : u64 = 0;
    for (key, value) in vote2.into_iter() {
        count = value.len() as u64; // depends on how many they ranked
       for applicant in value.into_iter() {
            if let Some(x) = results2.get_mut(applicant) {
                *x += count;
            }
            count = count - 1;
       }
    }
}

pub async fn firstVote(
    from: Principal,
    application: Principal,
    decision: bool,
    timestamp: i64
) {
    // check if in the right voting period
    let from : Principal = ic::caller();
    let voting_periods = ic::get::<VotingPeriods>();
    // TODO: deal with timestamps
    let current_period = &voting_periods.into_iter();
    // if (timestamp < current_period[0]) && (timestamp > current_period[1]) {
    //     return Err("Not correct voting period".to_string());
    // }
    let results1 = ic::get_mut::<Results1>();
    if results1.contains_key(&application) {
        let applicationVotes = results1.entry(application).or_insert(VoteStatus { yes: 0, no: 0 });
        if decision {
            applicationVotes.yes += 1;
        } else {
            applicationVotes.no += 1;
        }
    }

    // for (application, didVoteFor) in metadata.into_iter() {
    //     // TODO: ensure an application for 'application' does exist.
    //     let applicationVotes = results1.entry(application).or_insert(VoteStatus { yes: 0, no: 0 });
    //     if didVoteFor {
    //         applicationVotes.yes += 1;
    //     } else {
    //         applicationVotes.no += 1;
    //     }
    // }
}

pub async fn secondVoteAdd(
    from: Principal,
    applicant: Principal
) -> Result<(), String> {
    let vote2 = ic::get_mut::<Vote2>();
    let MINTING_CANISTER: Principal = Principal::from_str("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let BURN_ID: Principal = Principal::from_str("0x9762D80271de8fa872A2a1f770E2319c3DF643bC").unwrap();
    
    if let Some(x) = vote2.get_mut(&from) {
        x.push_back(applicant);
    }

    // TODO: proper error handling
    let (balance,): (u64,) = ic::call(MINTING_CANISTER, "balanceOf", (from,))
        .await
        .map_err(|(code, msg)| format!("Call failed with code={}: {}", code as u8, msg))?;
    if balance > 0 {
        ic::call(MINTING_CANISTER, "transfer", (from, BURN_ID, 1, ""))
        .await
        .map_err(|(code, msg)| format!("Call failed with code={}: {}", code as u8, msg))?;
        Ok(())
    } else {
        println!("You do not have enough votes");
        Ok(())
    }
}

fn firstRoundWinners(
    voter_count: u64
) {
    let results1 = ic::get_mut::<Results1>();
    let winners1 = ic::get_mut::<Winners1>();
    for (&key, value) in results1.into_iter(){
        let curr_voter_count = value.yes + value.no;
        if curr_voter_count as f64 >= (voter_count as f64) * 0.60 {
            if value.yes as f64 > curr_voter_count as f64 * 0.35 {
                winners1.push_back(key);
            }
        }
    }
}

pub fn addVotingPeriod (
    start: u64,
    end: u64
) {
    // add start and end to list of tuples
    let voting_periods = ic::get_mut::<VotingPeriods>();
    let mut vec = Vec::new();
    vec.push(start);
    vec.push(end);
    voting_periods.push_back(vec);
}