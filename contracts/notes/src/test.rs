#![cfg(test)]

use crate::{NFTContract, NFTContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String, Vec};





struct TestEnv<'a> {
    env:    Env,
    client: NFTContractClient<'a>,
    admin:  Address,
}

fn setup<'a>() -> TestEnv<'a> {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, NFTContract);
    let client = NFTContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "Stellar Apes"),
        &String::from_str(&env, "SAPE"),
        &String::from_str(&env, "NFT Collection di Soroban Testnet"),
        &100u32,
        &250u32,
        &admin,
    );

    TestEnv { env, client, admin }
}

fn mint_one<'a>(t: &TestEnv<'a>, to: &Address, id: u32) -> u32 {
    t.client.mint(
        to,
        &String::from_str(&t.env, &format!("Ape #{}", id)),
        &String::from_str(&t.env, "Seekor kera digital di blockchain Stellar"),
        &String::from_str(&t.env, &format!("ipfs://QmApe{}", id)),
    )
}





#[test]
fn test_initialize_collection_info() {
    let t = setup();
    let info = t.client.collection_info();

    assert_eq!(info.name,       String::from_str(&t.env, "Stellar Apes"));
    assert_eq!(info.symbol,     String::from_str(&t.env, "SAPE"));
    assert_eq!(info.max_supply, 100);
    assert_eq!(info.royalty_bps, 250);
    assert!(!info.is_paused);
    assert!(!info.whitelist_enabled);
    assert_eq!(info.total_minted, 0);

    println!("✅ Initialize & collection_info OK");
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_double_initialize_fails() {
    let t = setup();
    t.client.initialize(
        &t.admin,
        &String::from_str(&t.env, "X"),
        &String::from_str(&t.env, "X"),
        &String::from_str(&t.env, "X"),
        &0u32, &0u32, &t.admin,
    );
}





#[test]
fn test_mint_single() {
    let t = setup();
    let alice = Address::generate(&t.env);

    let tid = mint_one(&t, &alice, 1);
    assert_eq!(tid, 0);
    assert_eq!(t.client.total_supply(), 1);
    assert_eq!(t.client.total_minted(), 1);

    let nft = t.client.get_nft(&tid);
    assert_eq!(nft.owner,         alice.clone());
    assert_eq!(nft.creator,       alice.clone());
    assert_eq!(nft.transfer_count, 0);
    assert!(!nft.is_burned);
    assert!(nft.mint_timestamp > 0);

    println!("✅ Mint single OK — token_id: {}", tid);
}

#[test]
fn test_mint_increments_ids() {
    let t = setup();
    let alice = Address::generate(&t.env);

    let t0 = mint_one(&t, &alice, 1);
    let t1 = mint_one(&t, &alice, 2);
    let t2 = mint_one(&t, &alice, 3);

    assert_eq!(t0, 0);
    assert_eq!(t1, 1);
    assert_eq!(t2, 2);
    assert_eq!(t.client.total_minted(), 3);
    assert_eq!(t.client.tokens_of(&alice).len(), 3);

    println!("✅ Mint multiple IDs increments correctly");
}

#[test]
fn test_batch_mint() {
    let t = setup();
    let alice = Address::generate(&t.env);

    let mut names = Vec::new(&t.env);
    let mut descs = Vec::new(&t.env);
    let mut uris  = Vec::new(&t.env);

    for i in 0..5u32 {
        names.push_back(String::from_str(&t.env, &format!("Ape #{}", i)));
        descs.push_back(String::from_str(&t.env, "Batch minted NFT"));
        uris.push_back(String::from_str(&t.env, &format!("ipfs://QmBatch{}", i)));
    }

    let ids = t.client.batch_mint(&alice, &names, &descs, &uris);
    assert_eq!(ids.len(), 5);
    assert_eq!(t.client.total_supply(), 5);
    assert_eq!(t.client.tokens_of(&alice).len(), 5);


    for i in 0..5u32 {
        assert_eq!(ids.get(i).unwrap(), i);
    }

    println!("✅ Batch mint 5 NFTs OK");
}

#[test]
#[should_panic(expected = "max supply reached")]
fn test_mint_exceeds_max_supply() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, NFTContract);
    let client = NFTContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);


    client.initialize(
        &admin,
        &String::from_str(&env, "Limited"),
        &String::from_str(&env, "LTD"),
        &String::from_str(&env, "Limited collection"),
        &2u32, &0u32, &admin,
    );

    client.mint(&alice,
        &String::from_str(&env, "NFT 1"),
        &String::from_str(&env, "First"),
        &String::from_str(&env, "ipfs://1"),
    );
    client.mint(&alice,
        &String::from_str(&env, "NFT 2"),
        &String::from_str(&env, "Second"),
        &String::from_str(&env, "ipfs://2"),
    );

    client.mint(&alice,
        &String::from_str(&env, "NFT 3"),
        &String::from_str(&env, "Third"),
        &String::from_str(&env, "ipfs://3"),
    );
}





