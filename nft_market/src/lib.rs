use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct Contract {
    val: u64,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self { val: 10 }
    }

    pub fn increment(&mut self) {
        self.val += 1;
    }

    pub fn get_val(&self) -> u64 {
        near_sdk::log!("get val log");
        self.val
    }
}
