use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Lemon {
    pub cap: Option<Cap>,
    pub cloth: Option<Cloth>,
    pub exo: Exo,
    pub eyes: Eyes,
    pub head: Head,
    pub teeth: Teeth,
}

impl Lemon {
    pub const TRAITS_COUNT: usize = 4;

    pub fn from_random(nums: &[u8; Self::TRAITS_COUNT]) -> Self {
        let [exo, eyes, head, teeth] = nums;

        let exo = match exo {
            0..=33 => Exo::BA01,
            34..=66 => Exo::MA01,
            _ => Exo::ZA01,
        };

        let eyes = match eyes {
            0..=33 => Eyes::A01,
            34..=66 => Eyes::B01,
            _ => Eyes::Z01,
        };

        let head = match head {
            0..=33 => Head::A01,
            34..=66 => Head::B01,
            _ => Head::Z01,
        };

        let teeth = match teeth {
            0..=33 => Teeth::A01,
            34..=66 => Teeth::B01,
            _ => Teeth::Z01,
        };

        Self {
            cap: None,
            cloth: None,
            exo,
            eyes,
            head,
            teeth,
        }
    }
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
#[serde(crate = "near_sdk::serde")]
)]
pub enum Exo {
    #[serde(rename = "ARM1_Exo_BA01")]
    BA01,
    #[serde(rename = "ARM1_Exo_MA01")]
    MA01,
    #[serde(rename = "ARM1_Exo_ZA01")]
    ZA01,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Cap {
    #[serde(rename = "ARM1_Cap_MA01")]
    MA01,
    #[serde(rename = "ARM1_Cap_ZA01")]
    ZA01,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Cloth {
    #[serde(rename = "ARM1_Cloth_MA01")]
    MA01,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Eyes {
    #[serde(rename = "ARM1_Eyes_A01")]
    A01,
    #[serde(rename = "ARM1_Eyes_B01")]
    B01,
    #[serde(rename = "ARM1_Eyes_Z01")]
    Z01,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Head {
    #[serde(rename = "ARM1_Head_A01")]
    A01,
    #[serde(rename = "ARM1_Head_B01")]
    B01,
    #[serde(rename = "ARM1_Head_Z01")]
    Z01,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Teeth {
    #[serde(rename = "ARM1_Teeth_A01")]
    A01,
    #[serde(rename = "ARM1_Teeth_B01")]
    B01,
    #[serde(rename = "ARM1_Teeth_Z01")]
    Z01,
}
