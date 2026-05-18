# Stellar Soroban NFT

Stellar Soroban NFT is a smart contract project built on Soroban and deployed to the Stellar Testnet. This project is designed to create, manage, and interact with an NFT collection directly on-chain using Rust smart contracts.

## Features

- Initialize NFT collection
- Mint NFT
- Batch mint NFT
- Transfer NFT
- Burn NFT
- Approve NFT transfer
- Set operator approval
- Whitelist address
- Batch whitelist address
- Pause and unpause contract
- Update NFT URI
- Update royalty
- Update max supply
- Check NFT owner
- Check NFT metadata
- Check total supply
- Check total minted
- Check collection information

## Technologies

- Rust
- Soroban SDK
- Stellar CLI
- Stellar Testnet

## Project Structure

contracts/
└── notes/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        └── test.rs

## Build Contract

stellar contract build

## Deploy Contract

stellar contract deploy --source-account diazcahya05

## Contract ID

CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L

## Initialize Contract

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- initialize --admin diazcahya05 --collection_name "Stellar Apes" --collection_symbol "SAPE" --collection_desc "NFT Collection on Soroban Testnet" --max_supply 100 --royalty_bps 250 --royalty_receiver diazcahya05

## Check Collection Info

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- collection_info

## Mint NFT

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- mint --to diazcahya05 --name "Ape #1" --description "My first NFT on Soroban" --uri "QmApe1"

## Check NFT

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- get_nft --token_id 0

## Check NFT Owner

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- owner_of --token_id 0

## Transfer NFT

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- transfer --from diazcahya05 --to DESTINATION_ADDRESS --token_id 0

## Burn NFT

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- burn --owner diazcahya05 --token_id 0

## Total Supply

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- total_supply

## Total Minted

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- total_minted

## Enable Whitelist

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- toggle_whitelist --enabled true

## Add Address to Whitelist

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- add_to_whitelist --address WALLET_ADDRESS

## Check Whitelist

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- is_whitelisted --address WALLET_ADDRESS

## Pause Contract

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- pause

## Unpause Contract

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- unpause

## Test Build

stellar contract build

## Test Deploy

stellar contract deploy --source-account diazcahya05

## Contract Link

https://lab.stellar.org/r/testnet/contract/CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L
## License

MIT
