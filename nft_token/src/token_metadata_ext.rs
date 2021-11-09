use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Option_ {
    OnSale,
    Auction,
    ForRent,
    LemonGen,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Century {
    Ancient,
    OurTime,
    Future,
    Otherworldly,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Type {
    Light,
    Medium,
    Heavy,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum LemonGen {
    Nakamoto,
    Buterin,
    Mask,
    Jobs,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Background {
    Red,
    Purple,
    Black,
    Yellow,
    Green,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Top {
    Headdress,
    Hairstyle,
    Classical,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum CyberSuit {
    Black,
    Metallic,
    Blue,
    Gold,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Expression {
    Brooding,
    Merry,
    Angry,
    Tense,
    Relaxed,
    Mask,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Eyes {
    Open,
    Close,
    Medium,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Hair {
    Elvis,
    BobMarley,
    Punkkez,
    Disco,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Accessory {
    Cigar,
    Toothpick,
    Tattoo,
    Scar,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenProperties {
    pub option: Option_,
    pub century: Century,
    pub type_: Type,
    pub lemon_gen: LemonGen,
    pub background: Background,
    pub top: Top,
    pub cyber_suit: CyberSuit,
    pub expression: Expression,
    pub eyes: Eyes,
    pub hair: Hair,
    pub accessory: Accessory,
    pub winrate: Option<u8>,
    pub rarity: u8,
}

#[derive(Serialize, Deserialize)]
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
    pub properties: TokenProperties,
}

impl TokenMetadataExt {
    pub fn get_token_metadata(&self) -> TokenMetadata {
        TokenMetadata {
            title: self.title.clone(),
            description: self.description.clone(),
            media: self.media.clone(),
            media_hash: self.media_hash.clone(),
            copies: self.copies,
            issued_at: self.issued_at.clone(),
            expires_at: self.expires_at.clone(),
            starts_at: self.starts_at.clone(),
            updated_at: self.updated_at.clone(),
            extra: self.extra.clone(),
            reference: self.reference.clone(),
            reference_hash: self.reference_hash.clone(),
        }
    }

    pub fn get_token_properties(&self) -> TokenProperties {
        self.properties.clone()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenExt {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: Option<TokenMetadata>,
    pub properties: TokenProperties,
    pub approved_account_ids: Option<HashMap<AccountId, u64>>,
}

impl TokenExt {
    pub fn from_parts(token: Token, properties: TokenProperties) -> TokenExt {
        TokenExt {
            token_id: token.token_id,
            owner_id: token.owner_id,
            metadata: token.metadata,
            approved_account_ids: token.approved_account_ids,
            properties,
        }
    }
}
