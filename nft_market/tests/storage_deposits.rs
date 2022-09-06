mod helpers;

use battlemon_models::nft::NftKind;
use helpers::{MARKET, MARKET_PATH, NFT, NFT_PATH};
use lemotests::prelude::*;
use lemotests_macro::add_helpers;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::json_types::U128;

add_helpers!("./nft_schema.json", "./market_schema.json",);

#[tokio::test]
async fn remove_deposits_works_without_stored_data_correctly() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let result = bchain
        .call_market_contract_init(NFT)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(Near(5))
        .with_label("storage_deposit")
        .then()
        .view_account(ALICE)?
        .with_label("alice_balance_before")
        .then()
        .alice_call_market_contract_storage_withdraw()?
        .with_gas(Tgas(10))
        .with_deposit(1)
        .then()
        .view_account(ALICE)?
        .with_label("alice_balance_after")
        .execute()
        .await?;

    let alice_balance_before = result.tx("alice_balance_before")?.balance();
    assert!((Near(5) - alice_balance_before) <= ALMOST_ZERO);

    let alice_balance_after = result.tx("alice_balance_after")?.balance();
    assert!((Near(10) - alice_balance_after) <= ALMOST_ZERO);

    Ok(())
}

#[tokio::test]
async fn remove_deposits_works_with_stored_one_bid_works_correctly() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let result = bchain
        .call_market_contract_init(NFT)?
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .execute()
        .await?;

    let minimum_storage_balance: U128 = result[1].json()?;

    let result = result
        .into_state()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(minimum_storage_balance.0)
        .then()
        .alice_call_market_contract_add_bid("1", None)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_market_contract_storage_withdraw()?
        .with_gas(Tgas(10))
        .with_deposit(1)
        .with_label("storage_withdraw")
        .execute()
        .await?;

    let actual_withdraw: U128 = result.tx("storage_withdraw")?.json()?;

    assert_eq!(actual_withdraw.0, 0);

    Ok(())
}

#[tokio::test]
async fn ask_rejected_without_storage_deposit() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let [nft, market, alice] = bchain.string_ids()?;
    let msg = format!("{{\"price\":\"{}\"}}", Near(5));

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
        .alice_call_nft_contract_nft_approve("1", &market, Some(&msg))?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .execute()
        .await;

    assert!(result.contains_error("Not enough storage deposits to create new order"));

    Ok(())
}

#[tokio::test]
async fn bids_rejected_without_storage_deposit() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let result = bchain
        .call_market_contract_init(NFT)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_market_contract_add_bid("1", None)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .execute()
        .await;

    assert!(result.contains_error("Not enough storage deposits to create new order"));

    Ok(())
}
