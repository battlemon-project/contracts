mod helpers;

use battlemon_models::market::{ask::AskForContract, bid::BidForContract};
use battlemon_models::nft::{NftKind, TokenExt};
use helpers::{MARKET, MARKET_PATH, NFT, NFT_PATH};
use lemotests::prelude::*;
use lemotests_macro::add_helpers;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::json_types::U128;

add_helpers!("./nft_schema.json", "./market_schema.json",);

#[tokio::test]
async fn alice_bid_for_token_5_near_then_balance_is_changed_and_bid_in_bids() -> anyhow::Result<()>
{
    let bchain = StateBuilder::sandbox()
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let [_market, alice] = bchain.string_ids()?;

    let result = bchain
        .call_market_contract_init(NFT)?
        .with_gas(Tgas(100))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;

    let result = result
        .into_state()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .alice_call_market_contract_add_bid("1", None)?
        .with_deposit(Near(5))
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_bids("1")?
        .with_label("view_bids")
        .then()
        .view_account(ALICE)?
        .with_label("alice_balance")
        .execute()
        .await?;

    let bids = result.tx("view_bids")?.json::<Vec<BidForContract>>()?;
    assert_eq!(bids.len(), 1);
    let bid = &bids[0];
    assert_eq!(bid.account_id().as_str(), &alice);
    assert_eq!(bid.token_id(), "1");
    assert_eq!(bid.price(), Near(5));
    let alice_balance = result.tx("alice_balance")?.balance();
    assert!((Near(5) - alice_balance) <= ALMOST_ZERO);

    Ok(())
}

#[tokio::test]
async fn alice_ask_for_nft_token_five_bob_bid_six_alice_receive_five_bob_receive_nft_token_and_change_one(
) -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let [nft, market, alice, bob] = bchain.string_ids()?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .call_market_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;

    let msg = format!("{{\"price\":\"{}\"}}", Near(5));
    let result = result
        .into_state()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .alice_call_nft_contract_nft_approve("1", &market, Some(&msg))?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .bob_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .bob_call_market_contract_add_bid("1", None)?
        .with_deposit(Near(6))
        .with_gas(Tgas(200))
        .then()
        .view_nft_contract_nft_token("1")?
        .with_label("view_nft_token")
        .then()
        .view_account(ALICE)?
        .with_label("view_alice")
        .then()
        .view_account(BOB)?
        .with_label("view_bob")
        .execute()
        .await?;

    let nft_token = result.tx("view_nft_token")?.json::<TokenExt>()?;
    assert_eq!(nft_token.owner_id.as_str(), bob.as_str());

    let alice_balance = result.tx("view_alice")?.balance();
    let bob_balance = result.tx("view_bob")?.balance();

    // bob must have balance ~5N
    let diff = Near(5) - bob_balance;
    assert!(
        diff <= ALMOST_ZERO,
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        bob_balance
    );
    // alice must receive 5N
    let diff = Near(15) - alice_balance;
    assert!(
        diff <= ALMOST_ZERO,
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        alice_balance
    );

    Ok(())
}

#[tokio::test]
async fn bob_bid_six_alice_ask_for_nft_token_five_alice_receive_six_bob_receive_nft_token_and_no_change(
) -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let [nft, market, alice, bob] = bchain.string_ids()?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .call_market_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;

    let msg = format!("{{\"price\":\"{}\"}}", Near(5));
    let result = result
        .into_state()
        .bob_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .bob_call_market_contract_add_bid("1", None)?
        .with_deposit(Near(6))
        .with_gas(Tgas(10))
        .then()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .alice_call_nft_contract_nft_approve("1", &market, Some(&msg))?
        .with_deposit(Near(5))
        .with_gas(Tgas(50))
        .then()
        .view_nft_contract_nft_token("1")?
        .with_label("view_nft_token")
        .then()
        .view_account(ALICE)?
        .with_label("view_alice")
        .then()
        .view_account(BOB)?
        .with_label("view_bob")
        .execute()
        .await?;

    let nft_token = result.tx("view_nft_token")?.json::<TokenExt>()?;
    assert_eq!(nft_token.owner_id.as_str(), bob.as_str());

    let alice_balance = result.tx("view_alice")?.balance();
    let bob_balance = result.tx("view_bob")?.balance();

    // bob must have balance <=4N
    let diff = Near(4) - bob_balance;
    assert!(
        diff <= ALMOST_ZERO,
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        bob_balance
    );
    // alice must receive 6N
    let diff = Near(16) - alice_balance;
    assert!(
        diff <= ALMOST_ZERO,
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        alice_balance
    );

    Ok(())
}

#[tokio::test]
async fn bid_first_ask_second_then_trade_then_bid_is_removed_and_ask_does_not_created(
) -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .build()
        .await?;

    let [nft, market, alice, _] = bchain.string_ids()?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .call_market_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;

    let msg = format!("{{\"price\":\"{}\"}}", Near(5));
    let result = result
        .into_state()
        .bob_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .bob_call_market_contract_add_bid("1", None)?
        .with_deposit(Near(5))
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_bids("1")?
        .with_label("bids_before")
        .then()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .alice_call_nft_contract_nft_approve("1", &market, Some(&msg))?
        .with_deposit(Near(5))
        .with_gas(Tgas(50))
        .then()
        .view_market_contract_bids("1")?
        .with_label("bids_after")
        .then()
        .view_market_contract_ask("1")?
        .with_label("ask")
        .execute()
        .await?;

    let bids_before: Option<Vec<BidForContract>> = result.tx("bids_before")?.json()?;
    assert_eq!(bids_before.unwrap().len(), 1);
    let bids_after: Option<Vec<BidForContract>> = result.tx("bids_after")?.json()?;
    assert_eq!(bids_after, None);
    let ask: Option<AskForContract> = result.tx("ask")?.json()?;
    assert_eq!(ask, None);

    Ok(())
}

#[tokio::test]
async fn ask_first_bid_second_then_trade_then_ask_is_removed_and_bid_does_not_created(
) -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .build()
        .await?;

    let [nft, market, alice, _] = bchain.string_ids()?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .call_market_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;

    let msg = format!("{{\"price\":\"{}\"}}", Near(5));
    let result = result
        .into_state()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .alice_call_nft_contract_nft_approve("1", &market, Some(&msg))?
        .with_deposit(Near(5))
        .with_gas(Tgas(50))
        .then()
        .view_market_contract_ask("1")?
        .with_label("ask_before")
        .then()
        .bob_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .bob_call_market_contract_add_bid("1", None)?
        .with_deposit(Near(5))
        .with_gas(Tgas(40))
        .then()
        .view_market_contract_ask("1")?
        .with_label("ask_after")
        .then()
        .view_market_contract_bids("1")?
        .with_label("bids")
        .execute()
        .await?;

    let ask_before: Option<AskForContract> = result.tx("ask_before")?.json()?;
    assert!(ask_before.is_some());
    let ask_after: Option<AskForContract> = result.tx("ask_after")?.json()?;
    assert!(ask_after.is_none());
    let bids: Option<Vec<BidForContract>> = result.tx("bids")?.json()?;
    assert!(bids.is_none());

    Ok(())
}
