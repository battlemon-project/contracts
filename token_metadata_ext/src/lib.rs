use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use nft_models::ModelKind;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadataExt {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<String>, // ISO 8601 datetime when token was issued or minted
    pub expires_at: Option<String>, // ISO 8601 datetime when token expires
    pub starts_at: Option<String>, // ISO 8601 datetime when token starts being valid
    pub updated_at: Option<String>, // ISO 8601 datetime when token was last updated
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    pub model: ModelKind,
}

impl TokenMetadataExt {
    pub fn split(self) -> (TokenMetadata, ModelKind) {
        let metadata = TokenMetadata {
            title: self.title,
            description: self.description,
            media: self.media,
            media_hash: self.media_hash,
            copies: self.copies,
            issued_at: self.issued_at,
            expires_at: self.expires_at,
            starts_at: self.starts_at,
            updated_at: self.updated_at,
            extra: self.extra,
            reference: self.reference,
            reference_hash: self.reference_hash,
        };

        (metadata, self.model)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenExt {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: Option<TokenMetadata>,
    pub model: ModelKind,
    pub approved_account_ids: Option<HashMap<AccountId, u64>>,
}

impl TokenExt {
    pub fn from_parts(token: Token, model: ModelKind) -> TokenExt {
        TokenExt {
            token_id: token.token_id,
            owner_id: token.owner_id,
            metadata: token.metadata,
            approved_account_ids: token.approved_account_ids,
            model,
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use nft_models::*;

    fn get_lemon_model() -> Lemon {
        let expected_model = Lemon {
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
            body_slot: None,
            left_weapon_slot: None,
            right_weapon_slot: None,
        };
        expected_model
    }

    #[test]
    fn token_metadata_ext_split() {
        let expected_model = get_lemon_model();

        let token_metadata_ext = TokenMetadataExt {
            title: None,
            description: None,
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
            model: expected_model.clone().into(),
        };

        let (actual_metadata, actual_model) = token_metadata_ext.clone().split();

        assert_eq!(actual_model, expected_model.into());
        assert_eq!(token_metadata_ext.title, actual_metadata.title);
        assert_eq!(token_metadata_ext.description, actual_metadata.description);
        assert_eq!(token_metadata_ext.media, actual_metadata.media);
        assert_eq!(token_metadata_ext.media_hash, actual_metadata.media_hash);
        assert_eq!(token_metadata_ext.copies, actual_metadata.copies);
        assert_eq!(token_metadata_ext.issued_at, actual_metadata.issued_at);
        assert_eq!(token_metadata_ext.expires_at, actual_metadata.expires_at);
        assert_eq!(token_metadata_ext.extra, actual_metadata.extra);
        assert_eq!(token_metadata_ext.reference, actual_metadata.reference);
        assert_eq!(
            token_metadata_ext.reference_hash,
            actual_metadata.reference_hash
        );
    }

    #[test]
    fn token_ext_from_parts() {
        let token = Token {
            token_id: "123".to_string(),
            owner_id: "owner".parse().unwrap(),
            metadata: None,
            approved_account_ids: None,
        };

        let expected_model = get_lemon_model();

        let token_ext = TokenExt::from_parts(token.clone(), expected_model.clone().into());
        assert_eq!(token.token_id, token_ext.token_id);
        assert_eq!(token.metadata, token_ext.metadata);
        assert_eq!(token.owner_id, token_ext.owner_id);
        assert_eq!(token.approved_account_ids, token_ext.approved_account_ids);
        let expected: ModelKind = expected_model.into();
        assert_eq!(expected, token_ext.model)
    }
}
