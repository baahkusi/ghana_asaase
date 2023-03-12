// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::{LookupMap, LookupSet, UnorderedSet};
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};
use std::collections::HashSet;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct GhanaAsaase {
    land_owner: LookupMap<String, AccountId>, // land_address:land_owner
    owner_lands: LookupMap<AccountId, UnorderedSet<String>>, // land_owner: land_addresses
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
        self.require_admin();
        require!(
            !self.land_owner.contains_key(&land_address),
            "Land is already owned by someone, cannot assign new ownership ..."
        );
        self.land_owner
            .insert(land_address.clone(), owner_id.clone());
        match self.owner_lands.get_mut(&owner_id) {
            Some(lands) => {
                lands.insert(land_address);
            }
            None => {
                let mut lands = UnorderedSet::new(StorageKeys::OwnerLands(owner_id.clone()));
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
        self.require_admin();
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
                let mut lands = UnorderedSet::new(StorageKeys::OwnerLands(new_owner_id.clone()));
                lands.insert(cloned_land);
                self.owner_lands.insert(new_owner_id, lands);
            }
        }
        let old_owner_lands = self.owner_lands.get_mut(&old_owner_id).unwrap();
        old_owner_lands.remove(&land_address);
    }
    pub fn get_owner_lands(&self, owner_id: AccountId) -> Vec<String> {
        match self.owner_lands.get(&owner_id) {
            Some(lands) => {
                let mut owner_lands = vec![];
                for l in lands {
                    owner_lands.push(l.to_owned());
                }
                owner_lands
            }
            None => vec![],
        }
    }
    pub fn get_land_owner(&self, land_address: String) -> Option<&AccountId> {
        self.land_owner.get(&land_address)
    }
    fn require_admin(&self) {
        require!(
            self.admins.contains(&env::signer_account_id()),
            "Admin required ..."
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, Balance};

    use std::collections::HashSet;

    const NEAR: u128 = 1000000000000000000000000;

    fn set_context(predecessor: AccountId) {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(predecessor);
        builder.attached_deposit(1 * NEAR);
        testing_env!(builder.build());
    }

    fn admins() -> HashSet<AccountId> {
        HashSet::from([0].map(|n| accounts(n)))
    }

    fn owner(id: usize) -> AccountId {
        [accounts(2), accounts(3), accounts(4)][id].to_owned()
    }

    fn not_owner() -> AccountId {
        accounts(5)
    }

    fn not_admin() -> AccountId {
        accounts(1)
    }

    fn land(id: usize) -> String {
        String::from(
            [
                "Accra",
                "Kumasi",
                "Cape Coast",
                "Takoradi",
                "Wale Wale",
                "Yor Yor Tatale",
                "Oda",
                "Kwahu",
            ][id],
        )
    }

    #[test]
    fn assign_ownership() {
        let mut contract = GhanaAsaase::new(admins());
        let one_admin = accounts(0);
        set_context(one_admin);
        contract.assign_ownership(land(0), owner(0));
        contract.assign_ownership(land(1), owner(1));
        contract.assign_ownership(land(2), owner(2));
        contract.assign_ownership(land(3), owner(2));
        contract.assign_ownership(land(4), owner(2));
        assert_eq!(
            contract
                .get_land_owner(land(0))
                .expect("land has no owner ..."),
            &owner(0)
        );
        assert_eq!(
            contract
                .get_land_owner(land(1))
                .expect("land has no owner ..."),
            &owner(1)
        );
        assert_eq!(
            contract
                .get_land_owner(land(3))
                .expect("land has no owner ..."),
            &owner(2)
        );
        let owner0_lands = vec![land(0)];
        assert_eq!(owner0_lands, contract.get_owner_lands(owner(0)));
        let owner1_lands = vec![land(1)];
        assert_eq!(owner1_lands, contract.get_owner_lands(owner(1)));
        let owner2_lands = vec![land(2), land(3), land(4)];
        assert_eq!(owner2_lands, contract.get_owner_lands(owner(2)));
        let not_owner_lands: Vec<String> = vec![];
        assert_eq!(not_owner_lands, contract.get_owner_lands(not_owner()))
    }

    #[test]
    #[should_panic]
    fn assign_ownership_not_admin() {
        let mut contract = GhanaAsaase::new(admins());
        contract.assign_ownership(land(0), owner(0));
    }

    #[test]
    #[should_panic]
    fn double_assign_ownership() {
        let mut contract = GhanaAsaase::new(admins());
        let one_admin = accounts(0);
        set_context(one_admin);
        contract.assign_ownership(land(2), owner(2));
        contract.assign_ownership(land(2), owner(1));
    }

    #[test]
    fn change_ownership() {
        let mut contract = GhanaAsaase::new(admins());
        let one_admin = accounts(0);
        set_context(one_admin);
        contract.assign_ownership(land(2), owner(2));
        contract.assign_ownership(land(3), owner(2));
        contract.assign_ownership(land(4), owner(2));
        assert_eq!(
            contract
                .get_land_owner(land(2))
                .expect("land has no owner ..."),
            &owner(2)
        );
        contract.change_ownership(land(2), owner(2), owner(1));
        assert_eq!(
            contract
                .get_land_owner(land(2))
                .expect("land has no owner ..."),
            &owner(1)
        );
    }

    #[test]
    #[should_panic]
    fn change_ownership_not_admin() {
        let mut contract = GhanaAsaase::new(admins());
        let one_admin = accounts(0);
        set_context(one_admin);
        contract.assign_ownership(land(2), owner(2));
        contract.assign_ownership(land(3), owner(2));
        contract.assign_ownership(land(4), owner(2));
        assert_eq!(
            contract
                .get_land_owner(land(2))
                .expect("land has no owner ..."),
            &owner(2)
        );
        let not_admin = not_admin();
        set_context(not_admin);
        contract.change_ownership(land(2), owner(2), owner(1));
        assert_eq!(
            contract
                .get_land_owner(land(2))
                .expect("land has no owner ..."),
            &owner(1)
        );
    }

    #[test]
    #[should_panic]
    fn change_ownership_wrong_owner() {
        let mut contract = GhanaAsaase::new(admins());
        let one_admin = accounts(0);
        set_context(one_admin);
        contract.assign_ownership(land(2), owner(1));
        contract.assign_ownership(land(3), owner(2));
        contract.assign_ownership(land(4), owner(2));
        assert_eq!(
            contract
                .get_land_owner(land(2))
                .expect("land has no owner ..."),
            &owner(2)
        );
        contract.change_ownership(land(2), owner(2), owner(1));
        assert_eq!(
            contract
                .get_land_owner(land(2))
                .expect("land has no owner ..."),
            &owner(1)
        );
    }
}
