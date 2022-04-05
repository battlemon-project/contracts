use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{env, AccountId};
use rand::distributions::uniform::SampleUniform;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

use nft_models::ModelKind;

use crate::error::{BattlemonError as BtlError, Result};
use crate::Contract;

impl Contract {
    pub(crate) fn owner(&self, body_id: &TokenId) -> Result<AccountId> {
        self.tokens
            .owner_by_id
            .get(&body_id)
            .ok_or_else(|| BtlError::OwnerNotFound(body_id.to_owned()))
    }

    pub(crate) fn model(&self, token_id: &TokenId) -> Result<ModelKind> {
        self.model_by_id
            .get(token_id)
            .ok_or_else(|| BtlError::ModelNotFound(token_id.to_owned()))
    }
}

pub(crate) fn get_random_vec_range<T: SampleUniform + Copy>(n: u64, begin: T, end: T) -> Vec<T> {
    let seed: [u8; 32] = env::random_seed().try_into().unwrap();
    let mut rng = StdRng::from_seed(seed);
    (0..n).map(|_| rng.gen_range(begin, end)).collect()
}
