mod main
use std::collections::HashMap;
use ic_kit::{ic , Principal};
use num_traits::pow;
//assuming time is in seconds
static REWARD_CONST: f32 = 1209600.0;

type Stakers = HashMap<Principal, u64>;
type Transactions = HashMap<Principal, LinkedList<Transaction>>;

struct Transaction {
    amount: u64,
    time: u64,
}

fn stake(
    caller: Option<Principal>,
    amount: u64,
    fee: u64,
    timestamp: u64,
) -> bool {
    let stakers = ic::get_mut::<Stakers>();
    let transactions = ic::get_mut::<Transactions>();
    if !stakers.contains_key(caller) {
        stakers.insert(caller, 0);
    }
    u64 current = stakers.get(caller);
    stakers.insert(caller, amount + current);
    if !transactions.contains_key(caller) {
        transactions.insert(caller, LinkedList::new());
    }
    let tx_list = transactions.get(caller)
    tx_list.push_back(Transaction{
        amount: amount,
        time: time,
    });
    
    true
}

fn removeStake(
    caller: Option<Principal>,
    amount: u64,
    fee: u64,
    timestamp: u64,
) -> bool {
    let stakers = ic::get::<Stakers>();
    let transactions = ic::get_mut::<Transactions>();

    if stakers.get(caller) < amount {
        false
    }
    match stakers.get(&caller) {
        Some(balance) => balance - amount,
        None => 0,
    }
    let tx_list = transactions.get(caller);
    while amount > 0 && !tx_list.is_empty()  {
        let topTx = tx_list.pop_back();
        let topVal = topTx.amount;
        if amount == topVal {
            break;
        } else if amount > topVal {
            amount -= topVal;
        } else {
            topVal.amount -= amount;
            tx_list.push_back(topTx);
        }
    }
    true
}

fn calculateReward(
    caller: Option<Principal>,
    fee: u64,
    timestamp: u64,
) -> f64 {
    let transactions = ic::get_mut::<Transactions>();
    let tx_list = transactions.get(caller);
    let reward_amount = 0;
    while !tx_list.is_empty() {
        let top_Tx = tx_list.pop_back();
        let val = top_Tx.amount;
        let time = top_Tx.time;
        //todo: Add cliffs
        let reward_mul = (timestamp - time) / REWARD_CONST;
        reward_amount += reward_mul * val;
    }
    reward_amount
}

// changed f32 type to u32
static VOTE_EXPONENT: u32 = 2;

#[ic_cdk_macros::query]
fn getNumVotes(icpAdded: u32, currVotes: u32) {
    //new votes: currvotes ^ vote_exp + icpAdded
    let num_icp = currVotes.pow(VOTE_EXPONENT);
    num_icp += icpAdded;
    let conv_ratio = 1 / VOTE_EXPONENT;
    ic_cdk::print(num_icp.pow(conv_ratio));
}