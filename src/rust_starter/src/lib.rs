#[ic_cdk_macros::query]
static VOTE_EXPONENT: f32 = 2.0;

fn getNumVotes(icpAdded: f32, currVotes: f32) {
    //new votes: currvotes ^ vote_exp + icpAdded
    let num_icp = currVotes.pow(VOTE_EXPONENT)
    num_icp += icpAdded
    let conv_ratio = 1 / VOTE_EXPONENT
    ic_cdk::print(num_icp.pow(conv_ratio));
}