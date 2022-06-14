use anyhow::Context;
use serde_json::json;
use test_helpers::{deploy_contract, parse_near, workspaces, workspaces::prelude::*};
const CONTRACT_WASM: &str = "../target/wasm32-unknown-unknown/release/nft_token.wasm";

#[tokio::test]
async fn contract_is_initable() -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let root = worker.dev_create_account().await?;
    let contract =
        deploy_contract(&worker, "nft", parse_near!("10 N"), &root, CONTRACT_WASM).await?;
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
    let worker = workspaces::testnet()
        .await
        .context("Failed to create worker")?;
    let root = worker.dev_create_account().await?;
    let contract =
        deploy_contract(&worker, "nft", parse_near!("10 N"), &root, CONTRACT_WASM).await?;
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
    let worker = workspaces::testnet()
        .await
        .context("Failed to create worker")?;
    let root = worker.dev_create_account().await?;
    let contract =
        deploy_contract(&worker, "nft", parse_near!("10 N"), &root, CONTRACT_WASM).await?;

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
    let worker = workspaces::testnet().await?;
    let root = worker.dev_create_account().await?;
    let contract =
        deploy_contract(&worker, "nft", parse_near!("10 N"), &root, CONTRACT_WASM).await?;

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
    let worker = workspaces::testnet().await?;
    let root = worker.dev_create_account().await?;
    let contract =
        deploy_contract(&worker, "nft", parse_near!("10 N"), &root, CONTRACT_WASM).await?;
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

#[tokio::test]
async fn nft_transfer_works() -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let root = worker.dev_create_account().await?;
    let contract =
        deploy_contract(&worker, "nft", parse_near!("10 N"), &root, CONTRACT_WASM).await?;
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

    let bob = worker.dev_create_account().await?;
    let result = alice
        .call(&worker, contract.id(), "nft_transfer")
        .deposit(1)
        .args_json(json!({"receiver_id": bob.id(), "token_id": "1"}))?
        .transact()
        .await?;

    assert!(result.is_success());

    let result = contract
        .call(&worker, "nft_token")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<token_metadata_ext::TokenExt>()?;

    assert_eq!(result.owner_id.as_str(), bob.id().as_str());

    Ok(())
}

#[tokio::test]
async fn nft_transfer_is_prohibited_for_not_owner() -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let root = worker.dev_create_account().await?;
    let contract =
        deploy_contract(&worker, "nft", parse_near!("10 N"), &root, CONTRACT_WASM).await?;
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

    let bob = worker.dev_create_account().await?;
    let result = bob
        .call(&worker, contract.id(), "nft_transfer")
        .deposit(1)
        .args_json(json!({"receiver_id": bob.id(), "token_id": "1"}))?
        .transact()
        .await;
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Smart contract panicked: Unauthorized"));

    Ok(())
}

#[tokio::test]
async fn transferred_token_not_allowed_for_prev_owner() -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let root = worker.dev_create_account().await?;
    let contract =
        deploy_contract(&worker, "nft", parse_near!("10 N"), &root, CONTRACT_WASM).await?;
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

    let bob = worker.dev_create_account().await?;
    let result = alice
        .call(&worker, contract.id(), "nft_transfer")
        .deposit(1)
        .args_json(json!({"receiver_id": bob.id(), "token_id": "1"}))?
        .transact()
        .await?;
    assert!(result.is_success());

    let result = alice
        .call(&worker, contract.id(), "nft_transfer")
        .deposit(1)
        .args_json(json!({"receiver_id": alice.id(), "token_id": "1"}))?
        .transact()
        .await;
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Smart contract panicked: Unauthorized"));

    Ok(())
}

#[tokio::test]
async fn update_token_media_works() -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let root = worker.dev_create_account().await?;
    let contract =
        deploy_contract(&worker, "nft", parse_near!("10 N"), &root, CONTRACT_WASM).await?;
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
        .call(&worker, "update_token_media")
        .args_json(json!({"token_id": "1", "new_media": "foo"}))?
        .transact()
        .await?;
    assert!(result.is_success());

    let result = contract
        .call(&worker, "nft_token")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<token_metadata_ext::TokenExt>()?;
    let metadata = result.metadata.expect("Metadata must be present");
    assert_eq!(metadata.media, Some("foo".to_string()));

    Ok(())
}

#[tokio::test]
async fn update_token_media_can_be_called_only_by_contract_account() -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let root = worker.dev_create_account().await?;
    let contract =
        deploy_contract(&worker, "nft", parse_near!("10 N"), &root, CONTRACT_WASM).await?;
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

    let result = alice
        .call(&worker, contract.id(), "update_token_media")
        .deposit(1)
        .args_json(json!({"token_id": "1", "new_media": "foo"}))?
        .transact()
        .await;

    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Smart contract panicked: Unauthorized"));

    Ok(())
}
