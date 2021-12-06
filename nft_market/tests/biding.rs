use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::serde_json::json;
use near_sdk::{AccountId, Gas};
use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS, STORAGE_AMOUNT};
use nft_market::{OfferCondition, SaleCondition, SaleType};
use once_cell::unsync::Lazy;
use test_utils::baz_token_metadata_ext;
use test_utils::{INVALID_TOKEN_ID, INVALID_TOKEN_PRICE, VALID_TOKEN_ID, VALID_TOKEN_PRICE};
use token_metadata_ext::TokenExt;
use utils::{PromiseResultUtils, State};

mod utils;

const IMAGINED_MAX_FEE: Lazy<u128> = Lazy::new(|| to_yocto("0.1"));

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
        deposit = *VALID_TOKEN_PRICE
    )
    .assert_success();

    let bids: Vec<(TokenId, Vec<OfferCondition>)> = view!(market.list_bids()).unwrap_json();
    let (_, conditions) = bids.last().unwrap();
    assert_eq!(conditions.len(), 1);

    let condition = conditions.last().unwrap();
    assert_eq!(condition.token_id, VALID_TOKEN_ID.to_string());
    assert_eq!(condition.bidder_id, alice.account_id());
    assert_eq!(condition.price.0, *VALID_TOKEN_PRICE);
}

