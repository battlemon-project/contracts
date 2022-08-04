use battlemon_models::market::ask_contract::Ask;
use battlemon_models::market::bid_contract::Bid;
use near_sdk::env::STORAGE_PRICE_PER_BYTE;
use near_sdk::{Balance, Gas};

pub const STORAGE_PER_SALE: u128 = minimum_deposit();
pub const EVENT_PREFIX: &str = "EVENT_JSON";

pub const NO_DEPOSIT: Balance = 0;
pub const ONE_YOCTO: Balance = 1;
pub const BUY_METHOD_TOTAL_GAS: Gas = Gas(80_000_000_000_000);
pub const NFT_TRANSFER_GAS: Gas = Gas(44_000_000_000_000);
pub const AFTER_NFT_TRANSFER_GAS: Gas = Gas(20_000_000_000_000);

const MAX_ACCOUNT_ID_LENGTH: usize = 64;
const MAX_TOKEN_ID_LENGTH: usize = 20; //u64::MAX in string representation is 20 chars

const fn minimum_deposit() -> Balance {
    let bid_size = std::mem::size_of::<Bid>() + MAX_ACCOUNT_ID_LENGTH + MAX_TOKEN_ID_LENGTH;
    let ask_size = std::mem::size_of::<Ask>() + MAX_ACCOUNT_ID_LENGTH + MAX_TOKEN_ID_LENGTH;

    let largest = if bid_size > ask_size {
        bid_size
    } else {
        ask_size
    };

    STORAGE_PRICE_PER_BYTE * largest as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_bid_equals_172() {
        let bid_size = std::mem::size_of::<Bid>() + MAX_ACCOUNT_ID_LENGTH + MAX_TOKEN_ID_LENGTH;
        assert_eq!(bid_size, 172);
    }

    #[test]
    fn size_of_ask_equals_156() {
        let ask_size = std::mem::size_of::<Ask>() + MAX_ACCOUNT_ID_LENGTH + MAX_TOKEN_ID_LENGTH;
        assert_eq!(ask_size, 156);
    }
}
