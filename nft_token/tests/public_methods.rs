use battlemon_models::nft::NftKind;
use lemotests::prelude::*;
use lemotests_macro::add_helpers;
use near_contract_standards::non_fungible_token::TokenId;
use token_metadata_ext::TokenExt;

const NFT_PATH: &str = "../target/wasm32-unknown-unknown/release/nft_token.wasm";
const NFT: &str = "nft_contract";

add_helpers!("./nft_schema.json");

#[tokio::test]
async fn contract_is_initable() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .build()
        .await?;

    bchain
        .call_nft_contract_init(ALICE)?
        .with_gas(Tgas(200))
        .execute()
        .await?;

    Ok(())
}

#[tokio::test]
async fn contract_is_initable_by_any_account() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let alice = bchain.alice_id()?.to_owned();

    let result = bchain
        .alice_call_nft_contract_init(&alice)?
        .with_gas(Tgas(10))
        .execute()
        .await?;

    assert!(result[0].is_success());

    Ok(())
}

#[tokio::test]
async fn double_initialization_contract_rejected() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .build()
        .await?;

    let result = bchain
        .call_nft_contract_init(ALICE)?
        .with_gas(Tgas(10))
        .then()
        .call_nft_contract_init(BOB)?
        .with_gas(Tgas(10))
        .execute()
        .await;

    assert!(result.contains_error("The contract has already been initialized"));

    Ok(())
}

#[tokio::test]
async fn mint_works() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let [nft, alice] = bchain.string_ids()?;

    bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .view_nft_contract_nft_token("1")?
        .execute()
        .await?;

    Ok(())
}

#[tokio::test]
async fn minted_token_belongs_to_receiver_id() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let [nft, alice] = bchain.string_ids()?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .view_nft_contract_nft_token("1")?
        .execute()
        .await?;
    let view_tx = result[2].json::<TokenExt>()?;

    assert_eq!(view_tx.token_id, "1");
    assert_eq!(view_tx.owner_id.as_str(), alice);

    Ok(())
}

#[tokio::test]
async fn nft_transfer_works() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let [nft, alice, bob] = bchain.string_ids()?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_nft_transfer(&bob, "1", None, None)?
        .with_deposit(1)
        .with_gas(Tgas(10))
        .then()
        .view_nft_contract_nft_token("1")?
        .execute()
        .await?;

    let view_tx = result[3].json::<TokenExt>()?;
    assert_eq!(view_tx.owner_id.as_str(), bob.as_str());

    Ok(())
}

#[tokio::test]
async fn nft_transfer_is_prohibited_for_not_owner() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let [nft, alice, bob] = bchain.string_ids()?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .bob_call_nft_contract_nft_transfer(&bob, "1", None, None)?
        .with_deposit(1)
        .with_gas(Tgas(10))
        .execute()
        .await;

    assert!(result.contains_error("Smart contract panicked: Unauthorized"));

    Ok(())
}

#[tokio::test]
async fn transferred_token_not_allowed_for_prev_owner() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let [nft, alice, bob] = bchain.string_ids()?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_nft_transfer(&bob, "1", None, None)?
        .with_deposit(1)
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_transfer(&alice, "1", None, None)?
        .with_deposit(1)
        .with_gas(Tgas(10))
        .execute()
        .await;

    let error = format!("{:?}", result.unwrap_err());
    assert!(error.contains("Smart contract panicked: Unauthorized"));

    Ok(())
}

#[tokio::test]
async fn update_token_media_works() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let [nft, alice] = bchain.string_ids()?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .call_nft_contract_update_token_media("1", "foo-hash")?
        .with_deposit(1)
        .with_gas(Tgas(10))
        .then()
        .view_nft_contract_nft_token("1")?
        .execute()
        .await?;

    let view_tx = result[3].json::<TokenExt>()?;

    let metadata = view_tx.metadata.expect("Metadata must be present");
    assert_eq!(metadata.media, Some("foo-hash".to_string()));

    Ok(())
}

#[tokio::test]
async fn update_token_media_can_be_called_only_by_contract_account() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let [nft, alice] = bchain.string_ids()?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_update_token_media("1", "foo-hash")?
        .with_deposit(1)
        .with_gas(Tgas(10))
        .execute()
        .await;

    assert!(result.contains_error("Smart contract panicked: Unauthorized"));

    Ok(())
}

#[tokio::test]
async fn nft_approve_method_works() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let [nft, alice, bob] = bchain.string_ids()?;

    bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_nft_approve("1", &bob, None)?
        .with_deposit(490000000000000000000)
        .with_gas(Tgas(10))
        .execute()
        .await?;

    Ok(())
}

#[tokio::test]
async fn after_mint_exceeded_attached_deposit_is_refunded() -> anyhow::Result<()> {
    let bchain = StateBuilder::sandbox()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let [nft, alice] = bchain.string_ids()?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_gas(Tgas(10))
        .with_deposit(Near(9))
        .then()
        .view_account(ALICE)?
        .execute()
        .await?;
    let alice_balance = result[2].balance();

    assert!(alice_balance > Near(9));
    Ok(())
}
