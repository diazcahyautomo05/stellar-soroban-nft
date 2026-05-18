#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String, Vec, symbol_short,
};





#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTMetadata {
    pub token_id:       u32,
    pub name:           String,
    pub description:    String,
    pub uri:            String,
    pub owner:          Address,
    pub creator:        Address,
    pub is_burned:      bool,
    pub mint_timestamp: u64,
    pub transfer_count: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionInfo {
    pub name:               String,
    pub symbol:             String,
    pub description:        String,
    pub max_supply:         u32,
    pub total_minted:       u32,
    pub royalty_bps:        u32,
    pub royalty_receiver:   Address,
    pub is_paused:          bool,
    pub whitelist_enabled:  bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoyaltyInfo {
    pub receiver:   Address,
    pub amount_bps: u32,
}

#[contracttype]
pub enum DataKey {

    Admin,
    CollectionName,
    CollectionSymbol,
    CollectionDesc,
    MaxSupply,
    TokenCount,
    ActiveCount,
    Paused,
    WhitelistEnabled,


    RoyaltyBps,
    RoyaltyReceiver,


    Token(u32),
    OwnerTokens(Address),


    Approved(u32),
    OperatorApproval(Address, Address),


    Whitelist(Address),
}





#[contract]
pub struct NFTContract;

#[contractimpl]
impl NFTContract {








    pub fn initialize(
        env:                Env,
        admin:              Address,
        collection_name:    String,
        collection_symbol:  String,
        collection_desc:    String,
        max_supply:         u32,
        royalty_bps:        u32,
        royalty_receiver:   Address,
    ) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        if royalty_bps > 10000 {
            panic!("royalty_bps cannot exceed 10000 (100%)");
        }

        env.storage().instance().set(&DataKey::Admin,            &admin);
        env.storage().instance().set(&DataKey::CollectionName,   &collection_name);
        env.storage().instance().set(&DataKey::CollectionSymbol, &collection_symbol);
        env.storage().instance().set(&DataKey::CollectionDesc,   &collection_desc);
        env.storage().instance().set(&DataKey::MaxSupply,        &max_supply);
        env.storage().instance().set(&DataKey::TokenCount,       &0u32);
        env.storage().instance().set(&DataKey::ActiveCount,      &0u32);
        env.storage().instance().set(&DataKey::Paused,           &false);
        env.storage().instance().set(&DataKey::WhitelistEnabled, &false);
        env.storage().instance().set(&DataKey::RoyaltyBps,       &royalty_bps);
        env.storage().instance().set(&DataKey::RoyaltyReceiver,  &royalty_receiver);


        env.events().publish(
            (symbol_short!("init"), symbol_short!("col")),
            collection_name,
        );
    }






    pub fn mint(
        env:         Env,
        to:          Address,
        name:        String,
        description: String,
        uri:         String,
    ) -> u32 {
        Self::require_admin(&env);
        Self::require_not_paused(&env);
        Self::check_max_supply(&env);

        if Self::is_whitelist_enabled(&env) && !Self::is_whitelisted(env.clone(), to.clone()) {
            panic!("address not whitelisted");
        }

        let token_id = Self::next_token_id(&env);

        let nft = NFTMetadata {
            token_id,
            name:           name.clone(),
            description,
            uri,
            owner:          to.clone(),
            creator:        to.clone(),
            is_burned:      false,
            mint_timestamp: env.ledger().timestamp(),
            transfer_count: 0,
        };

        Self::save_token(&env, &nft);
        Self::add_token_to_owner(&env, &to, token_id);
        Self::increment_active_count(&env);


        env.events().publish(
            (symbol_short!("mint"), token_id),
            to,
        );

        token_id
    }


    pub fn batch_mint(
        env:          Env,
        to:           Address,
        names:        Vec<String>,
        descriptions: Vec<String>,
        uris:         Vec<String>,
    ) -> Vec<u32> {
        Self::require_admin(&env);
        Self::require_not_paused(&env);

        let count = names.len();
        if count != descriptions.len() || count != uris.len() {
            panic!("names, descriptions, uris must have equal length");
        }
        if count == 0 {
            panic!("batch cannot be empty");
        }


        let max_supply: u32 = env.storage().instance()
            .get(&DataKey::MaxSupply).unwrap_or(0);
        if max_supply > 0 {
            let token_count: u32 = env.storage().instance()
                .get(&DataKey::TokenCount).unwrap_or(0);
            if token_count + count > max_supply {
                panic!("batch would exceed max supply");
            }
        }

        if Self::is_whitelist_enabled(&env) && !Self::is_whitelisted(env.clone(), to.clone()) {
            panic!("address not whitelisted");
        }

        let mut minted_ids: Vec<u32> = Vec::new(&env);

        for i in 0..count {
            let token_id = Self::next_token_id(&env);
            let nft = NFTMetadata {
                token_id,
                name:           names.get(i).unwrap(),
                description:    descriptions.get(i).unwrap(),
                uri:            uris.get(i).unwrap(),
                owner:          to.clone(),
                creator:        to.clone(),
                is_burned:      false,
                mint_timestamp: env.ledger().timestamp(),
                transfer_count: 0,
            };

            Self::save_token(&env, &nft);
            Self::add_token_to_owner(&env, &to, token_id);
            Self::increment_active_count(&env);

            env.events().publish(
                (symbol_short!("mint"), token_id),
                to.clone(),
            );

            minted_ids.push_back(token_id);
        }

        minted_ids
    }







    pub fn transfer(env: Env, from: Address, to: Address, token_id: u32) {
        from.require_auth();
        Self::require_not_paused(&env);

        let mut nft = Self::get_token_or_panic(&env, token_id);

        if nft.is_burned {
            panic!("token is burned");
        }


        let is_approved_addr = env.storage().instance()
            .get::<DataKey, Address>(&DataKey::Approved(token_id))
            .map(|a| a == from)
            .unwrap_or(false);

        let is_operator = env.storage().instance()
            .get::<DataKey, bool>(&DataKey::OperatorApproval(nft.owner.clone(), from.clone()))
            .unwrap_or(false);

        if nft.owner != from && !is_approved_addr && !is_operator {
            panic!("not owner or approved");
        }


        Self::remove_token_from_owner(&env, &nft.owner, token_id);
        nft.owner = to.clone();
        nft.transfer_count += 1;
        Self::save_token(&env, &nft);
        Self::add_token_to_owner(&env, &to, token_id);


        env.storage().instance().remove(&DataKey::Approved(token_id));


        let royalty_bps: u32 = env.storage().instance()
            .get(&DataKey::RoyaltyBps).unwrap_or(0);
        env.events().publish(
            (symbol_short!("transfer"), token_id),
            (from, to, royalty_bps),
        );
    }






    pub fn approve(env: Env, owner: Address, approved: Address, token_id: u32) {
        owner.require_auth();

        let nft = Self::get_token_or_panic(&env, token_id);
        if nft.owner != owner {
            panic!("not owner");
        }
        if nft.is_burned {
            panic!("token is burned");
        }

        env.storage().instance()
            .set(&DataKey::Approved(token_id), &approved);

        env.events().publish(
            (symbol_short!("approve"), token_id),
            approved,
        );
    }


    pub fn set_approval_for_all(
        env:      Env,
        owner:    Address,
        operator: Address,
        approved: bool,
    ) {
        owner.require_auth();
        if owner == operator {
            panic!("owner cannot be operator of themselves");
        }

        env.storage().instance().set(
            &DataKey::OperatorApproval(owner.clone(), operator.clone()),
            &approved,
        );

        env.events().publish(
            (symbol_short!("op_appr"), symbol_short!("all")),
            (owner, operator, approved),
        );
    }






    pub fn burn(env: Env, owner: Address, token_id: u32) {
        owner.require_auth();
        Self::require_not_paused(&env);

        let mut nft = Self::get_token_or_panic(&env, token_id);

        if nft.owner != owner {
            panic!("not owner");
        }
        if nft.is_burned {
            panic!("already burned");
        }

        nft.is_burned = true;
        Self::save_token(&env, &nft);
        Self::remove_token_from_owner(&env, &owner, token_id);
        env.storage().instance().remove(&DataKey::Approved(token_id));


        let active: u32 = env.storage().instance()
            .get(&DataKey::ActiveCount).unwrap_or(0);
        if active > 0 {
            env.storage().instance()
                .set(&DataKey::ActiveCount, &(active - 1));
        }

        env.events().publish(
            (symbol_short!("burn"), token_id),
            owner,
        );
    }






    pub fn update_uri(env: Env, token_id: u32, new_uri: String) {
        Self::require_admin(&env);

        let mut nft = Self::get_token_or_panic(&env, token_id);
        if nft.is_burned {
            panic!("token is burned");
        }

        nft.uri = new_uri;
        Self::save_token(&env, &nft);
    }


    pub fn update_royalty(env: Env, royalty_bps: u32, royalty_receiver: Address) {
        Self::require_admin(&env);

        if royalty_bps > 10000 {
            panic!("royalty_bps cannot exceed 10000");
        }

        env.storage().instance().set(&DataKey::RoyaltyBps,      &royalty_bps);
        env.storage().instance().set(&DataKey::RoyaltyReceiver, &royalty_receiver);
    }


    pub fn update_max_supply(env: Env, new_max: u32) {
        Self::require_admin(&env);

        let token_count: u32 = env.storage().instance()
            .get(&DataKey::TokenCount).unwrap_or(0);


        if new_max != 0 && new_max < token_count {
            panic!("new max supply below current minted count");
        }

        env.storage().instance().set(&DataKey::MaxSupply, &new_max);
    }






    pub fn pause(env: Env) {
        Self::require_admin(&env);
        env.storage().instance().set(&DataKey::Paused, &true);
        env.events().publish(
            (symbol_short!("paused"),),
            true,
        );
    }


    pub fn unpause(env: Env) {
        Self::require_admin(&env);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.events().publish(
            (symbol_short!("paused"),),
            false,
        );
    }


    pub fn transfer_admin(env: Env, new_admin: Address) {
        Self::require_admin(&env);
        new_admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }






    pub fn toggle_whitelist(env: Env, enabled: bool) {
        Self::require_admin(&env);
        env.storage().instance().set(&DataKey::WhitelistEnabled, &enabled);
    }


    pub fn add_to_whitelist(env: Env, address: Address) {
        Self::require_admin(&env);
        env.storage().instance().set(&DataKey::Whitelist(address), &true);
    }


    pub fn remove_from_whitelist(env: Env, address: Address) {
        Self::require_admin(&env);
        env.storage().instance().remove(&DataKey::Whitelist(address));
    }


    pub fn batch_add_whitelist(env: Env, addresses: Vec<Address>) {
        Self::require_admin(&env);
        for addr in addresses.iter() {
            env.storage().instance().set(&DataKey::Whitelist(addr), &true);
        }
    }






    pub fn get_nft(env: Env, token_id: u32) -> NFTMetadata {
        Self::get_token_or_panic(&env, token_id)
    }


    pub fn owner_of(env: Env, token_id: u32) -> Address {
        let nft = Self::get_token_or_panic(&env, token_id);
        if nft.is_burned { panic!("token is burned"); }
        nft.owner
    }


    pub fn tokens_of(env: Env, owner: Address) -> Vec<u32> {
        env.storage().instance()
            .get(&DataKey::OwnerTokens(owner))
            .unwrap_or(Vec::new(&env))
    }


    pub fn get_approved(env: Env, token_id: u32) -> Option<Address> {
        env.storage().instance()
            .get(&DataKey::Approved(token_id))
    }


    pub fn is_approved_for_all(env: Env, owner: Address, operator: Address) -> bool {
        env.storage().instance()
            .get(&DataKey::OperatorApproval(owner, operator))
            .unwrap_or(false)
    }






    pub fn total_minted(env: Env) -> u32 {
        env.storage().instance()
            .get(&DataKey::TokenCount)
            .unwrap_or(0)
    }


    pub fn total_supply(env: Env) -> u32 {
        env.storage().instance()
            .get(&DataKey::ActiveCount)
            .unwrap_or(0)
    }


    pub fn collection_info(env: Env) -> CollectionInfo {
        CollectionInfo {
            name:               env.storage().instance().get(&DataKey::CollectionName).unwrap(),
            symbol:             env.storage().instance().get(&DataKey::CollectionSymbol).unwrap(),
            description:        env.storage().instance().get(&DataKey::CollectionDesc).unwrap(),
            max_supply:         env.storage().instance().get(&DataKey::MaxSupply).unwrap_or(0),
            total_minted:       env.storage().instance().get(&DataKey::TokenCount).unwrap_or(0),
            royalty_bps:        env.storage().instance().get(&DataKey::RoyaltyBps).unwrap_or(0),
            royalty_receiver:   env.storage().instance().get(&DataKey::RoyaltyReceiver).unwrap(),
            is_paused:          env.storage().instance().get(&DataKey::Paused).unwrap_or(false),
            whitelist_enabled:  env.storage().instance().get(&DataKey::WhitelistEnabled).unwrap_or(false),
        }
    }


    pub fn royalty_info(env: Env) -> RoyaltyInfo {
        RoyaltyInfo {
            receiver:   env.storage().instance().get(&DataKey::RoyaltyReceiver).unwrap(),
            amount_bps: env.storage().instance().get(&DataKey::RoyaltyBps).unwrap_or(0),
        }
    }


    pub fn is_paused(env: Env) -> bool {
        env.storage().instance()
            .get(&DataKey::Paused)
            .unwrap_or(false)
    }


    pub fn is_whitelisted(env: Env, address: Address) -> bool {
        env.storage().instance()
            .get(&DataKey::Whitelist(address))
            .unwrap_or(false)
    }


    pub fn get_admin(env: Env) -> Address {
        env.storage().instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic!("not initialized"))
    }





    fn require_admin(env: &Env) {
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic!("not initialized"));
        admin.require_auth();
    }

    fn require_not_paused(env: &Env) {
        let paused: bool = env.storage().instance()
            .get(&DataKey::Paused)
            .unwrap_or(false);
        if paused {
            panic!("contract is paused");
        }
    }

    fn check_max_supply(env: &Env) {
        let max_supply: u32 = env.storage().instance()
            .get(&DataKey::MaxSupply).unwrap_or(0);
        if max_supply == 0 { return; }

        let token_count: u32 = env.storage().instance()
            .get(&DataKey::TokenCount).unwrap_or(0);
        if token_count >= max_supply {
            panic!("max supply reached");
        }
    }

    fn is_whitelist_enabled(env: &Env) -> bool {
        env.storage().instance()
            .get(&DataKey::WhitelistEnabled)
            .unwrap_or(false)
    }

    fn next_token_id(env: &Env) -> u32 {
        let count: u32 = env.storage().instance()
            .get(&DataKey::TokenCount).unwrap_or(0);
        env.storage().instance()
            .set(&DataKey::TokenCount, &(count + 1));
        count
    }

    fn increment_active_count(env: &Env) {
        let active: u32 = env.storage().instance()
            .get(&DataKey::ActiveCount).unwrap_or(0);
        env.storage().instance()
            .set(&DataKey::ActiveCount, &(active + 1));
    }

    fn save_token(env: &Env, nft: &NFTMetadata) {
        env.storage().instance().set(&DataKey::Token(nft.token_id), nft);
    }

    fn get_token_or_panic(env: &Env, token_id: u32) -> NFTMetadata {
        env.storage().instance()
            .get(&DataKey::Token(token_id))
            .unwrap_or_else(|| panic!("token not found"))
    }

    fn add_token_to_owner(env: &Env, owner: &Address, token_id: u32) {
        let mut tokens: Vec<u32> = env.storage().instance()
            .get(&DataKey::OwnerTokens(owner.clone()))
            .unwrap_or(Vec::new(env));
        tokens.push_back(token_id);
        env.storage().instance()
            .set(&DataKey::OwnerTokens(owner.clone()), &tokens);
    }

    fn remove_token_from_owner(env: &Env, owner: &Address, token_id: u32) {
        let mut tokens: Vec<u32> = env.storage().instance()
            .get(&DataKey::OwnerTokens(owner.clone()))
            .unwrap_or(Vec::new(env));

        let len = tokens.len();
        let mut idx: Option<u32> = None;
        let mut i = 0u32;
        while i < len {
            if tokens.get(i).unwrap() == token_id {
                idx = Some(i);
                break;
            }
            i += 1;
        }

        if let Some(i) = idx {
            tokens.remove(i);
        }

        env.storage().instance()
            .set(&DataKey::OwnerTokens(owner.clone()), &tokens);
    }
}


#[cfg(test)]
mod test;