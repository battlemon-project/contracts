use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use slots_derive_macro::Slots;

use crate::slots::Slots;

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug, Slots,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Lemon {
    pub option: Option_,
    pub century: Century,
    pub r#type: Type,
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
    pub body_slot: Option<TokenId>,
    pub left_weapon_slot: Option<TokenId>,
    pub right_weapon_slot: Option<TokenId>,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum Option_ {
    OnSale,
    Auction,
    ForRent,
    LemonGen,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum Century {
    Ancient,
    OurTime,
    Future,
    Otherworldly,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum Type {
    Light,
    Medium,
    Heavy,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum LemonGen {
    Nakamoto,
    Buterin,
    Mask,
    Jobs,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum Background {
    Red,
    Purple,
    Black,
    Yellow,
    Green,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum Top {
    Headdress,
    Hairstyle,
    Classical,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum CyberSuit {
    Black,
    Metallic,
    Blue,
    Gold,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum Expression {
    Brooding,
    Merry,
    Angry,
    Tense,
    Relaxed,
    Mask,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum Eyes {
    Open,
    Close,
    Medium,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum Hair {
    Elvis,
    BobMarley,
    Punkkez,
    Disco,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum Accessory {
    Cigar,
    Toothpick,
    Tattoo,
    Scar,
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn lemon_model() {
        let _lemon = Lemon {
            option: Option_::OnSale,
            century: Century::Ancient,
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
            r#type: Type::Light,
        };
    }
}
