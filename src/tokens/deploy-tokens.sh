sudo dfx canister --no-wallet create --all
cargo run > src/tokens/src/token.did
sudo dfx build tokens
OWNER="principal \"$( \
   dfx identity get-principal
)\""
sudo dfx canister --no-wallet install tokens --argument "(\"test logo\", \"test token\", \"TT\", 8:nat8, 100000000:nat64, $OWNER, 0)" -m=reinstall
