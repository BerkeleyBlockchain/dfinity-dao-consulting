// mod token;
use std::{collections::HashMap, str::FromStr};
use ic_kit::{ic, Principal};

// TODO: update since time is returned in nanoseconds
//assuming time is in seconds

static REWARD_CONST: f64 = 1209600.0;
static APY: f64 = 0.08;
static TIME_STEPS_PER_YEAR: u64 = 31536000;

type Stakers = HashMap<Principal, u64>;
type Unlocked = HashMap<Principal, u64>; // I think this should be an f64

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
    return Principal::from_str("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
}

pub fn stake(
    caller: Principal,
    amount: u64,
    fee: u64,
    locktime: u64,
    timestamp: u64,
) {
    

    let stakers = ic::get_mut::<Stakers>();
    let transactions = ic::get_mut::<Transactions>();
    
    let current_stake = stakers.get(&caller).copied().unwrap_or(0);

    stakers.insert(caller, amount + current_stake);

    let tx_map = transactions.entry(caller).or_insert_with(|| HashMap::new());

    let take_tx = Transaction {
        amount: amount,
        time: timestamp,
        locktime: locktime,
        return_amount: calculateReturnLocked(caller, fee, timestamp, locktime, amount)
    };

    // whas tx_list before, but changed since it was giving error
    tx_map.insert(locktime + timestamp, take_tx);
}

pub fn get_stakers() -> LinkedList<Principal> {
    let staker_map = ic::get::<Stakers>();
    let stakers = LinkedList<Principal> = LinkedList::new();
    for (&key, value) in staker_map.into_iter() {
        stakers.push_back(&key);
    }
    stakers

}


fn removeUnlocked(
    caller: Principal,
    amount: u64,
    fee: u64,
    timestamp: u64
) -> bool {
    //transfer out
    unlockFunds(caller, fee, timestamp);
    let amt_avail = getUnlockedAmount(caller, fee, timestamp);
    if amount > amt_avail {
        let unlock_amt = getUnlockedAmount(caller, fee, timestamp);
        let unlocked = ic::get_mut::<Unlocked>();
        unlocked.insert(caller, unlock_amt - amount);
        true
    } else {
        false
    }
}

fn removeUnlockedAll(
    caller: Principal,
    fee: u64,
    timestamp: u64,
) -> u64 {
    unlockFunds(caller, fee, timestamp);
    let unlock_amt = getUnlockedAmount(caller, fee, timestamp);
    let unlocked = ic::get_mut::<Unlocked>();
    unlocked.insert(caller, 0);
    unlock_amt
}

fn unlockFunds(
    caller: Principal,
    fee: u64,
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
                new_unlock = new_unlock + (tx.return_amount as u64);
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
    fee: u64,
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
    fee: u64,
    timestamp: u64,
    locktime: u64,
    amount: u64
) -> f64 {
    let num_years = (locktime as f64) / (TIME_STEPS_PER_YEAR as f64);
    num_years * APY * (amount as f64) + (amount as f64)
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
