use crate::utils::State;
use near_sdk::serde_json::json;
use near_sdk::{AccountId, Gas};
use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS, STORAGE_AMOUNT};
use near_sdk_sim::transaction::ExecutionStatus;
use nft_market::{OfferCondition, SaleCondition};
use test_utils::*;

mod utils;

#[test]
fn list_asks() {
    let (_root, nft, market, _alice) = utils::init();
    let token_metadata = baz_token_metadata_ext();
    let token_id = "1".to_string();
    // mint 1 nft token
    call!(
        nft.user_account,
        nft.mint(token_id.clone(), token_metadata, None),
        deposit = (STORAGE_AMOUNT / 2)
    )
    .assert_success();

    // try to buy token
    let price = json!({
        "price": "1",
    })
    .to_string();
    // simulate frontend call for selling nft token.
    call!(
        nft.user_account,
        nft.nft_approve(token_id.clone(), market.account_id(), Some(price)),
        deposit = (STORAGE_AMOUNT / 2)
    )
    .assert_success();

    let sale_conditions: Vec<SaleCondition> = view!(market.list_asks()).unwrap_json();
    assert_eq!(sale_conditions.len(), 1);
    let sale = sale_conditions.first().unwrap();
    assert_eq!(sale.token_id, token_id);
    assert_eq!(sale.owner_id, nft.account_id());
    assert_eq!(sale.price, 1);
}

#[test]
fn buying() {
    let (root, nft, market, alice) = utils::init();
    let token_id = "some title".to_string();
    let token_metadata = baz_token_metadata_ext();
    // mint 1 nft token to bob
    let bob = root.create_user("bob".parse().unwrap(), to_yocto("100"));
    call!(
        nft.user_account,
        nft.mint(token_id.clone(), token_metadata, Some(bob.account_id())),
        deposit = (STORAGE_AMOUNT / 2)
    )
    .assert_success();

    // try to buy token
    let price = json!({
        "price": to_yocto("10").to_string(),
    })
    .to_string();
    // simulate frontend call for selling nft token.
    call!(
        bob,
        nft.nft_approve(token_id.clone(), market.account_id(), Some(price)),
        deposit = STORAGE_AMOUNT
    )
    .assert_success();

    // simulate buying process from user
    let outcome = call!(
        alice,
        market.buy(token_id.clone()),
        to_yocto("10"),
        DEFAULT_GAS
    );

    outcome.assert_success();

    assert_eq!(outcome.promise_errors().len(), 0);
    println!("outcome status: {:?}", outcome.outcome().status);
    let expected_gas_ceiling = 300 * u64::pow(10, 12);
    assert!(outcome.gas_burnt() < Gas(expected_gas_ceiling));

    // checking that asks is empty
    let sale_conditions: Vec<SaleCondition> = view!(market.list_asks()).unwrap_json();
    assert_eq!(sale_conditions.len(), 0);
    // checking that new owner is Alice
    let owner_id: AccountId = view!(nft.get_owner_by_token_id(token_id.clone())).unwrap_json();
    assert_eq!(owner_id, alice.account_id());
}

#[test]
fn bid_adding_to_state_successful() {
    let (root, nft, market, alice) = utils::init();
    let token_id = "some title".to_string();
    let token_metadata = baz_token_metadata_ext();
    // mint 1 nft token to bob
    let bob = root.create_user("bob".parse().unwrap(), to_yocto("100"));
    call!(
        nft.user_account,
        nft.mint(token_id.clone(), token_metadata, Some(bob.account_id())),
        deposit = (STORAGE_AMOUNT / 2)
    )
    .assert_success();
    call!(alice, market.bid(token_id.clone()), deposit = 100).assert_success();

    let mut conditions: Vec<OfferCondition> = view!(market.list_bids()).unwrap_json();
    assert_eq!(conditions.len(), 1);

    let condition = conditions.pop().unwrap();
    assert_eq!(condition.token_id, token_id);
    assert_eq!(condition.bidder_id, alice.account_id());
    assert_eq!(condition.price, 100);
}

#[test]
fn bid_failure_nft_token_must_refund_attached_deposit() {
    let (root, _spoiled_nft, market) = utils::init_spoiled();
    let alice = root.create_user("alice".parse().unwrap(), to_yocto("10"));
    let token_id = "some_title".to_string();
    let initial_amount = alice.get_amount();
    let result = call!(alice, market.bid(token_id.clone()), deposit = to_yocto("5"));
    let actual_amount = alice.get_amount();
    let diff = initial_amount - actual_amount;
    // the gas fee for execution tx smaller than 1, so the balance the same before and after
    assert!(diff < to_yocto("1"));

    // promise results must contain errors
    let promise_results = result.promise_results();
    let panic_msg = "this is spoiled `nft_token` method.";
    let failures = promise_results
        .clone()
        .into_iter()
        .filter_map(|r| match r.unwrap().status() {
            ExecutionStatus::Failure(e) if e.to_string().contains(panic_msg) => Some(e),
            _ => None,
        })
        .count();
    assert_eq!(failures, 1);

    // promise results must contain log about refund
    let logs = promise_results
        .into_iter()
        .map(|r| r.unwrap().logs().to_owned())
        .flatten()
        .collect::<Vec<_>>();

    let err_log =
        "`nft_token` execution error was occurred, attached deposit was returned".to_string();
    assert!(logs.contains(&err_log));
}
