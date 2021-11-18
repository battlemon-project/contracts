//! This is **Play2Earn** smart-contract
//! It's accumulates player's match statistics and send Juice fungible token as royalties for
//! the player's progress.

use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{self, Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, require, AccountId, Balance,
    BorshStorageKey, Gas, PanicOnDefault, Promise,
};

const PROCESS_PROGRESS_GAS: Gas = Gas(10_000_000_000_000 + FT_TRANSFER_GAS.0);
const FT_TRANSFER_GAS: Gas = Gas(5_000_000_000_000);
const ONE_YOCTO: Balance = 1;

///Juice's interface of smart-contract
#[ext_contract]
trait ExtJuice {
    fn ft_transfer(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) -> Promise;
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Progress,
}

/// Player progress
/// Each incoming progress will be converted into Juice fungible token
#[derive(Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Progress {
    pub player_id: AccountId,
    pub played_time: u64,
    pub total_damage: u64,
    pub hp_level: u64,
    pub walking_distance: u64,
    pub match_result: bool,
}

impl Progress {
    /// Convert progress into amount of Juice token.
    fn calculate_juice(&self) -> u64 {
        let ret = self.played_time + self.hp_level + self.total_damage + self.walking_distance;

        if self.match_result {
            return ret * 2;
        }
        ret
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    juice_id: AccountId,
    progress_provider_id: AccountId,
}

#[near_bindgen]
impl Contract {
    /// Initialization of smart-contract
    /// # Arguments
    /// * `juice_id` - account id of Juice fungible token smart-contract
    /// * `progress_provider_id` - game's account id ()
    #[init]
    pub fn init(juice_id: AccountId, progress_provider_id: AccountId) -> Self {
        Self {
            juice_id, // progress_by_token_id: LookupMap::new(StorageKey::Progress),
            progress_provider_id,
        }
    }

    /// Process incoming player's progress
    /// Deserialize JSON into `Progress`, calculate juice and give out royalties
    /// # Arguments
    /// * `progress` - `Progress` deserialized struct.
    pub fn process_progress(&mut self, progress: Progress) -> Promise {
        assert_one_yocto();
        require!(
            env::prepaid_gas() >= PROCESS_PROGRESS_GAS,
            format!("prepared gas must be more than {}", PROCESS_PROGRESS_GAS.0)
        );
        require!(
            env::predecessor_account_id() == self.progress_provider_id,
            "You aren't verified progress provider"
        );

        let royalty_amount = progress.calculate_juice();
        ext_juice::ft_transfer(
            progress.player_id,
            U128(royalty_amount as u128),
            None,
            self.juice_id.clone(),
            ONE_YOCTO,
            FT_TRANSFER_GAS,
        )
    }
}
