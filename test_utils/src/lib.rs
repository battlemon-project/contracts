use near_sdk::AccountId;
use near_sdk::serde_json::json;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use token_metadata_ext::*;

pub fn sample_token_metadata() -> TokenMetadataExt {
    let properties = TokenProperties {
        option: Option_::OnSale,
        century: Century::Ancient,
        type_: Type::Light,
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
    };
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
        properties,
    }
}

pub fn foo_token_metadata_ext() -> TokenMetadataExt {
    let properties = TokenProperties {
        option: Option_::LemonGen,
        century: Century::Future,
        type_: Type::Medium,
        lemon_gen: LemonGen::Buterin,
        background: Background::Purple,
        top: Top::Hairstyle,
        cyber_suit: CyberSuit::Gold,
        expression: Expression::Brooding,
        eyes: Eyes::Open,
        hair: Hair::BobMarley,
        accessory: Accessory::Tattoo,
        winrate: Some(100),
        rarity: 10,
    };

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
        properties,
    }
}

pub fn baz_token_metadata_ext() -> TokenMetadataExt {
    let properties = TokenProperties {
        option: Option_::Auction,
        century: Century::Otherworldly,
        type_: Type::Heavy,
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
    };

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
        properties,
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
