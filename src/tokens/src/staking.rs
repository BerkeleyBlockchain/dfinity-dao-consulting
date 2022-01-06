// mod token;
use std::{collections::HashMap, collections::LinkedList, str::FromStr};
use ic_kit::{ic, Principal};
use sha2::{Sha256};
use serde::{Serialize, Deserialize};
use ic_ledger_types::{AccountBalanceArgs, AccountIdentifier, Subaccount, TransferArgs, Memo, Timestamp, Tokens};
use ic_cdk::api;
use chrono::prelude::*;
//https://github.com/dfinity/examples/tree/master/rust/tokens_transfer
// TODO: update since time is returned in nanoseconds
//assuming time is in seconds

static REWARD_CONST: f64 = 1209600.0;
static APY: f64 = 0.08;
static TIME_STEPS_PER_YEAR: u64 = 31536000;
static FIRST_VOTE_COST: u64 = 10; // in ICP

type Stakers = HashMap<Principal, f64>;
type Unlocked = HashMap<Principal, f64>; // I think this should be an f64

// This only allows 1 txn per second which may not be what we want. 
// May be better to use BTreeSet<Transaction>
type Transactions = HashMap<Principal, HashMap<u64, Transaction>>;

struct Transaction {
    amount: u64,
    time: u64,
    locktime: u64,
    return_amount: f64
}


