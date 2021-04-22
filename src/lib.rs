use near_sdk::borsh;
use near_sdk::borsh::BorshDeserialize;
use near_sdk::borsh::BorshSerialize;
use near_sdk::collections::LookupSet;
use near_sdk::env;
use near_sdk::near_bindgen;
use near_sdk::setup_alloc;
use near_sdk::BorshStorageKey;
use near_sdk::{Balance, Promise};

setup_alloc!();

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Hash,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MergeData {
    hash_set: LookupSet<String>,
}

impl Default for MergeData {
    fn default() -> Self {
        Self {
            hash_set: LookupSet::new(StorageKey::Hash),
        }
    }
}

fn is_empty(data: &[Vec<u8>]) -> bool {
    if data.is_empty() || data.len() < 2 {
        return true;
    }
    for item in data {
        if item.is_empty() {
            return true;
        }
    }
    false
}

#[near_bindgen]
impl MergeData {
    /// ```bash
    /// near view mergeData.YOU.testnet merge {"data": [[102, 111, 111], [102, 111, 111]]}'
    /// ```
    #[payable]
    pub fn merge(&mut self, data: Vec<Vec<u8>>, sort: Option<bool>) -> Vec<u8> {
        let amount = data.len() as Balance;
        if is_empty(&data) {
            env::panic(b"Given empty data")
        }
        let deposit = env::attached_deposit();
        if deposit == 0 {
            env::panic(b"Given empty deposit")
        }
        let mut result = vec![];
        for mut item in data {
            if sort != None {
                item.sort_unstable();
                if sort == Some(false) {
                    item.reverse()
                }
            }
            result.append(&mut item);
        }
        let checksum = hex::encode(env::sha256(&result));
        if self.hash_set.contains(&checksum) {
            return result;
        }
        if !self.hash_set.insert(&checksum) {
            env::panic(b"Failed to save checksum");
        }
        let account_id = env::predecessor_account_id();
        Promise::new(account_id).transfer(amount);
        result
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    use near_sdk::test_utils::accounts;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    fn context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(accounts(0));
        builder
    }

    #[test]
    #[should_panic(expected = "Given empty data")]
    fn merge_empty_data() {
        testing_env!(context().build());
        let mut contract = MergeData::default();
        contract.merge(vec![], None);
    }

    #[test]
    #[should_panic(expected = "Given empty deposit")]
    fn merge_empty_deposit() {
        testing_env!(context().build());
        let mut contract = MergeData::default();
        contract.merge(vec![vec![1], vec![2]], None);
    }

    #[test]
    fn merge_sort_none() {
        testing_env!(context().attached_deposit(1).build());
        let mut contract = MergeData::default();
        let actual = contract.merge(vec![vec![2, 1, 3], vec![4, 3, 5]], None);
        assert_eq!(vec![2, 1, 3, 4, 3, 5], actual);
    }

    #[test]
    fn merge_sort_asc() {
        testing_env!(context().attached_deposit(1).build());
        let mut contract = MergeData::default();
        let actual = contract.merge(vec![vec![2, 1, 3], vec![4, 3, 5]], Some(true));
        assert_eq!(vec![1, 2, 3, 3, 4, 5], actual);
    }

    #[test]
    fn merge_sort_desc() {
        testing_env!(context().attached_deposit(1).build());
        let mut contract = MergeData::default();
        let actual = contract.merge(vec![vec![2, 1, 3], vec![4, 3, 5]], Some(false));
        assert_eq!(vec![3, 2, 1, 5, 4, 3], actual);
    }
}
