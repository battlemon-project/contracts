use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{AccountId, Gas};
use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS, STORAGE_AMOUNT};
use nft_market::{OfferCondition, SaleCondition};
use test_utils::*;
use test_utils::{INVALID_TOKEN_ID, VALID_TOKEN_ID, VALID_TOKEN_PRICE};
use token_metadata_ext::TokenExt;
use utils::{PromiseResultUtils, State};

mod utils;

#[test]
fn list_asks() {
    // todo: add more test here
    let (_root, nft, market, _alice) = utils::init_mint_approve();

    let sale_conditions: Vec<SaleCondition> = view!(market.list_asks()).unwrap_json();
    assert_eq!(sale_conditions.len(), 1);
    let sale = sale_conditions.first().unwrap();
    assert_eq!(sale.token_id, VALID_TOKEN_ID.to_string());
    assert_eq!(sale.owner_id, nft.account_id());
    assert_eq!(sale.price.0, *VALID_TOKEN_PRICE);
}
