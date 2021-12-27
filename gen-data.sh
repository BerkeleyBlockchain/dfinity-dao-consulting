dfx identity new bob
dfx identity new alice
dfx identity new john

dfx identity use bob
dfx canister call tokens submitApp '(record { grant_size = 5; name = "Blockchain at Berkeley"; description = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." })'

dfx identity use alice
dfx canister call tokens submitApp '(record { grant_size = 5; name = "Centralized Finance at Berkeley"; description = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." })'

dfx identity use john
dfx canister call tokens submitApp '(record { grant_size = 100; name = "Bad Proposal Everyone Hates"; description = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." })'