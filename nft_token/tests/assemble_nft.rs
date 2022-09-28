use battlemon_models::nft::{Lemon, ModelKind, NftKind, TokenExt};
use lemotests::prelude::*;
use lemotests_macro::add_helpers;
use near_contract_standards::non_fungible_token::TokenId;

const NFT_PATH: &str = "../target/wasm32-unknown-unknown/release/nft_token.wasm";
const NFT: &str = "nft_contract";

add_helpers!("./nft_schema.json");

#[tokio::test]
async fn assemble_works() -> anyhow::Result<()> {
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
        .alice_call_nft_contract_nft_mint(&alice, NftKind::FireArm)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::ColdArm)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Cloth)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Back)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Cap)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .view_nft_contract_nft_tokens_for_owner(&alice)?
        .with_label("tokens")
        .execute()
        .await?;

    let alice_tokens: Vec<TokenExt> = result.tx("tokens")?.json()?;
    let lemon_token = alice_tokens.first().expect("Empty tokens list");

    let alice_tokens_ids = alice_tokens
        .iter()
        .map(|t| t.token_id.clone())
        .collect::<Vec<TokenId>>();

    let result = result
        .into_state()
        .alice_call_nft_contract_assemble_compound_nft(alice_tokens_ids.clone())?
        .with_gas(Tgas(10))
        .with_deposit(1)
        .then()
        .view_nft_contract_nft_token(&lemon_token.token_id)?
        .with_label("token")
        .execute()
        .await?;

    let lemon: TokenExt = result.tx("token")?.json()?;
    let ModelKind::Lemon(
        Lemon {
            fire_arm: Some(fire_arm),
            cold_arm: Some(cold_arm),
            cloth: Some(cloth),
            cap: Some(cap),
            back: Some(back),
            ..
        }
    ) = lemon.model else {
        panic!("Wrong model kind")
    };
    assert!(matches!(lemon_token.model, ModelKind::Lemon(_)));
    assert_eq!(lemon.token_id, alice_tokens_ids[0]);
    assert_eq!(fire_arm.token_id, alice_tokens_ids[1]);
    assert_eq!(cold_arm.token_id, alice_tokens_ids[2]);
    assert_eq!(cloth.token_id, alice_tokens_ids[3]);
    assert_eq!(back.token_id, alice_tokens_ids[4]);
    assert_eq!(cap.token_id, alice_tokens_ids[5]);

    Ok(())
}

#[tokio::test]
async fn disassemble_works() -> anyhow::Result<()> {
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
        .alice_call_nft_contract_nft_mint(&alice, NftKind::FireArm)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::ColdArm)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Cloth)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Back)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Cap)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .view_nft_contract_nft_tokens_for_owner(&alice)?
        .with_label("tokens")
        .execute()
        .await?;

    let alice_tokens: Vec<TokenExt> = result.tx("tokens")?.json()?;
    let lemon_token = alice_tokens.first().expect("Empty tokens list");

    let alice_tokens_ids = alice_tokens
        .iter()
        .map(|t| t.token_id.clone())
        .collect::<Vec<TokenId>>();

    let result = result
        .into_state()
        .alice_call_nft_contract_assemble_compound_nft(alice_tokens_ids.clone())?
        .with_gas(Tgas(10))
        .with_deposit(1)
        .then()
        .view_nft_contract_nft_token(&lemon_token.token_id)?
        .with_label("token")
        .execute()
        .await?;

    let lemon: TokenExt = result.tx("token")?.json()?;
    assert!(matches!(
        lemon.model,
        ModelKind::Lemon(Lemon {
            fire_arm: Some(_),
            cold_arm: Some(_),
            cloth: Some(_),
            cap: Some(_),
            back: Some(_),
            ..
        })
    ));
    assert!(matches!(lemon_token.model, ModelKind::Lemon(_)));

    let result = result
        .into_state()
        .alice_call_nft_contract_disassemble_compound_nft(alice_tokens_ids)?
        .with_gas(Tgas(10))
        .with_deposit(1)
        .then()
        .view_nft_contract_nft_token(&lemon_token.token_id)?
        .with_label("token")
        .execute()
        .await?;

    let lemon: TokenExt = result.tx("token")?.json()?;

    assert!(matches!(
        lemon.model,
        ModelKind::Lemon(Lemon {
            fire_arm: None,
            cold_arm: None,
            cloth: None,
            cap: None,
            back: None,
            ..
        })
    ));

    Ok(())
}
