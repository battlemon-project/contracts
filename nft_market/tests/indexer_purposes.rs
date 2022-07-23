mod helpers;

use crate::helpers::{MARKET, MARKET_PATH};
use helpers::{NFT, NFT_PATH};
use lemotests::prelude::*;
use lemotests_macro::add_helpers;
use near_sdk::json_types::U128;
use token_metadata_ext::TokenExt;

add_helpers!("./nft_schema.json", "./market_schema.json",);

#[tokio::test]
async fn nft_mint_testnet() -> anyhow::Result<()> {
    let bchain = StateBuilder::testnet()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let [nft, alice] = bchain.string_ids()?;
    println!("{}", nft);
    bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .execute()
        .await?;

    Ok(())
}

#[tokio::test]
async fn market_sale_testnet() -> anyhow::Result<()> {
    let bchain = StateBuilder::testnet()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let [nft, market, alice, _bob] = bchain.string_ids()?;

    println!("nft: {nft}, market: {market}");

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .call_market_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;

    let msg = format!("{{\"price\":\"{}\"}}", Near(5));
    result
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
        .execute()
        .await?;

    Ok(())
}
