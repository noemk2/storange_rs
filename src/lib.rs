use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
//use near_sdk::AccountId;
//use near_sdk::{env, near_bindgen};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

near_sdk::setup_alloc!();

#[near_bindgen]
//#[derive(Default, BorshDeserialize, BorshSerialize)]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //pub status_updates: HashMap<AccountId, String>,
    pub status_updates: UnorderedMap<String, AccountId>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        // Initializing `status_updates` with unique key prefix.
        Self {
            status_updates: UnorderedMap::new(b"s".to_vec()),
        }
    }

    pub fn set_status(&mut self, status: String) {
        self.status_updates
            .insert(&env::predecessor_account_id(), &status);
        // Note, don't need to check size, since `UnorderedMap` doesn't store all data in memory.
    }

    pub fn delete_status(&mut self) {
        self.status_updates.remove(&env::predecessor_account_id());
    }

    pub fn get_status(&self, account_id: AccountId) -> Option<String> {
        self.status_updates.get(&account_id)
    }
    //pub fn hello_world(&self) -> &str {
    //return "Hello world";
    //}

    //pub fn hello(&self) -> String {
    //let account_id = env::signer_account_id();
    //return "Hello ".to_owned() + &account_id.to_string();
    //}
    //pagination
    // Retrieves multiple elements from the `UnorderedMap`.
    // - `from_index` is the index to start from.
    // - `limit` is the maximum number of elements to return.
    pub fn get_updates(&self, from_index: u64, limit: u64) -> Vec<(AccountId, String)> {
        let keys = self.status_updates.keys_as_vector();
        let values = self.status_updates.values_as_vector();
        (from_index..std::cmp::min(from_index + limit, self.status_updates.len()))
            .map(|index| (keys.get(index).unwrap(), values.get(index).unwrap()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn hello_world() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Contract {};
        assert_eq!("Hello world", contract.hello_world());
    }

    #[test]
    fn hello() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = Contract {};
        let account_id = env::signer_account_id();
        assert_eq!(
            "Hello ".to_owned() + &account_id.to_string(),
            contract.hello()
        );
    }
}
