// mod token;
use std::{collections::HashMap, str::FromStr, convert::TryInto};
use ic_kit::{ic, Principal};
use sha2::{Sha256, Digest};
use serde::{Serialize};
use candid::{CandidType, Deserialize};
use ic_ledger_types::{AccountBalanceArgs, AccountIdentifier, Subaccount, TransferArgs, Memo, Timestamp, Tokens, TransferResult};
use ic_cdk::api;
use chrono::prelude::*;
//https://github.com/dfinity/examples/tree/master/rust/tokens_transfer
// TODO: update since time is returned in nanoseconds
//assuming time is in seconds

static REWARD_CONST: f64 = 1209600.0;
static APY: f64 = 0.08;
static TIME_STEPS_PER_YEAR: u64 = 31536000;
static FIRST_VOTE_COST: u64 = 10; // in ICP

type Stakers = HashMap<Principal, u64>;
type Unlocked = HashMap<Principal, u64>; // I think this should be an f64

// This only allows 1 txn per second which may not be what we want. 
// May be better to use BTreeSet<Transaction>
type Transactions = HashMap<Principal, HashMap<u64, Transaction>>;

struct Transaction {
    amount: u64,
    time: u64,
    locktime: u64,
    return_amount: u64
}


pub fn get_source_token_principal() -> Principal {
    // TODO: change 
    return Principal::from_str("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
}

#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct Invoice {
    amount: u64,
    random: u64
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
        amount: amount,
        random: 2
    };
    return invoice;
}

fn hash_invoice (
    invoice: Invoice
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(invoice.amount.to_be_bytes());
    hasher.update(invoice.random.to_be_bytes());
    let hashvalue = hasher.finalize();
    let x: [u8; 32] = [
        hashvalue[0],
        hashvalue[1],
        hashvalue[2],
        hashvalue[3],
        hashvalue[4],
        hashvalue[5],
        hashvalue[6],
        hashvalue[7],
        hashvalue[8],
        hashvalue[9],
        hashvalue[10],
        hashvalue[11],
        hashvalue[12],
        hashvalue[13],
        hashvalue[14],
        hashvalue[15],
        hashvalue[16],
        hashvalue[17],
        hashvalue[18],
        hashvalue[19],
        hashvalue[20],
        hashvalue[21],
        hashvalue[22],
        hashvalue[23],
        hashvalue[24],
        hashvalue[25],
        hashvalue[26],
        hashvalue[27],
        hashvalue[28],
        hashvalue[29],
        hashvalue[30],
        hashvalue[31],
    ];
    x
}

pub async fn notify(
    caller: Principal,
    paid: Invoice,
    locktime: u64,
    block: u64
) -> Result<(), String> {
    let LEDGER_CANISTER: Principal = ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
    let amt = paid.amount;
    let hash: [u8; 32] = hash_invoice(paid);
    let call = ic_ledger_types::account_balance(ic_ledger_types::MAINNET_LEDGER_CANISTER_ID, AccountBalanceArgs{account: AccountIdentifier::new(&api::id(), &Subaccount(hash))}).await;
    // let call : Result<(u64,), (ic_cdk::api::call::RejectionCode, std::string::String)> = ic::call(LEDGER_CANISTER, "account_balance", (AccountIdentifier::new(&api::id(), &Subaccount(hash)),)).await;
    // let call = ic::call(LEDGER_CANISTER, "account_balance", (AccountIdentifier::new(&api::id(), &Subaccount(hash)),)).await;
    // let amt = await ic::call(LEDGER_CANISTER, "account_balance", AccountIdentifier::new(api::id(), hash));
    // let amt = match amt {
    //     Ok(amount) => amount,
    //     Err(error) => {
    //         return Err("Canister account balance call invalid.".to_string());
    //     }
    // };
    // let result : Result<u64, (ic_cdk::api::call::RejectionCode, String)> = call as Result<T, E>;
    if !call.is_ok() {
        return Err("Legister canister call was unsuccessful.".to_string());
    }
    if call.unwrap().e8s() != amt {
        return Err("Canister subaccount did not receive invoice amount.".to_string());
    }
    let utc: DateTime<Utc> = Utc::now();
    let seconds: u64 = utc.timestamp().unsigned_abs();
    let base: u64 = 10;
    let nanoseconds: u64 = seconds * base.pow(9);
    let memo = Memo(0);

    let subaccount : Option<Subaccount> = Some(Subaccount(hash));
    let timestamp: Option<Timestamp> = Some(Timestamp {
        timestamp_nanos: nanoseconds
    });
    ic_ledger_types::transfer(ic_ledger_types::MAINNET_LEDGER_CANISTER_ID, TransferArgs {
        memo: memo,
        amount: Tokens::from_e8s(amt),
        fee: ic_ledger_types::DEFAULT_FEE,
        from_subaccount: subaccount,
        to: AccountIdentifier::new(&api::id(), &ic_ledger_types::DEFAULT_SUBACCOUNT),
        created_at_time: timestamp
    });

    // copied from stake fn below (above is to verify user placed appropriate funds in one-time account)
    let stakers = ic::get_mut::<Stakers>();
    let transactions = ic::get_mut::<Transactions>();
    
    let current_stake = stakers.get(&caller).copied().unwrap_or(0);

    stakers.insert(caller, amt + current_stake);

    let tx_map = transactions.entry(caller).or_insert_with(|| HashMap::new());

    let take_tx = Transaction {
        amount: amt,
        time: nanoseconds,
        locktime: locktime,
        return_amount: calculateReturnLocked(caller, nanoseconds, locktime, amt)
    };

    // was tx_list before, but changed since it was giving error
    // ******* COMMENTED OUT DUE TO ERRORS ********
    // tx_map.insert(locktime + nanoseconds, take_tx);

    // add transfer function call

    // transfer voting tokens (don't delete) and add proper error handling
    let numVotes : u64 = calculateNumVoteTokens(amt);
    let MINTING_CANISTER: Principal = Principal::from_str("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    ic::call(MINTING_CANISTER, "transfer", (caller, numVotes, ""))
         .await
         .map_err(|(code, msg)| format!("Call failed with code={}: {}", code as u8, msg))?;
    // let minting = minting_canister(Principal::from_text("minting-canister-id").unwrap())
    // minting.transfer(...)
    // ic::call(MINTING_CANISTER, "transfer", (caller, numVotes));
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
    amount: u64,
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
) -> u64 {
    unlockFunds(caller, timestamp);
    let unlock_amt = getUnlockedAmount(caller, timestamp);
    let unlocked = ic::get_mut::<Unlocked>();
    unlocked.insert(caller, 0);
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
    let mut new_unlock = unlocked.get(&caller).copied().unwrap_or(0);

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
) -> u64 {
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
) -> u64 {
    let num_years = (locktime as f64) / (TIME_STEPS_PER_YEAR as f64);
    let r_float = num_years * APY * (amount as f64) + (amount as f64);
    r_float as u64
}

fn calculateNumVoteTokens(
    mut staked: u64,
) -> u64 {
    // minimum amount is the same as the cost of one vote token
    let cost : u64 = FIRST_VOTE_COST;
    let mut votes : u64 = 0;
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
