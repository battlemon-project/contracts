use serde_json::json;
use test_helpers::{get_nft_wasm, workspaces, workspaces::prelude::*};
const CONTRACT_WASM: &str = "../target/wasm32-unknown-unknown/release/nft_token.wasm";

#[tokio::test]
async fn contract_is_initable() -> anyhow::Result<()> {
    let wasm = get_nft_wasm("./").await;
    let worker = workspaces::testnet().await?;
    let account = worker.dev_create_account().await?;
    let contract = account.deploy(&worker, wasm).await?.into_result()?;
    let result = contract
        .call(&worker, "init")
        .args_json(json!({"owner_id": contract.id()}))?
        .transact()
        .await?;

    assert!(result.is_success());

    Ok(())
}

#[tokio::test]
async fn contract_is_initable_by_any_account() -> anyhow::Result<()> {
    let wasm = get_nft_wasm("./").await;
    let worker = workspaces::testnet().await?;
    let alice = worker.dev_create_account().await?;
    let contract = alice.deploy(&worker, wasm).await?.into_result()?;
    let bob = worker.dev_create_account().await?;

    let result = bob
        .call(&worker, contract.id(), "init")
        .args_json(json!({"owner_id": bob.id()}))?
        .transact()
        .await?;

    assert!(result.is_success());

    Ok(())
}

#[tokio::test]
async fn double_initialization_contract_rejected() -> anyhow::Result<()> {
    let wasm = get_nft_wasm("./").await;
    let worker = workspaces::testnet().await?;
    let account = worker.dev_create_account().await?;
    let contract = account.deploy(&worker, wasm).await?.into_result()?;
    let result = contract
        .call(&worker, "init")
        .args_json(json!({"owner_id": contract.id()}))?
        .transact()
        .await?;
    assert!(result.is_success());

    let result = contract
        .call(&worker, "init")
        .args_json(json!({"owner_id": contract.id()}))?
        .transact()
        .await;

    assert!(result
        .unwrap_err()
        .to_string()
        .contains("The contract has already been initialized"));

    Ok(())
}
// #[tokio::test]
// async fn rand() -> Result<()> {
//     let worker = workspaces::testnet().await?;
//     let wasm = workspaces::compile_project("../nft_token").await?;
//     let wasm = tokio::fs::read(CONTRACT_WASM).await?;
// let account = worker
//     .dev_create_account()
//     .await
//     .expect("could not dev-deploy contract");
//
// let nft = account
//     .create_subaccount(&worker, "nft")
//     .initial_balance(50_000_000_000_000_000_000_000_000)
//     .transact()
//     .await?
//     .into_result()?;
//
// let contract = nft.deploy(&worker, &wasm).await?.into_result()?;
// let res = contract
//     .call(&worker, "init")
//     .args_json(json!({
//             "owner_id": contract.id(),
//     }))?
//     .transact()
//     .await?;
// println!("{:?}", res.outcome());
// let res1 = account
//     .call(&worker, contract.id(), "nft_mint")
//     .args_json(json!({
//         "receiver_id": "f0m0.testnet"
//     }))?
//     .max_gas()
//     .deposit(6470000000000000000000)
//     .transact()
//     .await?;
// println!("{res1:?}");

// account
//     .call(&worker, contract.id(), "nft_mint")
//     .args_json(json!({
//         "receiver_id": "battlemon.testnet"
//     }))?
//     .gas(300_000_000_000_000)
//     .deposit(6470000000000000000000)
//     .transact()
//     .await?;
//
// account
//     .call(&worker, contract.id(), "nft_mint")
//     .args_json(json!({
//         "receiver_id": "battlemon.testnet"
//     }))?
//     .gas(300_000_000_000_000)
//     .deposit(6470000000000000000000)
//     .transact()
//     .await?;
//
// account
//     .call(&worker, contract.id(), "nft_mint")
//     .args_json(json!({
//         "receiver_id": "battlemon.testnet"
//     }))?
//     .gas(300_000_000_000_000)
//     .deposit(6470000000000000000000)
//     .transact()
//     .await?;
// println!("\nres1: {:#?}", res1);

// Ok(())
// }
