mod helpers;

use battlemon_models::{market::ask::AskForContract, nft::NftKind};
use helpers::{MARKET, MARKET_PATH, NFT, NFT_PATH};
use lemotests::prelude::*;
use lemotests::Nearable;
use lemotests_macro::add_helpers;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::json_types::U128;

add_helpers!("./nft_schema.json", "./market_schema.json",);

#[tokio::test]
async fn ask_works_for_storage_deposit_made_by_alice_for_bob() -> anyhow::Result<()> {
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
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let msg = format!("{{\"price\":\"{}\"}}", Near(5));
    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;
    let result = result
        .into_state()
        .alice_call_market_contract_storage_deposit(Some(&bob))?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .bob_call_nft_contract_nft_mint(&bob, NftKind::Lemon)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .bob_call_nft_contract_nft_approve("1", &market, Some(&msg))?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_ask("1")?
        .with_label("asks")
        .execute()
        .await?;

    let ask: AskForContract = result.tx("asks")?.json()?;
    assert_eq!(ask.price.0, Near(5).parse());
    assert_eq!(ask.account_id.to_string(), bob);

    Ok(())
}