#[test]
fn test_transfer_basic() {
    let t = setup();
    let alice = Address::generate(&t.env);
    let bob   = Address::generate(&t.env);

    let tid = mint_one(&t, &alice, 1);
    t.client.transfer(&alice, &bob, &tid);

    assert_eq!(t.client.owner_of(&tid), bob);
    assert_eq!(t.client.tokens_of(&alice).len(), 0);
    assert_eq!(t.client.tokens_of(&bob).len(), 1);

    let nft = t.client.get_nft(&tid);
    assert_eq!(nft.transfer_count, 1);
    assert_eq!(nft.creator, alice);

    println!("✅ Transfer basic OK, transfer_count = {}", nft.transfer_count);
}

#[test]
fn test_transfer_chain() {
    let t     = setup();
    let alice = Address::generate(&t.env);
    let bob   = Address::generate(&t.env);
    let carol = Address::generate(&t.env);

    let tid = mint_one(&t, &alice, 1);
    t.client.transfer(&alice, &bob, &tid);
    t.client.transfer(&bob, &carol, &tid);

    assert_eq!(t.client.owner_of(&tid), carol);
    let nft = t.client.get_nft(&tid);
    assert_eq!(nft.transfer_count, 2);
    assert_eq!(nft.creator, alice);

    println!("✅ Transfer chain OK — transfer_count: {}", nft.transfer_count);
}

#[test]
#[should_panic(expected = "not owner or approved")]
fn test_transfer_by_non_owner_fails() {
    let t      = setup();
    let alice  = Address::generate(&t.env);
    let hacker = Address::generate(&t.env);
    let victim = Address::generate(&t.env);

    let tid = mint_one(&t, &alice, 1);
    t.client.transfer(&hacker, &victim, &tid);
}





#[test]
fn test_approve_single_token() {
    let t        = setup();
    let alice    = Address::generate(&t.env);
    let operator = Address::generate(&t.env);
    let bob      = Address::generate(&t.env);

    let tid = mint_one(&t, &alice, 1);
    t.client.approve(&alice, &operator, &tid);

    assert_eq!(t.client.get_approved(&tid), Some(operator.clone()));


    t.client.transfer(&operator, &bob, &tid);
    assert_eq!(t.client.owner_of(&tid), bob);


    assert_eq!(t.client.get_approved(&tid), None);

    println!("✅ Approve single token & transfer OK");
}

#[test]
fn test_set_approval_for_all() {
    let t        = setup();
    let alice    = Address::generate(&t.env);
    let operator = Address::generate(&t.env);
    let bob      = Address::generate(&t.env);

    let tid1 = mint_one(&t, &alice, 1);
    let tid2 = mint_one(&t, &alice, 2);


    t.client.set_approval_for_all(&alice, &operator, &true);
    assert!(t.client.is_approved_for_all(&alice, &operator));


    t.client.transfer(&operator, &bob, &tid1);
    t.client.transfer(&operator, &bob, &tid2);

    assert_eq!(t.client.owner_of(&tid1), bob);
    assert_eq!(t.client.owner_of(&tid2), bob);
    assert_eq!(t.client.tokens_of(&bob).len(), 2);


    t.client.set_approval_for_all(&alice, &operator, &false);
    assert!(!t.client.is_approved_for_all(&alice, &operator));

    println!("✅ SetApprovalForAll & revoke OK");
}

#[test]
#[should_panic(expected = "not owner")]
fn test_approve_by_non_owner_fails() {
    let t      = setup();
    let alice  = Address::generate(&t.env);
    let hacker = Address::generate(&t.env);
    let bob    = Address::generate(&t.env);

    let tid = mint_one(&t, &alice, 1);
    t.client.approve(&hacker, &bob, &tid);
}





