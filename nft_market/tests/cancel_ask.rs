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
async fn cancel_ask_works() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_alice(Near(10))?
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .build()
        .await?;

    let [nft, market, alice] = bchain.string_ids()?;

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
        .then()
        .view_nft_contract_nft_tokens_for_owner(&alice)?
        .with_label("alice_tokens")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;
    let alice_tokens: Vec<TokenExt> = result.tx("alice_tokens")?.json()?;
    let alice_token_id = alice_tokens
        .get(0)
        .expect("alice tokens are empty")
        .token_id
        .clone();

    let msg = format!("{{\"price\":\"{}\"}}", Near(5));
    let result = result
        .into_state()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .alice_call_nft_contract_nft_approve(alice_token_id.as_str(), &market, Some(&msg))?
        .with_deposit(Near(5))
        .with_gas(Tgas(50))
        .then()
        .view_market_contract_ask(&alice_token_id)?
        .with_label("ask_before")
        .then()
        .alice_call_market_contract_cancel_ask(&alice_token_id)?
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_ask(&alice_token_id)?
        .with_label("ask_after")
        .execute()
        .await?;

    let ask_before: Option<AskForContract> = result.tx("ask_before")?.json()?;
    assert!(ask_before.is_some());
    let ask_after: Option<AskForContract> = result.tx("ask_after")?.json()?;
    assert!(ask_after.is_none());

    Ok(())
}

#[tokio::test]
async fn cancel_ask_works_only_for_owner() -> anyhow::Result<()> {
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
        .then()
        .view_nft_contract_nft_tokens_for_owner(&alice)?
        .with_label("alice_tokens")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;
    let alice_tokens: Vec<TokenExt> = result.tx("alice_tokens")?.json()?;
    let alice_token_id = alice_tokens
        .get(0)
        .expect("alice tokens are empty")
        .token_id
        .clone();

    let msg = format!("{{\"price\":\"{}\"}}", Near(5));
    let result = result
        .into_state()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .alice_call_nft_contract_nft_approve(alice_token_id.as_str(), &market, Some(&msg))?
        .with_deposit(Near(5))
        .with_gas(Tgas(50))
        .then()
        .view_market_contract_ask(&alice_token_id)?
        .with_label("ask_before")
        .then()
        .bob_call_market_contract_cancel_ask(&alice_token_id)?
        .with_gas(Tgas(10))
        .execute()
        .await;

    assert!(result.contains_error("Ask's owner is not the same as the caller"));

    Ok(())
}
