use crate::{Ask, Bid};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{AccountId, Promise, PromiseError, PromiseOrValue};

#[near_sdk::ext_contract(nft)]
pub trait Nft {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> Promise;

    fn nft_token(&self, token_id: TokenId) -> Promise;
}
//
// #[near_sdk::ext_contract]
// trait ExtSelf {
//     fn after_nft_transfer_for_ask(&mut self, sale: Ask, buyer_id: AccountId) -> Promise;
//     fn after_nft_transfer_for_bid(&mut self, sale: Bid, owner_id: AccountId) -> PromiseOrValue<()>;
//
//     fn after_nft_token(
//         &mut self,
//         bidder_id: AccountId,
//         token_id: TokenId,
//         #[rustfmt::skip]
//         #[callback_result]
//         result: Result<Option<token_metadata_ext::TokenExt>, PromiseError>,
//     ) -> PromiseOrValue<()>;
// }
