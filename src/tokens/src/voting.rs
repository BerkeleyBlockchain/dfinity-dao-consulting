use std::collections::{HashMap, BTreeMap, LinkedList};
use std::str::FromStr;
use ic_cdk_macros::import;
use ic_kit::{ic, Principal};
use candid::{CandidType, Deserialize};

type Results1 = BTreeMap<Principal, VoteStatus>;
type Results2 = BTreeMap<Principal, u64>;
type Winners1 = LinkedList<Principal>;
type GrantSizes = LinkedList<u64>;

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
    for value in sizes {
        grant_sizes.push_back(value);
    }
}

// fn secondVoteScores() {
//     let vote2 = ic::get::<Vote2>();
//     let count = 0;
//     for (key, value) in vote2.into_iter() {
//         let iter_ranked = value.iter();
//         count = value.len(); // depends on how many they ranked
//         let results2 = ic::get_mut::<Results2>();
//         while iter_ranked != None {
//             match results2.get(iter_ranked) {
//                 Some(votes) => votes + count,
//                 None => 0,
//             }
//             iter_ranked = iter_ranked.next();
//             count = count - 1;
//         }
//     }
    
// }

// list of applications they've looked at 
pub async fn firstVote(
    from: Principal,
    metadata: HashMap<Principal, bool>
) -> Result<(), String> {
    let MINTING_CANISTER: Principal = Principal::from_str("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let BURN_ID: Principal = Principal::from_str("0x9762D80271de8fa872A2a1f770E2319c3DF643bC").unwrap();

    // TODO: proper error handling
    let (balance,): (u64,) = ic::call(MINTING_CANISTER, "balanceOf", (from,))
        .await
        .map_err(|(code, msg)| format!("Call failed with code={}: {}", code as u8, msg))?;

    // use dfinity's account as grantee
    ic::call(MINTING_CANISTER, "transfer", (from, BURN_ID, balance, ""))
        .await
        .map_err(|(code, msg)| format!("Call failed with code={}: {}", code as u8, msg))?;

    let results1 = ic::get_mut::<Results1>();
    for (application, didVoteFor) in metadata.into_iter() {
        // TODO: ensure an application for 'application' does exist.
        let applicationVotes = results1.entry(application).or_insert(VoteStatus { yes: 0, no: 0 });
        if didVoteFor {
            applicationVotes.yes += 1;
        } else {
            applicationVotes.no += 1;
        }
    }
    Ok(())
}

pub fn secondVote(
    from: Principal,
    metadata: LinkedList<Principal>
) {
    let vote2 = ic::get_mut::<Vote2>();
    // match vote2.get(&from) {
    //     Some(ranks) => { metadata; }
    //     None => None;
    // }
    // what to do with token?
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

// second vote winners function
// fn secondVoteWinners()

// time lock
// have a way of looking at how many voters there are 
// have a way at looking at grant sizes
// need to rank second vote according to grant sizes