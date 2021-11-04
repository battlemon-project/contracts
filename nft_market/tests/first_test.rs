use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::Gas;
use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS};
use nft_market::SaleCondition;
use utils::ONE_YOCTO;

mod utils;

#[test]
fn list_asks() {
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

    // mint 1 nft token
    call!(
        root,
        nft.mint(token_id.clone(), token_metadata),
        to_yocto("1"),
        DEFAULT_GAS
    )
    .assert_success();

    // try to buy token
    let price = json!({
        "price": "1",
    })
    .to_string();
    // simulate frontend's call for selling nft token.
    call!(
        root,
        nft.nft_approve(token_id.clone(), market.account_id(), Some(price)),
        deposit = 180000000000000000000
    )
    .assert_success();

    let sale_conditions: Vec<SaleCondition> = view!(market.list_asks()).unwrap_json();
    assert_eq!(sale_conditions.len(), 1);
    let sale = sale_conditions.first().unwrap();
    assert_eq!(sale.token_id, token_id);
    assert_eq!(sale.owner_id, root.account_id());
    assert_eq!(sale.price, U128(1));
}
