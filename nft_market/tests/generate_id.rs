mod helpers;

use helpers::{MARKET, MARKET_PATH, NFT};
use lemotests::workspaces::operations::Function;
use lemotests::{Gasable, Near, StateBuilder, Tgas};
use lemotests_macro::add_helpers;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use nft_market::Bid;

add_helpers!("./nft_schema.json", "./market_schema.json",);

#[tokio::test]
async fn create_ten_bids_all_unique_ids() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let [market, _alice] = bchain.string_ids()?;
    let result = bchain
        .call_market_contract_init(NFT)?
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;

    let result = result
        .into_state()
        .alice_call_market_contract_storage_deposit(Some(&market))?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit * 10)
        .execute()
        .await?;

    let method = Function::new("add_bid")
        .args_json(json!({
            "token_id": "1",
        }))?
        .deposit(0)
        .gas(Tgas(10).parse());

    let state = result.into_state();

    state
        .contract(MARKET)?
        .batch(state.worker())
        .call(method.clone())
        .call(method.clone())
        .call(method.clone())
        .call(method.clone())
        .call(method.clone())
        .call(method.clone())
        .call(method.clone())
        .call(method.clone())
        .call(method.clone())
        .call(method.clone())
        .transact()
        .await?;

    let result = state
        .view_market_contract_bids("1")?
        .with_label("view_bids")
        .execute()
        .await?;

    let bids = result.tx("view_bids")?.json::<Vec<Bid>>()?;
    dbg!(bids);
    Ok(())
}
