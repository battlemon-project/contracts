use anyhow::Context;
use near_sdk::serde_json::json;
use test_helpers::workspaces::prelude::*;
use test_helpers::*;

#[tokio::test]
async fn new_api_works() -> anyhow::Result<()> {
    let nft_wasm = tokio::fs::read("../target/wasm32-unknown-unknown/release/nft_token.wasm")
        .await
        .context("Failed to load nft wasm")?;
    let worker = workspaces::testnet().await?;
    let root = worker.dev_create_account().await?;
    let nft = root
        .create_subaccount(&worker, "nft")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    let nft_contract = nft.deploy(&worker, &nft_wasm).await?.into_result()?;

    let result = nft_contract
        .call(&worker, "init")
        .args_json(json!({"owner_id": nft_contract.id()}))?
        .transact()
        .await?;
    assert!(result.is_success());

    let alice = worker.dev_create_account().await?;
    let result = alice
        .call(&worker, nft_contract.id(), "nft_mint")
        .deposit(parse_near!("1 N"))
        .args_json(json!({"receiver_id": alice.id()}))?
        .transact()
        .await?;

    assert!(result.is_success());

    let market = root
        .create_subaccount(&worker, "market")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    let market_wasm = tokio::fs::read("../target/wasm32-unknown-unknown/release/nft_market.wasm")
        .await
        .context("Failed to load market wasm")?;
    let market_contract = market.deploy(&worker, &market_wasm).await?.into_result()?;
    alice
        .call(&worker, nft_contract.id(), "nft_approve")
        .deposit(parse_near!("1 N"))
        .max_gas()
        .args_json(json!({
            "token_id": "1",
            "account_id": market.id(),
        }))?
        .transact()
        .await?;

    market_contract
        .call(&worker, "init")
        .args_json(json!({"nft_id": nft_contract.id()}))?
        .transact()
        .await?;

    let result = market_contract
        .call(&worker, "test")
        .max_gas()
        .deposit(parse_near!("1 N"))
        .transact()
        .await?;

    assert!(result.is_success());
    println!("{:#?}", result.outcomes());
    Ok(())
}
