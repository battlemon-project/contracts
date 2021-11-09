use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};

const DATA_IMAGE_SVG_LEMON_LOGO: &str = "data:image/svg+xml,%3C%3Fxml version='1.0' encoding='utf-8'%3F%3E%3Csvg version='1.1' id='Layer_1' xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' x='0px' y='0px' viewBox='0 0 841.9 595.3' style='enable-background:new 0 0 841.9 595.3;' xml:space='preserve'%3E%3Cstyle type='text/css'%3E .st0%7Bfill-rule:evenodd;clip-rule:evenodd;%7D%0A%3C/style%3E%3Cg%3E%3Cpath class='st0' d='M331.9,266c2.1-3.5,6.6-4.6,10.1-2.6l0.2,0.1c5.2,3.2,9,8.3,11.5,15.3c-7.4,1.2-13.7,0.3-19-2.7l-0.2-0.1 C331,274,329.9,269.5,331.9,266L331.9,266z M351.5,193.6c13.5-6,27.8-9.5,42.2-10.4l11.5-19.8c12.2,0.4,24.5,2.2,36.3,5.5l5.2,22.3 c13.4,5.1,26,12.7,37.2,22.4l22.1-5.8c4.6,4.8,8.5,9.5,11.8,14l-22.8,16.8l-5.4,4c-11-14.9-25.4-26.5-41.8-34.1 c-17.4-8.1-37-11.6-56.8-9.6c-19.8,2-38.3,9.3-53.7,20.7c-15.4,11.4-27.9,26.9-35.6,45.3c-7.7,18.3-10.2,38.1-7.5,57.1 c2.4,17.8,9.3,35,20.3,49.9l-5.5,4.1l-22.8,16.8l-0.1-0.1c-3.3-4.5-6.7-9.6-9.9-15.4l12.1-19.5c-6-13.5-9.5-27.8-10.4-42.2 l-19.8-11.5c0.4-12.2,2.2-24.5,5.5-36.3l22.3-5.2c5.1-13.4,12.7-26,22.4-37.2l-5.8-22.1c8.9-8.4,18.9-15.8,29.5-21.8L351.5,193.6 L351.5,193.6z M484.2,246.7l-62,45.8c-2.5-3.5-5.9-6.2-9.7-8c-1.1-0.5-2.2-0.9-3.4-1.3l11.5-76.4c8.5,1.6,16.7,4.2,24.4,7.8 c10.2,4.7,19.7,11.2,27.9,19.1c-4.3,4.5-8.9,9.4-13.1,16.2c6.9-4.2,11.6-8.5,16.5-12.8C479.1,240.2,481.8,243.3,484.2,246.7 L484.2,246.7z M416.7,296.5l-29.1,21.5c-1.9-2.5-3.1-5.4-3.5-8.5c-0.4-3.2,0-6.4,1.3-9.5c1.3-3,3.4-5.6,5.9-7.5 c2.6-1.9,5.7-3.1,8.9-3.5c3.3-0.3,6.6,0.2,9.4,1.6C412.4,292,414.9,294,416.7,296.5L416.7,296.5z M382.1,322.1l-62,45.8 c-3.8-5.1-7.1-10.5-9.8-16.1c7.5-3.6,15.7-7.7,24.2-14.3c-10.6,2.1-18.5,5.3-26.8,8.4c-3.3-8-5.5-16.3-6.7-24.8 c-1.2-8.4-1.3-17-0.3-25.5l76.4,11.5c0,1.2,0.1,2.3,0.2,3.5c0.2,1.1,0.4,2.3,0.7,3.4c-5.6,1.8-11.7,4-18.3,7.8 c7.8-0.5,13.7-2,19.9-3.5C380.4,319.6,381.2,320.9,382.1,322.1L382.1,322.1z M398.1,282.5c-1.2-5.7-2.6-11.9-5.7-18.8 c-0.4,7.7,0.5,13.7,1.2,19.9c-1.3,0.4-2.5,1-3.6,1.6l-45.8-62c5.6-3.9,11.6-7.2,17.9-9.9c1.9,4.2,4.2,8.7,7.7,13.4 c-1-5.8-2.6-10.3-4.2-14.8c7.8-3,15.9-5,24.4-6c0.9,8.4,2.1,17.4,5.6,27.7c1.5-10.8,1-19.3,0.8-28.3c5.9-0.3,11.7-0.1,17.4,0.5 l-11.5,76.3c-1,0-1.9,0-2.9,0.1C399,282.4,398.6,282.4,398.1,282.5L398.1,282.5z M443.4,237.2c3.5,2.1,4.6,6.6,2.6,10.1l-0.1,0.2 c-3.2,5.2-8.3,9-15.3,11.5c-1.2-7.4-0.3-13.7,2.7-19l0.1-0.2C435.4,236.3,439.9,235.1,443.4,237.2L443.4,237.2z M384.5,289.4 l-45.8-62.1c-13.2,10.5-23.8,24.2-30.6,40.3c-0.4,1-0.9,2.1-1.3,3.1c4.4,1.8,8.8,3.6,13.7,6.8c-5.8-0.5-10.6-1.8-15-3.2 c-1.6,4.8-2.9,9.6-3.8,14.5l76.3,11.5c0.3-1,0.6-1.9,1-2.8C380.3,294.4,382.2,291.6,384.5,289.4L384.5,289.4z M316.7,315.3 c-1.1,1-1,3.3,0.4,4c5.5-0.6,11.5-1.2,18.4-3.6C328.2,314.7,322.7,315.2,316.7,315.3L316.7,315.3z'/%3E%3Cpath class='st0' d='M416.7,296.5l-29.1,21.5c-1.9-2.5-3.1-5.4-3.5-8.5c-0.4-3.2,0-6.4,1.3-9.5c1.3-3,3.4-5.6,5.9-7.5 c2.6-1.9,5.7-3.1,8.9-3.5c3.3-0.3,6.6,0.2,9.4,1.6C412.4,292,414.9,294,416.7,296.5L416.7,296.5z'/%3E%3Cpath class='st0' d='M351.5,193.6c13.5-6,27.8-9.5,42.2-10.4l11.5-19.8c12.2,0.4,24.5,2.2,36.3,5.5l5.2,22.3 c13.4,5.1,26,12.7,37.2,22.4l22.1-5.8c4.6,4.8,8.5,9.5,11.8,14l-22.8,16.8l-5.4,4c-11-14.9-25.4-26.5-41.8-34.1 c-17.4-8.1-37-11.6-56.8-9.6c-19.8,2-38.3,9.3-53.7,20.7c-15.4,11.4-27.9,26.9-35.6,45.3c-7.7,18.3-10.2,38.1-7.5,57.1 c2.4,17.8,9.3,35,20.3,49.9l-5.5,4.1l-22.8,16.8l-0.1-0.1c-3.3-4.5-6.7-9.6-9.9-15.4l12.1-19.5c-6-13.5-9.5-27.8-10.4-42.2 l-19.8-11.5c0.4-12.2,2.2-24.5,5.5-36.3l22.3-5.2c5.1-13.4,12.7-26,22.4-37.2l-5.8-22.1c8.9-8.4,18.9-15.8,29.5-21.8L351.5,193.6 L351.5,193.6z'/%3E%3Cpath class='st0' d='M463,371.8c3.9-0.9,6.4-4.9,5.4-8.9l0-0.2c-1.5-5.9-5.3-11.1-11.3-15.5c-3.3,6.7-4.3,13-3,19l0,0.2 C455.1,370.4,459.1,372.8,463,371.8L463,371.8z M526.3,231.9l-62,45.8c2.6,3.4,4.2,7.4,4.8,11.6c0.2,1.2,0.2,2.4,0.2,3.6l76.4,11.5 c1-8.6,0.9-17.2-0.3-25.7c-1.5-11.2-4.9-22.1-10.1-32.2c-5.5,2.8-11.6,5.8-19.4,7.8c6-5.3,11.5-8.5,17.1-12 C531,238.7,528.7,235.3,526.3,231.9L526.3,231.9z M424.1,307.4l-62,45.8c3.8,5.1,8,9.8,12.6,14.1c5.7-6.2,11.9-12.7,20.8-18.9 c-5.1,9.6-10.5,16.1-15.9,23.2c6.7,5.5,14,10.1,21.7,13.7c7.7,3.6,15.9,6.2,24.3,7.8l11.5-76.4c-1.1-0.3-2.2-0.8-3.3-1.2 c-1.1-0.5-2.1-1.1-3-1.7c-3.4,4.8-7.2,10-12.8,15.2c2.7-7.3,6-12.5,9.2-18C426,309.8,425,308.6,424.1,307.4L424.1,307.4z M466.7,303.7c5.1,2.8,10.6,6,16.3,11c-7.5-1.9-12.9-4.5-18.7-7c-0.8,1.1-1.7,2.1-2.6,3l45.8,62c5.3-4.2,10.3-9,14.7-14.2 c-3.5-3.1-7.1-6.5-10.5-11.2c5.2,2.7,9.1,5.5,12.9,8.3c5.1-6.5,9.5-13.8,12.9-21.5c-7.7-3.4-16-7.1-24.8-13.5 c10.8,1.7,18.8,4.7,27.3,7.5c2-5.5,3.6-11.1,4.6-16.8l-76.3-11.5c-0.3,0.9-0.6,1.8-1,2.7C467.1,302.9,466.9,303.3,466.7,303.7 L466.7,303.7z M523.3,273.7c-0.9-3.9-4.9-6.4-8.8-5.4l-0.2,0c-5.9,1.5-11.1,5.3-15.5,11.3c6.7,3.3,13,4.3,19,3l0.2,0 C521.9,281.6,524.3,277.7,523.3,273.7L523.3,273.7z M456.2,314.7l45.8,62.1c-13.9,9.5-30.2,15.7-47.5,17.4 c-1.1,0.1-2.2,0.2-3.4,0.3c-0.4-4.7-0.9-9.4-2.4-15.1c-1.2,5.7-1.4,10.7-1.3,15.3c-5.1,0.2-10.1-0.1-15-0.6l11.5-76.3 c1,0,2,0,3-0.1C450.2,317.2,453.3,316.2,456.2,314.7L456.2,314.7z M411.5,371.9c-1.2,0.8-3.5,0-3.8-1.6c2.2-5.1,4.5-10.6,8.8-16.5 C415.4,361,413.3,366.2,411.5,371.9L411.5,371.9z'/%3E%3Cpath class='st0' d='M458.8,281.8l-29.1,21.5c1.8,2.6,4.3,4.5,7.1,5.8c2.9,1.3,6.1,1.9,9.4,1.6c3.3-0.3,6.4-1.6,8.9-3.5 c2.6-1.9,4.7-4.5,5.9-7.5c1.3-3.1,1.7-6.3,1.3-9.5C461.9,287.2,460.7,284.3,458.8,281.8L458.8,281.8z'/%3E%3Cpath class='st0' d='M538,374.4c9.7-11.2,17.2-23.8,22.4-37.2l22.3-5.2c3.3-11.8,5.1-24,5.5-36.3l-19.8-11.6 c-1-14.3-4.4-28.6-10.5-42.1l12.1-19.5c-3.3-5.8-6.6-10.9-9.9-15.4l-22.8,16.8l-5.4,4c11,14.9,17.9,32.1,20.3,49.9 c2.6,19,0.2,38.8-7.5,57.1c-7.7,18.4-20.2,33.9-35.6,45.3C493.5,391.6,475,399,455.2,401c-19.8,2-39.4-1.5-56.8-9.6 c-16.3-7.6-30.8-19.2-41.8-34.1l-5.5,4.1l-22.8,16.8l0.1,0.1c3.3,4.5,7.2,9.2,11.8,14l22.2-5.8c11.2,9.7,23.8,17.2,37.2,22.4 l5.2,22.3c11.8,3.3,24,5.1,36.3,5.5l11.6-19.8c14.3-1,28.6-4.4,42.1-10.5l19.5,12.1c10.6-6,20.6-13.4,29.5-21.8L538,374.4 L538,374.4z'/%3E%3Cpath class='st0' d='M366.3,141.5c3.9-1,7.9,1.4,8.9,5.3l0,0.2c1.4,5.9,0.5,12.2-2.7,19c-6.1-4.4-9.9-9.5-11.5-15.3l-0.1-0.2 C360,146.5,362.4,142.5,366.3,141.5L366.3,141.5z'/%3E%3Cpath class='st0' d='M489,168.7c-0.3-2.9-4.2-5.5-7-4.1c-5.5,9.6-9.6,16.9-11.8,27.4C478.7,184.3,482.2,178.5,489,168.7L489,168.7z '/%3E%3Cpath class='st0' d='M263.4,220.5c-2.9,0.3-5.5,4.2-4.1,7c9.6,5.5,16.9,9.6,27.4,11.8C279,230.8,273.2,227.4,263.4,220.5 L263.4,220.5z'/%3E%3Cpath class='st0' d='M236.2,343.1c-1-3.9,1.3-7.9,5.3-8.9l0.2,0c5.9-1.4,12.2-0.5,19,2.7c-4.4,6.1-9.5,9.9-15.3,11.5l-0.2,0.1 C241.2,349.4,237.2,347.1,236.2,343.1L236.2,343.1z'/%3E%3Cpath class='st0' d='M592.1,375.6c2.1-3.5,1-8-2.4-10.1l-0.2-0.1c-5.2-3.1-11.5-4.1-18.9-3c2.4,7.1,6.1,12.3,11.3,15.5l0.2,0.1 C585.5,380.1,590,379,592.1,375.6L592.1,375.6z'/%3E%3Cpath class='st0' d='M602.2,250.3c2.7,1.1,4,5.6,1.9,7.9c-10.8,2.4-19,4.3-29.7,3.2C584.3,255.5,590.8,253.9,602.2,250.3 L602.2,250.3z'/%3E%3Cpath class='st0' d='M486.4,450.7c-1.1,2.7-5.6,4-7.9,1.9c-2.4-10.8-4.2-19-3.2-29.7C481.2,432.8,482.7,439.3,486.4,450.7 L486.4,450.7z'/%3E%3Cpath class='st0' d='M361.2,440.6c3.5,2.1,8,1,10.1-2.4l0.1-0.2c3.1-5.2,4.1-11.5,3-18.9c-7.1,2.4-12.3,6.1-15.5,11.3l-0.1,0.2 C356.7,434,357.8,438.5,361.2,440.6L361.2,440.6z'/%3E%3C/g%3E%3C/svg%3E%0A";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Metadata,
    NonFungibleToken,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner_id: AccountId) -> Self {
        let metadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Battlemon".to_string(),
            symbol: "BTLMN".to_string(),
            icon: Some(DATA_IMAGE_SVG_LEMON_LOGO.to_string()),
            base_uri: None,
            reference: None,
            reference_hash: None,
        };

        Self::new(owner_id, metadata)
    }

    fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        metadata.assert_valid();
        let metadata = LazyOption::new(StorageKey::Metadata, Some(&metadata));
        let tokens = NonFungibleToken::new(
            StorageKey::NonFungibleToken,
            owner_id,
            Some(StorageKey::TokenMetadata),
            Some(StorageKey::Enumeration),
            Some(StorageKey::Approval),
        );

        Self { tokens, metadata }
    }

    #[payable]
    pub fn mint(
        &mut self,
        token_id: TokenId,
        token_metadata: TokenMetadata,
        owner_id: Option<AccountId>,
    ) -> Token {
        require!(
            self.tokens.owner_id == env::predecessor_account_id(),
            "The contract caller must be owner"
        );

        let owner_id = if let Some(id) = owner_id {
            id
        } else {
            self.tokens.owner_id.clone()
        };

        self.tokens
            .internal_mint(token_id, owner_id, Some(token_metadata))
    }

    pub fn get_owner_by_token_id(&self, token_id: TokenId) -> Option<AccountId> {
        self.tokens.owner_by_id.get(&token_id)
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::token_metadata_ext::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use std::collections::HashMap;

    use super::*;

    const MINT_STORAGE_COST: u128 = 6_000_000_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_token_metadata() -> TokenMetadataExt {
        let properties = TokenProperties {
            option: Option_::OnSale,
            century: Century::Ancient,
            type_: Type::Light,
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
        };

        TokenMetadataExt {
            title: Some("foo title".into()),
            description: Some("this is description for foo title's token".into()),
            media: None,
            media_hash: None,
            copies: Some(1),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
            properties,
        }
    }

    #[test]
    fn init() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::init(accounts(1));
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_total_supply().0, 0);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        Contract::default();
    }

    #[test]
    fn mint_with_owner_contract() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(0));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .build());
        let token_id = "0".to_string();
        let token = contract.mint(token_id.clone(), sample_token_metadata(), None);

        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id, accounts(0));
        assert_eq!(
            token.metadata.unwrap(),
            sample_token_metadata().get_token_metadata()
        );
        assert_eq!(
            token.properties,
            sample_token_metadata().get_token_properties()
        );
        assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
    }

    #[test]
    fn mint_with_owner_id_is_not_contract_id() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(0));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .build());

        let token_id = "0".to_string();
        let token = contract.mint(token_id.clone(), sample_token_metadata(), Some(accounts(1)));
        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id, accounts(1));
        assert_eq!(
            token.metadata.unwrap(),
            sample_token_metadata().get_token_metadata()
        );
        assert_eq!(
            token.properties,
            sample_token_metadata().get_token_properties()
        );
        assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
    }

    #[test]
    #[should_panic(expected = "The contract caller must be owner")]
    fn mint_can_be_called_only_by_contract_owner() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(0));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .build());

        contract.mint("0".to_string(), sample_token_metadata(), Some(accounts(2)));
    }

    #[test]
    fn get_owner_by_id_valid_and_it_is_contract_owner() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(0));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .build());
        let token_id = "0".to_string();
        let token = contract.mint(token_id, sample_token_metadata(), None);
        let owner_id = contract.get_owner_by_token_id(token.token_id).unwrap();
        assert_eq!(owner_id, accounts(0));
    }

    #[test]
    fn get_owner_by_id_valid_and_it_is_not_contract_owner() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(0));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .build());
        let token_id = "0".to_string();
        let token = contract.mint(token_id, sample_token_metadata(), Some(accounts(2)));
        let owner_id = contract.get_owner_by_token_id(token.token_id).unwrap();
        assert_eq!(owner_id, accounts(2));
    }

    #[test]
    fn get_owner_by_id_invalid() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(0));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .build());
        let token_id = "0".to_string();
        let invalid_token_id = "1".to_string();
        contract.mint(token_id, sample_token_metadata(), None);

        assert!(contract.get_owner_by_token_id(invalid_token_id).is_none());
    }
}