#[test]
fn test_burn() {
    let t     = setup();
    let alice = Address::generate(&t.env);

    let tid = mint_one(&t, &alice, 1);
    assert_eq!(t.client.total_supply(), 1);

    t.client.burn(&alice, &tid);

    let nft = t.client.get_nft(&tid);
    assert!(nft.is_burned);
    assert_eq!(t.client.tokens_of(&alice).len(), 0);
    assert_eq!(t.client.total_supply(), 0);
    assert_eq!(t.client.total_minted(), 1);

    println!("✅ Burn OK — total_supply: {}, total_minted: {}",
        t.client.total_supply(), t.client.total_minted());
}

#[test]
#[should_panic(expected = "already burned")]
fn test_double_burn_fails() {
    let t     = setup();
    let alice = Address::generate(&t.env);
    let tid   = mint_one(&t, &alice, 1);
    t.client.burn(&alice, &tid);
    t.client.burn(&alice, &tid);
}

#[test]
#[should_panic(expected = "not owner")]
fn test_burn_by_non_owner_fails() {
    let t      = setup();
    let alice  = Address::generate(&t.env);
    let hacker = Address::generate(&t.env);
    let tid    = mint_one(&t, &alice, 1);
    t.client.burn(&hacker, &tid);
}

#[test]
#[should_panic(expected = "token is burned")]
fn test_transfer_burned_token_fails() {
    let t     = setup();
    let alice = Address::generate(&t.env);
    let bob   = Address::generate(&t.env);
    let tid   = mint_one(&t, &alice, 1);
    t.client.burn(&alice, &tid);
    t.client.transfer(&alice, &bob, &tid);
}





#[test]
fn test_pause_unpause() {
    let t     = setup();
    let alice = Address::generate(&t.env);

    assert!(!t.client.is_paused());
    t.client.pause();
    assert!(t.client.is_paused());
    t.client.unpause();
    assert!(!t.client.is_paused());


    let tid = mint_one(&t, &alice, 1);
    assert_eq!(tid, 0);

    println!("✅ Pause / Unpause OK");
}

#[test]
#[should_panic(expected = "contract is paused")]
fn test_mint_while_paused_fails() {
    let t     = setup();
    let alice = Address::generate(&t.env);
    t.client.pause();
    mint_one(&t, &alice, 1);
}

#[test]
#[should_panic(expected = "contract is paused")]
fn test_transfer_while_paused_fails() {
    let t     = setup();
    let alice = Address::generate(&t.env);
    let bob   = Address::generate(&t.env);
    let tid   = mint_one(&t, &alice, 1);
    t.client.pause();
    t.client.transfer(&alice, &bob, &tid);
}





#[test]
fn test_whitelist_flow() {
    let t     = setup();
    let alice = Address::generate(&t.env);
    let bob   = Address::generate(&t.env);


    t.client.toggle_whitelist(&true);
    assert!(!t.client.is_whitelisted(&alice));


    t.client.add_to_whitelist(&alice);
    assert!(t.client.is_whitelisted(&alice));


    let tid = mint_one(&t, &alice, 1);
    assert_eq!(tid, 0);


    t.client.remove_from_whitelist(&alice);
    assert!(!t.client.is_whitelisted(&alice));

    println!("✅ Whitelist flow OK");
    let _ = bob;
}

#[test]
#[should_panic(expected = "address not whitelisted")]
fn test_mint_not_whitelisted_fails() {
    let t     = setup();
    let alice = Address::generate(&t.env);

    t.client.toggle_whitelist(&true);

    mint_one(&t, &alice, 1);
}

#[test]
fn test_batch_add_whitelist() {
    let t  = setup();
    let a1 = Address::generate(&t.env);
    let a2 = Address::generate(&t.env);
    let a3 = Address::generate(&t.env);

    let mut addrs = Vec::new(&t.env);
    addrs.push_back(a1.clone());
    addrs.push_back(a2.clone());
    addrs.push_back(a3.clone());

    t.client.batch_add_whitelist(&addrs);
    assert!(t.client.is_whitelisted(&a1));
    assert!(t.client.is_whitelisted(&a2));
    assert!(t.client.is_whitelisted(&a3));

    println!("✅ Batch add whitelist OK");
}





#[test]
fn test_royalty_info() {
    let t    = setup();
    let info = t.client.royalty_info();

    assert_eq!(info.amount_bps, 250);
    assert_eq!(info.receiver, t.admin);
    println!("✅ Royalty info OK — {}bps = {}%", info.amount_bps, info.amount_bps / 100);
}

