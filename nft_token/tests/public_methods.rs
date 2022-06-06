use anyhow::Context;
use serde_json::json;
use test_helpers::{load_wasm, parse_near, workspaces, workspaces::prelude::*};
const CONTRACT_WASM: &str = "../target/wasm32-unknown-unknown/release/nft_token.wasm";

#[tokio::test]
async fn contract_is_initable() -> anyhow::Result<()> {
    let wasm = load_wasm(CONTRACT_WASM).await;
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
    let wasm = load_wasm(CONTRACT_WASM).await;
    let worker = workspaces::testnet()
        .await
        .context("Failed to create worker")?;
    let alice = worker
        .dev_create_account()
        .await
        .context("Failed to create account for alice")?;
    let contract = alice
        .deploy(&worker, wasm)
        .await
        .context("Failed to deploy contract")?
        .into_result()?;
    let bob = worker
        .dev_create_account()
        .await
        .context("Failed to create account for bob")?;

    let result = bob
        .call(&worker, contract.id(), "init")
        .args_json(json!({"owner_id": bob.id()}))?
        .transact()
        .await
        .context("Failed to call contract's method `init`")?;

    assert!(result.is_success());

    Ok(())
}

#[tokio::test]
async fn double_initialization_contract_rejected() -> anyhow::Result<()> {
    let wasm = load_wasm(CONTRACT_WASM).await;
    let worker = workspaces::testnet()
        .await
        .context("Failed to create worker")?;
    let account = worker
        .dev_create_account()
        .await
        .context("Failed to create account")?;
    let contract = account
        .deploy(&worker, wasm)
        .await
        .context("Failed to deploy contract")?
        .into_result()?;
    let result = contract
        .call(&worker, "init")
        .args_json(json!({"owner_id": contract.id()}))?
        .transact()
        .await
        .context("Failed to call `init` contract's method")?;
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

#[tokio::test]
async fn mint_works() -> anyhow::Result<()> {
    let wasm = load_wasm(CONTRACT_WASM).await;
    let worker = workspaces::testnet().await?;
    let account = worker.dev_create_account().await?;
    let contract = account.deploy(&worker, wasm).await?.into_result()?;
    let result = contract
        .call(&worker, "init")
        .args_json(json!({"owner_id": contract.id()}))?
        .transact()
        .await?;
    assert!(result.is_success());

    let alice = worker.dev_create_account().await?;
    let result = alice
        .call(&worker, contract.id(), "nft_mint")
        .deposit(parse_near!("1 N"))
        .args_json(json!({"receiver_id": alice.id()}))?
        .transact()
        .await?;

    assert!(result.is_success());

    Ok(())
}

#[tokio::test]
async fn minted_token_belongs_to_receiver_id() -> anyhow::Result<()> {
    let wasm = load_wasm(CONTRACT_WASM).await;
    let worker = workspaces::testnet().await?;
    let account = worker.dev_create_account().await?;
    let contract = account.deploy(&worker, wasm).await?.into_result()?;
    let result = contract
        .call(&worker, "init")
        .args_json(json!({"owner_id": contract.id()}))?
        .transact()
        .await?;
    assert!(result.is_success());

    let alice = worker.dev_create_account().await?;
    let result = alice
        .call(&worker, contract.id(), "nft_mint")
        .deposit(parse_near!("1 N"))
        .args_json(json!({"receiver_id": alice.id()}))?
        .transact()
        .await?;
    assert!(result.is_success());

    let result = contract
        .call(&worker, "nft_token")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<token_metadata_ext::TokenExt>()?;
    assert_eq!(result.owner_id.as_str(), alice.id().as_str());

    Ok(())
}
