# Stellar Soroban NFT

Smart contract NFT berbasis Soroban dan Stellar Testnet. Project ini dibuat untuk membuat, mengelola, dan menjalankan koleksi NFT secara on-chain menggunakan smart contract Rust di Soroban.

## Fitur

- Initialize koleksi NFT
- Mint NFT
- Batch mint NFT
- Transfer NFT
- Burn NFT
- Approval NFT
- Approval operator
- Whitelist address
- Batch whitelist
- Pause dan unpause contract
- Update URI NFT
- Update royalty
- Update max supply
- Cek owner NFT
- Cek metadata NFT
- Cek total supply
- Cek total minted
- Cek collection info

## Teknologi

- Rust
- Soroban SDK
- Stellar CLI
- Stellar Testnet

## Struktur Folder

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

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- initialize --admin diazcahya05 --collection_name "Stellar Apes" --collection_symbol "SAPE" --collection_desc "NFT Collection di Soroban Testnet" --max_supply 100 --royalty_bps 250 --royalty_receiver diazcahya05

## Cek Collection Info

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- collection_info

## Mint NFT

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- mint --to diazcahya05 --name "Ape #1" --description "NFT pertama saya di Soroban" --uri "QmApe1"

## Cek NFT

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- get_nft --token_id 0

## Cek Owner NFT

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- owner_of --token_id 0

## Transfer NFT

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- transfer --from diazcahya05 --to ADDRESS_TUJUAN --token_id 0

## Burn NFT

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- burn --owner diazcahya05 --token_id 0

## Total Supply

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- total_supply

## Total Minted

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- total_minted

## Aktifkan Whitelist

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- toggle_whitelist --enabled true

## Tambah Address ke Whitelist

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- add_to_whitelist --address ADDRESS_WALLET

## Cek Whitelist

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- is_whitelisted --address ADDRESS_WALLET

## Pause Contract

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- pause

## Unpause Contract

stellar contract invoke --id CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L --source-account diazcahya05 --network testnet -- unpause

## Test Build

stellar contract build

## Test Deploy

stellar contract deploy --source-account diazcahya05

## Link Contract

https://lab.stellar.org/r/testnet/contract/CANIKMRK7XIB7IQF5TQOSQJ2NWEUT7PU3R75OS4MIHTLZAITGMP44Z4L

## License

MIT
