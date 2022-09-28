use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{env, AccountId};

use battlemon_models::nft::ModelKind;

use crate::error::{ContractError, Result};
use crate::Contract;

macro_rules! replace_outfit {
    ($lemon: ident . $field: ident, $outfit: expr) => {{
        $lemon.$field = Some($outfit);
        ModelKind::Lemon($lemon)
    }};
}

macro_rules! remove_outfit {
    ($lemon: ident . $field: ident) => {{
        $lemon.$field = None;
        ModelKind::Lemon($lemon)
    }};
}

impl Contract {
    pub(crate) fn owner(&self, id: &TokenId) -> Result<AccountId> {
        self.tokens
            .owner_by_id
            .get(id)
            .ok_or_else(|| ContractError::OwnerNotFound(id.to_owned()))
    }

    pub(crate) fn model(&self, id: &TokenId) -> Result<ModelKind> {
        self.model_by_id
            .get(id)
            .ok_or_else(|| ContractError::ModelNotFound(id.to_owned()))
    }

    pub(crate) fn check_instructions(&self, instructions: &[TokenId]) -> Result<()> {
        if instructions.len() < 2 {
            return Err(ContractError::InstructionError(
                "Not enough ids in instructions".to_string(),
            ));
        }

        let lemon_model = self.model(&instructions[0])?;
        if !matches!(lemon_model, ModelKind::Lemon(_)) {
            return Err(ContractError::InstructionError(
                "First id in instructions is not a lemon".to_string(),
            ));
        }
        self.are_ids_belong_to_predecessor(instructions)?;

        Ok(())
    }

    pub(crate) fn are_ids_belong_to_predecessor(&self, ids: &[TokenId]) -> Result<()> {
        for id in ids {
            let owner_of_id = self.owner(id)?;
            if !is_predecessor(&owner_of_id) {
                return Err(ContractError::NotAuthorized(format!(
                    "Contract caller isn't the owner of the token id: {id}"
                )));
            }
        }

        Ok(())
    }

    pub(crate) fn merge_ids(&mut self, lemon_id: &TokenId, outfit_id: &TokenId) {
        let lemon_model = self.model(lemon_id).unwrap();
        let outfit_model = self.model(outfit_id).unwrap();
        if let ModelKind::Lemon(mut lemon) = lemon_model {
            let lemon_model = match outfit_model {
                ModelKind::FireArm(firearm) => replace_outfit!(lemon.fire_arm, firearm),
                ModelKind::ColdArm(coldarm) => replace_outfit!(lemon.cold_arm, coldarm),
                ModelKind::Cloth(cloth) => replace_outfit!(lemon.cloth, cloth),
                ModelKind::Cap(cap) => replace_outfit!(lemon.cap, cap),
                ModelKind::Back(back) => replace_outfit!(lemon.back, back),
                _ => unreachable!(),
            };

            self.model_by_id.insert(lemon_id, &lemon_model);
        }
    }

    pub(crate) fn unmerge_ids(&mut self, lemon_id: &TokenId, outfit_id: &TokenId) {
        let lemon_model = self.model(lemon_id).unwrap();
        let outfit_model = self.model(outfit_id).unwrap();
        if let ModelKind::Lemon(mut lemon) = lemon_model {
            let lemon_model = match outfit_model {
                ModelKind::FireArm(_) => remove_outfit!(lemon.fire_arm),
                ModelKind::ColdArm(_) => remove_outfit!(lemon.cold_arm),
                ModelKind::Cloth(_) => remove_outfit!(lemon.cloth),
                ModelKind::Cap(_) => remove_outfit!(lemon.cap),
                ModelKind::Back(_) => remove_outfit!(lemon.back),
                _ => unreachable!(),
            };

            self.model_by_id.insert(lemon_id, &lemon_model);
        }
    }

    pub(crate) fn disassemble_all(&mut self, token_id: &TokenId) {
        let model = self.model(token_id).unwrap();
        if let ModelKind::Lemon(mut lemon) = model {
            lemon.fire_arm = None;
            lemon.cold_arm = None;
            lemon.cloth = None;
            lemon.cap = None;
            lemon.back = None;
            lemon.sets.clear();

            self.model_by_id.insert(token_id, &ModelKind::Lemon(lemon));
        }
    }

    pub(crate) fn burn_token(&mut self, token_id: &TokenId) {
        self.model_by_id.remove(token_id);
        let tokens = &mut self.tokens;
        let owner_id = tokens.owner_by_id.remove(token_id).unwrap();

        if let Some(collection) = &mut tokens.token_metadata_by_id {
            collection.remove(token_id);
        }

        if let Some(collection) = &mut tokens.tokens_per_owner {
            let mut tokens = collection.get(&owner_id).unwrap();
            tokens.remove(token_id);
            if tokens.is_empty() {
                collection.remove(&owner_id);
            } else {
                collection.insert(&owner_id, &tokens);
            }
        }

        if let Some(collection) = &mut tokens.approvals_by_id {
            collection.remove(token_id);
        }

        if let Some(collection) = &mut tokens.next_approval_id_by_id {
            collection.remove(token_id);
        }

        near_contract_standards::non_fungible_token::events::NftBurn {
            owner_id: &owner_id,
            token_ids: &[token_id.as_str()],
            authorized_id: None,
            memo: None,
        }
        .emit();
    }
}

pub(crate) fn is_predecessor(id: &AccountId) -> bool {
    *id == env::predecessor_account_id()
}
