use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use token_metadata_ext::TokenExt;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract;

#[near_bindgen]
impl Contract {
    #[allow(unused_variables)]
    pub fn nft_token(&self, token_id: TokenId) -> Option<TokenExt> {
        env::panic_str("this is spoiled `nft_token` method.");
    }
}
