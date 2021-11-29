use crate::utils::{PromiseResultUtils, State, INVALID_TOKEN_ID, TOKEN_PRICE, VALID_TOKEN_ID};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{AccountId, Gas};
use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS, STORAGE_AMOUNT};
use nft_market::{OfferCondition, SaleCondition};
use test_utils::*;
use token_metadata_ext::TokenExt;

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
    assert_eq!(sale.price.0, *TOKEN_PRICE);
}

#[test]
fn buying() {
    // todo: add more test here
    let (_root, nft, market, alice) = utils::init_mint_approve();

    // simulate buying process from user
    let exec_results = call!(
        alice,
        market.buy(VALID_TOKEN_ID.to_string()),
        *TOKEN_PRICE,
        DEFAULT_GAS
    );

    exec_results.assert_success();

    assert_eq!(exec_results.promise_errors().len(), 0);
    let expected_gas_ceiling = 300 * u64::pow(10, 12);
    assert!(exec_results.gas_burnt() < Gas(expected_gas_ceiling));

    // checking that asks is empty
    let sale_conditions: Vec<SaleCondition> = view!(market.list_asks()).unwrap_json();
    assert_eq!(sale_conditions.len(), 0);
    // checking that new owner is Alice
    let owner_id: AccountId =
        view!(nft.get_owner_by_token_id(VALID_TOKEN_ID.to_string())).unwrap_json();
    assert_eq!(owner_id, alice.account_id());
}

#[test]
fn bid_adding_to_state_successful() {
    let (root, nft, market, alice) = utils::init();
    let token_metadata = baz_token_metadata_ext();
    // mint 1 nft token to bob
    let bob = root.create_user("bob".parse().unwrap(), to_yocto("100"));
    call!(
        nft.user_account,
        nft.mint(
            VALID_TOKEN_ID.to_string(),
            token_metadata,
            Some(bob.account_id())
        ),
        deposit = (STORAGE_AMOUNT / 2)
    )
    .assert_success();
    call!(
        alice,
        market.bid(VALID_TOKEN_ID.to_string()),
        deposit = *TOKEN_PRICE
    )
    .assert_success();

    let bids: Vec<(TokenId, Vec<OfferCondition>)> = view!(market.list_bids()).unwrap_json();
    let (_, conditions) = bids.last().unwrap();
    assert_eq!(conditions.len(), 1);

    let condition = conditions.last().unwrap();
    assert_eq!(condition.token_id, VALID_TOKEN_ID.to_string());
    assert_eq!(condition.bidder_id, alice.account_id());
    assert_eq!(condition.price.0, *TOKEN_PRICE);
}

#[test]
fn bid_failure_nft_token_method_panic_than_must_refund_attached_deposit() {
    let (_root, _spoiled_nft, market, alice) = utils::init_spoiled();
    let initial_amount = alice.get_amount();
    let execution_result = call!(
        alice,
        market.bid(VALID_TOKEN_ID.to_string()),
        deposit = *TOKEN_PRICE
    );
    let actual_amount = alice.get_amount();
    let diff = initial_amount - actual_amount;
    // the gas fee for execution tx smaller than 1, so the balance the same before and after
    assert!(diff < to_yocto("1"));

    // promise results must contain particular error
    assert_eq!(execution_result.promise_errors().len(), 1);
    let panicked_with_msg = execution_result
        .promise_results()
        .contains_panic_message("this is spoiled `nft_token` method.");
    assert!(panicked_with_msg);

    // promise results must contain log about refund
    let with_log = execution_result
        .promise_results()
        .contains_log("`nft_token` execution error was occurred, attached deposit was returned");
    assert!(with_log);
}

    let err_log =
        "`nft_token` execution error was occurred, attached deposit was returned".to_string();
    assert!(logs.contains(&err_log));
}