pub fn get_source_token_principal() -> Principal {
    // TODO: change 
    return Principal::from_str("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Invoice {
    amount: u64,
    encoded: Vec<u8>
}

impl AsRef<[u8]> for Invoice {
    fn as_ref(&self) -> &[u8] {
        &self.encoded
    }
}

// type ICP = record {
//     e8s : nat64;
//   };
// STAKING: https://www.youtube.com/watch?v=Hm-NWwiUQZw&list=PLuhDt1vhGcrez-f3I0_hvbwGZHZzkZ7Ng&index=2&t=1s
// NNS Ledger Canlista: https://k7gat-daaaa-aaaae-qaahq-cai.ic0.app/listing/nns-ledger-10244/ryjl3-tyaaa-aaaaa-aaaba-cai
pub fn get_invoice(
    amount: u64
) -> Invoice {
    let invoice = Invoice {
        amount: amount
    };
    return invoice;
}

pub async fn notify(
    caller: Principal,
    paid: Invoice,
    locktime: u64,
    block: u64
) -> Result<(), String> {
    let LEDGER_CANISTER: Principal = ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
    
    let mut hasher = Sha256::new();
    hasher.update(paid);
    let hash = hasher.finalize();
    let amt = ic::call(LEDGER_CANISTER, "account_balance", AccountIdentifier(hash)).await;
    // let amt = await ic::call(LEDGER_CANISTER, "account_balance", AccountIdentifier::new(api::id(), hash));
    let amt = match amt {
        Ok(amount) => amount,
        Err(error) => {
            return Err("Canister account balance call invalid.".to_string());
        }
    };
    if (amt != paid.amount) {
        return Err("Canister subaccount did not receive invoice amount.".to_string());
    }
    let utc: DateTime<Utc> = Utc::now();
    let seconds: u64 = utc.timestamp().unsigned_abs();
    let base: u64 = 10;
    let nanoseconds: u64 = seconds * base.pow(9);
    let memo = Memo(0);
    let subaccount = Subaccount(hash);
    let timestamp: Timestamp = Timestamp {
        timestamp_nanos: nanoseconds
    };
    let transfer : TransferArgs = TransferArgs {
        memo: memo,
        amount: Tokens::from_e8s(paid.amount),
        fee: ic_ledger_types::DEFAULT_FEE,
        from_subaccount: subaccount,
        to: AccountIdentifier(api::id(), ic_ledger_types::DEFAULT_SUBACCOUNT),
        created_at_time: timestamp
    };
    ic::call(LEDGER_CANISTER, "transfer", transfer); 

    // copied from stake fn below (above is to verify user placed appropriate funds in one-time account)
    let stakers = ic::get_mut::<Stakers>();
    let transactions = ic::get_mut::<Transactions>();
    
    let current_stake = stakers.get(&caller).copied().unwrap_or(0.0);

    stakers.insert(caller, paid.amount + current_stake);

    let tx_map = transactions.entry(caller).or_insert_with(|| HashMap::new());

    let take_tx = Transaction {
        amount: paid.amount,
        time: nanoseconds,
        locktime: locktime,
        return_amount: calculateReturnLocked(caller, nanoseconds, locktime, paid.amount)
    };

    // was tx_list before, but changed since it was giving error
    tx_map.insert(locktime + nanoseconds, take_tx);

    // add transfer function call

    // transfer voting tokens (don't delete) and add proper error handling
    let numVotes : u64 = calculateNumVoteTokens(paid.amount);
    let MINTING_CANISTER: Principal = Principal::from_str("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    ic::call(MINTING_CANISTER, "transfer", (caller, numVotes));
    Ok(())
}

// pub async fn stake(
//     caller: Principal,
//     amount: u64,
//     fee: u64,
//     locktime: u64,
//     timestamp: u64,
// ) {
//     let stakers = ic::get_mut::<Stakers>();
//     let transactions = ic::get_mut::<Transactions>();
    
//     let current_stake = stakers.get(&caller).copied().unwrap_or(0);

//     stakers.insert(caller, amount + current_stake);

//     let tx_map = transactions.entry(caller).or_insert_with(|| HashMap::new());

//     let take_tx = Transaction {
//         amount: amount,
//         time: timestamp,
//         locktime: locktime,
//         return_amount: calculateReturnLocked(caller, fee, timestamp, locktime, amount)
//     };

//     // was tx_list before, but changed since it was giving error
//     tx_map.insert(locktime + timestamp, take_tx);

//     // add transfer function call

//     // transfer voting tokens (don't delete) and add proper error handling
//     let numVotes : u64 = calculateNumVoteTokens(amount);
//     let MINTING_CANISTER: Principal = Principal::from_str("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
//     ic::call(MINTING_CANISTER, "transfer", (caller, numVotes));
//         // .await
//         // .map_err(|(code, msg)| format!("Call failed with code={}: {}", code as u8, msg))?;

// }

// pub fn get_stakers() -> &LinkedList<Principal> {
//     let staker_map = ic::get::<Stakers>();
//     let stakers = LinkedList::new();
//     for (&key, value) in staker_map.into_iter() {
//         stakers.push_back(&key);
//     }
//     &stakers

// }

fn removeUnlocked(
    caller: Principal,
    amount: f64,
    timestamp: u64
) -> bool {
    //transfer out
    unlockFunds(caller, timestamp);
    let amt_avail = getUnlockedAmount(caller, timestamp);
    if amount > amt_avail {
        let unlock_amt = getUnlockedAmount(caller, timestamp);
        let unlocked = ic::get_mut::<Unlocked>();
        unlocked.insert(caller, unlock_amt - amount);
        true
    } else {
        false
    }
}

fn removeUnlockedAll(
    caller: Principal,
    timestamp: u64,
) -> f64 {
    unlockFunds(caller, timestamp);
    let unlock_amt = getUnlockedAmount(caller, timestamp);
    let unlocked = ic::get_mut::<Unlocked>();
    unlocked.insert(caller, 0.0);
    unlock_amt
}

fn unlockFunds(
    caller: Principal,
    timestamp: u64,
) {
    let stakers = ic::get_mut::<Stakers>();
    let transactions = ic::get_mut::<Transactions>();
    let unlocked = ic::get_mut::<Unlocked>();

    // TODO: is this right? Should the unlocked default to zero
    let mut new_unlock = unlocked.get(&caller).copied().unwrap_or(0.0);

    if let Some(tx_map) = transactions.get_mut(&caller) {
        tx_map.retain(|&time, tx| {
            if time > timestamp {
                // FIXME: u64 + f64 is sketchy.
                new_unlock = new_unlock + tx.return_amount;
                false // Remove this transaction
            } else {
                true
            }
        });
    }

    unlocked.insert(caller, new_unlock);
}

fn getUnlockedAmount(
    caller: Principal,
    timestamp: u64,
) -> f64 {
    let unlocked = ic::get_mut::<Unlocked>();
    *unlocked.get(&caller).unwrap()
}


// fn removeStake(
//     caller: Option<Principal>,
//     amount: u64,
//     fee: u64,
//     timestamp: u64,
// ) -> bool {
//     let stakers = ic::get::<Stakers>();
//     let transactions = ic::get_mut::<Transactions>();

//     if stakers.get(caller) < amount {
//         return false;
//     }
//     match stakers.get(&caller) {
//         Some(balance) => balance - amount,
//         None => 0,
//     }
//     let tx_list = transactions.get(caller);
//     while amount > 0 && !tx_list.is_empty()  {
//         let topTx = tx_list.pop_back();
//         let topVal = topTx.amount;
//         if amount == topVal {
//             break;
//         } else if amount > topVal {
//             amount -= topVal;
//         } else {
//             topVal.amount -= amount;
//             tx_list.push_back(topTx);
//         }
//     }
//     true
// }

fn calculateReturnLocked(
    caller: Principal,
    timestamp: u64,
    locktime: u64,
    amount: u64
) -> f64 {
    let num_years = (locktime as f64) / (TIME_STEPS_PER_YEAR as f64);
    num_years * APY * (amount as f64) + (amount as f64)
}

fn calculateNumVoteTokens(
    staked: u64,
) -> u64 {
    // minimum amount is the same as the cost of one vote token
    let cost : u64 = FIRST_VOTE_COST;
    let votes : u64 = 0;
    while staked >= cost {
        staked -= cost;
        votes += 1;
    }
    votes
}

// fn calculateReward(
//     caller: Option<Principal>,
//     fee: u64,
//     timestamp: u64,
// ) -> f64 {
//     let transactions = ic::get_mut::<Transactions>();
//     let tx_list = transactions.get(caller);
//     let reward_amount = 0;
//     while !tx_list.is_empty() {
//         let top_Tx = tx_list.pop_back();
//         let val = top_Tx.amount;
//         let time = top_Tx.time;
//         //todo: Add cliffs
//         let reward_mul = (timestamp - time) / REWARD_CONST;
//         reward_amount += reward_mul * val;
//     }
//     reward_amount
// }

// delegate voting tokens to others
