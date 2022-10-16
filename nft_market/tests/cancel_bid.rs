mod helpers;

use battlemon_models::market::{ask::AskForContract, bid::BidForContract};
use battlemon_models::nft::NftKind;
use helpers::{MARKET, MARKET_PATH};
use lemotests::prelude::*;
use lemotests_macro::add_helpers;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::json_types::U128;

add_helpers!("./nft_schema.json", "./market_schema.json",);

#[tokio::test]
async fn cancel_bid_works() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_alice(Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .build()
        .await?;

    let [nft, _alice] = bchain.string_ids()?;

    let result = bchain
        .call_market_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;
    let token_id = TokenId::from("1");
    let result = result
        .into_state()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .alice_call_market_contract_add_bid(token_id.as_str(), None)?
        .with_deposit(Near(5))
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_bids(token_id.as_str())?
        .with_label("bids_before")
        .execute()
        .await?;

    let bids_before: Vec<BidForContract> = result.tx("bids_before")?.json()?;
    let bid_id = bids_before
        .get(0)
        .expect("alice's bids are empty")
        .id
        .clone();

    let result = result
        .into_state()
        .alice_call_market_contract_cancel_bid(&token_id, &bid_id)?
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_bids(token_id.as_str())?
        .with_label("bids_after")
        .execute()
        .await?;

    let bids_after: Option<Vec<AskForContract>> = result.tx("bids_after")?.json()?;
    assert!(bids_after.is_none());

    Ok(())
}

#[tokio::test]
async fn cancel_bids_works_only_for_owner() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .build()
        .await?;

    let [nft, _alice, _bob] = bchain.string_ids()?;

    let result = bchain
        .call_market_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;
    let token_id = TokenId::from("1");
    let result = result
        .into_state()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .alice_call_market_contract_add_bid(token_id.as_str(), None)?
        .with_deposit(Near(5))
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_bids(token_id.as_str())?
        .with_label("bids_before")
        .execute()
        .await?;

    let bids_before: Vec<BidForContract> = result.tx("bids_before")?.json()?;
    let bid_id = bids_before
        .get(0)
        .expect("alice's bids are empty")
        .id
        .clone();

    let result = result
        .into_state()
        .bob_call_market_contract_cancel_bid(&token_id, &bid_id)?
        .with_gas(Tgas(10))
        .execute()
        .await;

    assert!(result.contains_error("Ask's owner is not the same as the caller"));

    Ok(())
}