#[test]
fn bid_failure_nft_token_method_panic_then_must_refund_attached_deposit() {
    let (_root, _spoiled_nft, market, alice) = utils::init_spoiled();
    let initial_amount = alice.get_amount();
    let execution_result = call!(
        alice,
        market.bid(VALID_TOKEN_ID.to_string()),
        deposit = *VALID_TOKEN_PRICE
    );
    let actual_amount = alice.get_amount();
    let diff = initial_amount - actual_amount;
    // the gas fee for execution tx smaller than 0.1, so the balance the same before and after
    assert!(diff < *IMAGINED_MAX_FEE);

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

#[test]
fn bid_failure_token_with_provided_id_do_not_exists_must_refund_attached_deposit() {
    let (_root, _nft, market, alice) = utils::init_mint();
    let initial_amount = alice.get_amount();
    let execution_result = call!(
        alice,
        market.bid(INVALID_TOKEN_ID.to_string()),
        deposit = to_yocto("5")
    );
    let actual_amount = alice.get_amount();
    let diff = initial_amount - actual_amount;

    // the gas fee for execution tx smaller than 0.1 near, so the balance the same before and after
    assert!(diff < *IMAGINED_MAX_FEE);
    assert_eq!(execution_result.promise_errors().len(), 0);

    let log = format!(
        "token with id: {} doesn't exist, attached deposit was returned",
        INVALID_TOKEN_ID
    );
    let with_log = execution_result.promise_results().contains_log(&log);
    assert!(with_log);
}

#[test]
fn bid_with_amount_bigger_than_ask_price_must_refund_diff() {
    let (_root, nft, market, alice) = utils::init_mint_approve();
    let nft_initial_balance = nft.user_account.get_amount();
    let alice_initial_balance = alice.get_amount();
    let double_price = *VALID_TOKEN_PRICE * 2;
    let execution_result = call!(
        alice,
        market.bid(VALID_TOKEN_ID.to_string()),
        deposit = double_price
    );

    assert_eq!(execution_result.promise_errors().len(), 0);

    let alice_actual_balance = alice.get_amount();
    let nft_actual_balance = nft.user_account.get_amount();

    // actual now is less than initial
    let alice_diff = alice_initial_balance - alice_actual_balance;
    let fee = alice_diff - *VALID_TOKEN_PRICE;
    // the gas fee for execution tx smaller than 0.1 near, so the balance the same before and after
    assert!(
        fee < *IMAGINED_MAX_FEE,
        "alice_diff - token_price: {} must be less than {}",
        fee,
        *IMAGINED_MAX_FEE
    );
    // actual now more than initial
    let nft_diff = nft_actual_balance - nft_initial_balance;
    let fee = nft_diff - *VALID_TOKEN_PRICE;
    assert!(fee < *IMAGINED_MAX_FEE);

    // alice must be new owner of token
    let nft_token: Option<TokenExt> =
        view!(nft.nft_token(VALID_TOKEN_ID.to_string())).unwrap_json();
    assert_eq!(nft_token.unwrap().owner_id, alice.account_id());

    let asks: Vec<SaleCondition> = view!(market.list_asks()).unwrap_json();
    assert_eq!(asks.len(), 0);

    let bids: Vec<(TokenId, Vec<OfferCondition>)> = view!(market.list_bids()).unwrap_json();
    assert_eq!(bids.len(), 0);
}

#[test]
fn bid_successful_and_equals_to_ask_with_same_token_id() {
    let (root, nft, market, alice, bob) = utils::init_mint_to_alice_approve();
    let alice_initial_balance = alice.get_amount();
    let execution_result = call!(
        bob,
        market.bid(VALID_TOKEN_ID.to_string()),
        deposit = *VALID_TOKEN_PRICE
    );

    assert_eq!(execution_result.promise_errors().len(), 0);

    let alice_actual_balance = alice.get_amount();

    // actual now is less than initial
    let alice_diff = alice_actual_balance - alice_initial_balance;
    let fee = alice_diff - *VALID_TOKEN_PRICE;
    // the gas fee for execution tx smaller than 0.1 near, so the balance the same before and after
    assert!(
        fee < *IMAGINED_MAX_FEE,
        "alice_diff - token_price: {} must be less than {}",
        fee,
        *IMAGINED_MAX_FEE
    );

    // bot must be new owner of token
    let nft_token: Option<TokenExt> =
        view!(nft.nft_token(VALID_TOKEN_ID.to_string())).unwrap_json();
    assert_eq!(nft_token.unwrap().owner_id, bob.account_id());

    let asks: Vec<SaleCondition> = view!(market.list_asks()).unwrap_json();
    assert_eq!(asks.len(), 0);

    let bids: Vec<(TokenId, Vec<OfferCondition>)> = view!(market.list_bids()).unwrap_json();
    assert_eq!(bids.len(), 0);
}

#[test]
fn accept_bid_successful() {
    let (_root, nft, market, alice, bob) = utils::init_mint_to_alice();
    call!(
        bob,
        market.bid(VALID_TOKEN_ID.to_string()),
        deposit = *VALID_TOKEN_PRICE
    )
    .assert_success();

    let msg = json!({
        "sale_type": "accept_bid",
        "price": "0"
    })
    .to_string();

    let alice_initial_balance = alice.get_amount();
    let execution_result = call!(
        alice,
        nft.nft_approve(VALID_TOKEN_ID.to_string(), market.account_id(), Some(msg)),
        deposit = 180000000000000000000
    );
    assert_eq!(execution_result.promise_errors().len(), 0);
    let alice_actual_balance = alice.get_amount();

    let alice_diff = alice_actual_balance - alice_initial_balance;
    // because we also pay for storing approval_id
    let fee = *VALID_TOKEN_PRICE - alice_diff;
    // the gas fee for execution tx smaller than 0.1 near, so the balance the same before and after
    assert!(
        fee < *IMAGINED_MAX_FEE,
        "alice_diff - token_price: {} must be less than {}",
        fee,
        *IMAGINED_MAX_FEE
    );

    let nft_token: Option<TokenExt> =
        view!(nft.nft_token(VALID_TOKEN_ID.to_string())).unwrap_json();
    assert_eq!(nft_token.unwrap().owner_id, bob.account_id());

    let bids: Vec<(TokenId, Vec<OfferCondition>)> = view!(market.list_bids()).unwrap_json();
    assert_eq!(bids.len(), 1);
    assert_eq!(bids[0].1.len(), 0);
}
