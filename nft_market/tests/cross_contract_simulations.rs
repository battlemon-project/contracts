use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{AccountId, Gas};
use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS, STORAGE_AMOUNT};
use nft_market::{SaleCondition, NO_DEPOSIT};
use utils::ONE_YOCTO;

mod utils;

#[test]
fn list_asks() {
    let (root, nft, market, _alice) = utils::init();
    let token_id = "some title".to_string();
    let token_metadata = TokenMetadata {
        title: Some(token_id.clone()),
        description: Some("Here some description".to_string()),
        media: None,
        media_hash: None,
        copies: None,
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: Some("Here can be extra json".to_string()),
        reference: None,
        reference_hash: None,
    };

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
    // simulate frontend's call for selling nft token.
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
    assert_eq!(sale.price, U128(1));
}

#[test]
fn buying() {
    let (root, nft, market, alice) = utils::init();
    let token_id = "some title".to_string();
    let token_metadata = TokenMetadata {
        title: Some(token_id.clone()),
        description: Some("Here some description".to_string()),
        media: None,
        media_hash: None,
        copies: None,
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: Some("Here can be extra json".to_string()),
        reference: None,
        reference_hash: None,
    };
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
    // simulate frontend's call for selling nft token.
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
