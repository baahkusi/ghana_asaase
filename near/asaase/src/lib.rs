// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::{LookupMap, LookupSet};
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};
use std::collections::HashSet;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct GhanaAsaase {
    land_owner: LookupMap<String, AccountId>, // land_address:land_owner
    owner_lands: LookupMap<AccountId, LookupSet<String>>, // land_owner: land_addresses
    admins: LookupSet<AccountId>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Land,
    OwnerLands(AccountId),
    Owner,
    Admins,
}

#[near_bindgen]
impl GhanaAsaase {
    #[init]
    pub fn new(admins: HashSet<AccountId>) -> Self {
        let mut initial_admins = LookupSet::new(StorageKeys::Admins);
        initial_admins.extend(admins);
        Self {
            admins: initial_admins,
            land_owner: LookupMap::new(StorageKeys::Land),
            owner_lands: LookupMap::new(StorageKeys::Owner),
        }
    }
    pub fn assign_ownership(&mut self, land_address: String, owner_id: AccountId) {
        self.reqire_admin();
        require!(
            !self.land_owner.contains_key(&land_address),
            "Land is already owned by someone, cannot assing new ownership ..."
        );
        self.land_owner
            .insert(land_address.clone(), owner_id.clone());
        match self.owner_lands.get_mut(&owner_id) {
            Some(lands) => {
                lands.insert(land_address);
            }
            None => {
                let mut lands = LookupSet::new(StorageKeys::OwnerLands(owner_id.clone()));
                lands.insert(land_address);
                self.owner_lands.insert(owner_id, lands);
            }
        }
    }
    pub fn change_ownership(
        &mut self,
        land_address: String,
        old_owner_id: AccountId,
        new_owner_id: AccountId,
    ) {
        self.reqire_admin();
        let old_owner_from_storage = match self.land_owner.get(&land_address) {
            Some(owner) => owner,
            None => {
                panic!("Land is not owned by anyone yet, cannot perform ownership change ...")
            }
        };
        require!(
            old_owner_from_storage == &old_owner_id,
            old_owner_id.as_str().to_owned()
                + "is not owner of "
                + &land_address
                + "there is another owner ..."
        );
        self.land_owner
            .insert(land_address.clone(), new_owner_id.clone());
        let cloned_land = land_address.clone();
        match self.owner_lands.get_mut(&new_owner_id) {
            Some(lands) => {
                lands.insert(cloned_land);
            }
            None => {
                let mut lands = LookupSet::new(StorageKeys::OwnerLands(new_owner_id.clone()));
                lands.insert(cloned_land);
                self.owner_lands.insert(new_owner_id, lands);
            }
        }
        let old_owner_lands = self.owner_lands.get_mut(&old_owner_id).unwrap();
        old_owner_lands.remove(&land_address);
    }
    pub fn get_owner_lands(&self, owner_id: AccountId) -> Option<&LookupSet<String>> {
        self.owner_lands.get(&owner_id)
    }
    pub fn get_land_owner(&self, land_address: String) -> Option<&AccountId> {
        self.land_owner.get(&land_address)
    }
    fn reqire_admin(&self) {
        require!(
            self.admins.contains(&env::signer_account_id()),
            "Admin required ..."
        )
    }
}

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
