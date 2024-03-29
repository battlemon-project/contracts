use battlemon_models::nft::{Lemon, ModelKind, NftKind, TokenExt};
use lemotests::prelude::*;
use lemotests_macro::add_helpers;
use near_contract_standards::non_fungible_token::TokenId;

const NFT_PATH: &str = "../target/wasm32-unknown-unknown/release/nft_token.wasm";
const NFT: &str = "nft_contract";

add_helpers!("./nft_schema.json");

#[tokio::test]
async fn full_mint_works() -> anyhow::Result<()> {
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
        .alice_call_nft_contract_nft_mint_full(&alice)?
        .with_gas(Tgas(100))
        .with_deposit(Near(1))
        .with_label("lemon")
        .execute()
        .await?;

    let tokens: Vec<TokenExt> = result.tx("lemon")?.json()?;
    let token = tokens.last().expect("Failed to get nft token");

    assert!(matches!(
        token.model,
        ModelKind::Lemon(Lemon {
            fire_arm: Some(_),
            cold_arm: Some(_),
            cloth: Some(_),
            cap: Some(_),
            back: Some(_),
            ..
        })
    ));

    Ok(())
}
