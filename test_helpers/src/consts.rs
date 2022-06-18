use near_sdk::Balance;
use near_units::parse_near;

pub const ALMOST_ZERO: Balance = parse_near!("0.1 N");
pub const ONE_NEAR: Balance = parse_near!("1 N");
pub const FOUR_NEAR: Balance = parse_near!("4 N");
pub const FIVE_NEAR: Balance = parse_near!("5 N");
pub const SIX_NEAR: Balance = parse_near!("6 N");
pub const TEN_NEAR: Balance = parse_near!("10 N");
pub const FIFTEEN_NEAR: Balance = parse_near!("15 N");
pub const SIXTEEN_NEAR: Balance = parse_near!("16 N");
pub const NFT: &str = "nft";
pub const MARKET: &str = "market";

pub const NFT_PATH: &str = "../target/wasm32-unknown-unknown/release/nft_token.wasm";
pub const MARKET_PATH: &str = "../target/wasm32-unknown-unknown/release/nft_market.wasm";
