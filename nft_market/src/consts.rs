use near_sdk::{Balance, Gas};

pub const EVENT_PREFIX: &str = "EVENT_JSON";

pub const NO_DEPOSIT: Balance = 0;
pub const ONE_YOCTO: Balance = 1;
pub const BUY_METHOD_TOTAL_GAS: Gas = Gas(80_000_000_000_000);
pub const NFT_TRANSFER_GAS: Gas = Gas(44_000_000_000_000);
pub const AFTER_NFT_TRANSFER_GAS: Gas = Gas(20_000_000_000_000);