#[test]
fn test_update_royalty() {
    let t            = setup();
    let new_receiver = Address::generate(&t.env);

    t.client.update_royalty(&500u32, &new_receiver);
    let info = t.client.royalty_info();

    assert_eq!(info.amount_bps, 500);
    assert_eq!(info.receiver, new_receiver);
    println!("✅ Update royalty OK — {}%", info.amount_bps / 100);
}

#[test]
#[should_panic(expected = "royalty_bps cannot exceed 10000")]
fn test_invalid_royalty_fails() {
    let t = setup();
    t.client.update_royalty(&10001u32, &t.admin);
}





#[test]
fn test_update_uri() {
    let t     = setup();
    let alice = Address::generate(&t.env);
    let tid   = mint_one(&t, &alice, 1);

    let new_uri = String::from_str(&t.env, "ipfs://QmNewIPFS123");
    t.client.update_uri(&tid, &new_uri);

    let nft = t.client.get_nft(&tid);
    assert_eq!(nft.uri, new_uri);
    println!("✅ Update URI OK");
}





#[test]
fn test_update_max_supply() {
    let t = setup();
    t.client.update_max_supply(&200u32);

    let info = t.client.collection_info();
    assert_eq!(info.max_supply, 200);


    t.client.update_max_supply(&0u32);
    let info = t.client.collection_info();
    assert_eq!(info.max_supply, 0);

    println!("✅ Update max supply OK");
}

#[test]
#[should_panic(expected = "new max supply below current minted count")]
fn test_update_max_supply_below_minted_fails() {
    let t     = setup();
    let alice = Address::generate(&t.env);

    mint_one(&t, &alice, 1);
    mint_one(&t, &alice, 2);
    mint_one(&t, &alice, 3);

    t.client.update_max_supply(&2u32);
}





#[test]
fn test_transfer_admin() {
    let t         = setup();
    let new_admin = Address::generate(&t.env);

    t.client.transfer_admin(&new_admin);
    assert_eq!(t.client.get_admin(), new_admin);
    println!("✅ Transfer admin OK");
}





#[test]
fn test_full_e2e_scenario() {
    let t        = setup();
    let alice    = Address::generate(&t.env);
    let bob      = Address::generate(&t.env);
    let operator = Address::generate(&t.env);

    println!("--- E2E Test Start ---");


    let mut names = Vec::new(&t.env);
    let mut descs = Vec::new(&t.env);
    let mut uris  = Vec::new(&t.env);
    for i in 0..3u32 {
        names.push_back(String::from_str(&t.env, &format!("Ape #{}", i)));
        descs.push_back(String::from_str(&t.env, "Stellar Ape NFT"));
        uris.push_back(String::from_str(&t.env, &format!("ipfs://Qm{}", i)));
    }
    let ids = t.client.batch_mint(&alice, &names, &descs, &uris);
    assert_eq!(t.client.total_supply(), 3);
    println!("✅ [1] Batch mint 3 OK");


    t.client.set_approval_for_all(&alice, &operator, &true);
    println!("✅ [2] SetApprovalForAll OK");


    t.client.transfer(&operator, &bob, &ids.get(0).unwrap());
    assert_eq!(t.client.owner_of(&ids.get(0).unwrap()), bob);
    println!("✅ [3] Operator transfer #0 ke bob OK");


    t.client.approve(&bob, &alice, &ids.get(0).unwrap());
    t.client.transfer(&alice, &alice, &ids.get(0).unwrap());
    println!("✅ [4] Bob approve alice, alice transfer balik OK");


    t.client.update_royalty(&500u32, &t.admin);
    assert_eq!(t.client.royalty_info().amount_bps, 500);
    println!("✅ [5] Update royalty ke 5% OK");


    t.client.pause();


    t.client.unpause();
    println!("✅ [6-7] Pause & unpause OK");


    t.client.burn(&alice, &ids.get(1).unwrap());
    assert_eq!(t.client.total_supply(), 2);
    assert_eq!(t.client.total_minted(), 3);
    println!("✅ [8] Burn #1 OK — active: {}, minted: {}",
        t.client.total_supply(), t.client.total_minted());


    t.client.update_uri(
        &ids.get(2).unwrap(),
        &String::from_str(&t.env, "ipfs://QmMigratedURI"),
    );
    println!("✅ [9] Update URI #2 OK");


    let nft0 = t.client.get_nft(&ids.get(0).unwrap());
    assert_eq!(nft0.creator, alice);
    println!("✅ [10] Creator tetap alice = {:?}", nft0.creator);

    println!("--- ✅ E2E Test PASSED ---");
}