// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault, BorshStorageKey, AccountId};
use near_sdk::store::{LookupSet, LookupMap, Vector};
use std::collections::HashSet;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct GhanaAsaase {
    land_owner: LookupMap<String, String>,
    owner_lands: LookupMap<String, Vector<String>>,
    admins: LookupSet<AccountId>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    LandOwner,
    OwnerLands,
    Admins,
}

#[near_bindgen]
impl GhanaAsaase {

    // init function for the smart contract
    #[init]
    pub fn new(admins: HashSet<AccountId>) -> Self {
        let mut initialAdmins = LookupSet::new(StorageKeys::Admins);
        initialAdmins.extend(admins);
        Self {
            admins: initialAdmins,
            land_owner: LookupMap::new(StorageKeys::LandOwner),
            owner_lands: LookupMap::new(StorageKeys::OwnerLands)
         }
    }
    pub fn assign_ownership(&mut self, owner_id: String, land_address: String) {
        self.reqire_admin();
    }
    pub fn change_ownership(&mut self, land_address: String, old_owner_id: String, new_owner_id: String) {
        self.reqire_admin();
    }
    pub fn get_owner_lands(&self) -> Vec<String>{
        vec![]
    }
    pub fn get_land_owner(&mut self, land_address: String) -> String {
        "asdfd".to_string()
    }
    fn reqire_admin(&self) {
        assert!(self.admins.contains(&env::predecessor_account_id()))
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_ownership() {
        let contract = GhanaAsaase::default();
    }

    #[test]
    fn change_ownership() {
        let contract = GhanaAsaase::default();
    }

    #[test]
    fn get_owner_lands() {
        let contract = GhanaAsaase::default();
    }

    #[test]
    fn get_land_owner() {
        let contract = GhanaAsaase::default();
    }
}
