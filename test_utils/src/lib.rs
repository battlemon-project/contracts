use near_sdk::serde_json::json;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::AccountId;
use near_sdk_sim::to_yocto;
use nft_models::lemon::Lemon;
use nft_models::ModelKind;
use once_cell::unsync::Lazy;
use token_metadata_ext::*;

pub const MARKET_ACCOUNT_ID: &str = "market";
pub const NFT_ACCOUNT_ID: &str = "nft";
pub const SPOILED_NFT_ACCOUNT_ID: &str = "spoiled_nft";
pub const VALID_TOKEN_ID: &str = "valid token id";
pub const INVALID_TOKEN_ID: &str = "invalid token id";
pub const VALID_TOKEN_PRICE: Lazy<u128> = Lazy::new(|| to_yocto("10"));
pub const INVALID_TOKEN_PRICE: Lazy<u128> = Lazy::new(|| to_yocto("5"));
pub const BASE_DEPOSIT: Lazy<u128> = Lazy::new(|| to_yocto("100"));

pub fn get_tokens_id(n: u8) -> Vec<AccountId> {
    (0..n).map(|n| n.to_string()).collect()
}

pub fn sample_token_metadata() -> TokenMetadataExt {
    let model: ModelKind = get_foo_lemon().into();

    TokenMetadataExt {
        title: Some("foo title".into()),
        description: Some("this is description for foo title's token".into()),
        media: None,
        media_hash: None,
        copies: Some(1),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
        model,
    }
}

pub fn get_foo_lemon() -> Lemon {
    use nft_models::lemon::*;

    Lemon {
        option: Option_::OnSale,
        century: Century::Ancient,
        r#type: Type::Light,
        lemon_gen: LemonGen::Nakamoto,
        background: Background::Red,
        top: Top::Headdress,
        cyber_suit: CyberSuit::Black,
        expression: Expression::Brooding,
        eyes: Eyes::Open,
        hair: Hair::Elvis,
        accessory: Accessory::Cigar,
        winrate: None,
        rarity: 0,
        parent: None,
        body_slot: None,
        left_weapon_slot: None,
        right_weapon_slot: None,
    }
}

pub fn foo_token_metadata_ext() -> TokenMetadataExt {
    use nft_models::weapon::*;
    let model: ModelKind = Weapon {
        level: 0,
        r#type: Type::Instant,
        parent: None,
        scope_slot: None,
        perk_slot: None,
        mag_slot: None,
        barrel_slot: None,
        muzzle_slot: None,
        grip_slot: None,
        stock_slot: None,
    }
    .into();

    TokenMetadataExt {
        title: Some("foo_token".into()),
        description: Some("this is description for foo title's token".into()),
        media: Some("link to media".into()),
        media_hash: Some(vec![0, 1, 2, 3, 4].into()),
        copies: Some(1),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: Some(
            json!({
                "some": "json",
                "values": 9,
            })
            .to_string(),
        ),
        reference: None,
        reference_hash: None,
        model,
    }
}

pub fn baz_token_metadata_ext() -> TokenMetadataExt {
    use nft_models::lemon::*;
    let model: ModelKind = Lemon {
        option: Option_::Auction,
        century: Century::Otherworldly,
        r#type: Type::Heavy,
        lemon_gen: LemonGen::Nakamoto,
        background: Background::Red,
        top: Top::Classical,
        cyber_suit: CyberSuit::Black,
        expression: Expression::Angry,
        eyes: Eyes::Close,
        hair: Hair::Punkkez,
        accessory: Accessory::Toothpick,
        winrate: Some(33),
        rarity: 88,
        parent: None,
        body_slot: None,
        left_weapon_slot: None,
        right_weapon_slot: None,
    }
    .into();

    TokenMetadataExt {
        title: Some("baz_token".into()),
        description: Some("this is description for baz title's token".into()),
        media: Some("link to media".into()),
        media_hash: Some(vec![2, 3, 4, 3, 4].into()),
        copies: Some(1),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: Some(
            json!({
                "rust": "bless you!",
                "values": 2,
            })
            .to_string(),
        ),
        reference: None,
        reference_hash: None,
        model,
    }
}

pub fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}
