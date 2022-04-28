// DOCUMENTATION
// NEP - 177 Standar
// DOC: https://nomicon.io/Standards/Tokens/NonFungibleToken/Metadata

use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC };
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;

use near_sdk::json_types::Base64VecU8;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

const ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId, name: String, symbol: String) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        Self {
            token: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval)
            ),
            metadata: LazyOption::new(
                StorageKey::Metadata,
                Some(&NFTContractMetadata {
                    spec: NFT_METADATA_SPEC.to_string(),
                    name: name,
                    symbol: symbol,
                    icon: Some(ICON.to_string()),
                    base_uri: Some("https://nft.storage/".to_string()),
                    reference: None,
                    reference_hash: None,
                    }
                )
            ),
        }
    }
    
    #[payable]
    pub fn nft_mint(&mut self, token_id: TokenId, receiver_id: ValidAccountId, title: String, description: String, media: String, hash: Base64VecU8) -> Token {
        let token_metadata = generate_token_meta_data(title, description, media, hash);
        self.token.mint(token_id, receiver_id, Some(token_metadata))
    }
}

pub fn generate_token_meta_data(title: String, description: String, media: String, hash: Base64VecU8) -> TokenMetadata{
    TokenMetadata {
        title: Some(title),
        description: Some(description),
        media: Some(media),
        media_hash: Some(hash),
        copies: Some(1),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None, 
        reference: None,
        reference_hash: None 
    }
}

// Mandatories lines to implement Standar NEP-171 NFT!
near_contract_standards::impl_non_fungible_token_core!(Contract, token);
near_contract_standards::impl_non_fungible_token_approval!(Contract, token);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, token);


#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::json_types::ValidAccountId;

    use std::convert::TryInto;

    fn get_context(current_account_id: String, input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: current_account_id,
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn validate_owner() {
        let current_account_id = String::from("eduardogpg.testnet");
        let context = get_context(current_account_id, vec![], false);
        testing_env!(context);
    }
    
    #[test]
    fn validate_token_metadata() { // https://docs.rs/near-contract-standards/3.2.0/near_contract_standards/non_fungible_token/metadata/struct.TokenMetadata.html
        let title = String::from("NFT Title");
        let description = String::from("NFT Description");
        let media = String::from("NFT Media");
        let hash = Base64VecU8::from( media.as_bytes().to_vec() );

        let token_metadata =  generate_token_meta_data(title, description, media, hash);

        assert_eq!(String::from("NFT Title"), token_metadata.title.unwrap());
        assert_eq!(String::from("NFT Description"), token_metadata.description.unwrap());
        assert_eq!(String::from("NFT Media"), token_metadata.media.unwrap());
    }

    #[test]
    fn validate_token_metadata_copies() {
        let title = String::from("NFT Title");
        let description = String::from("NFT Description");
        let media = String::from("NFT Media");
        let hash = Base64VecU8::from( media.as_bytes().to_vec() );

        let token_metadata =  generate_token_meta_data(title, description, media, hash);

        assert_eq!(1, token_metadata.copies.unwrap());
    }
    
    #[test]
    fn validate_non_fungible_token() {
        let owner_id: ValidAccountId = "eduardogpg.testnet".try_into().unwrap();
        assert_eq!("eduardogpg.testnet", owner_id.to_string());
    }
    
    #[test]
    fn validate_contract_owner() {
        let current_account_id = String::from("eduardogpg.testnet");
        let context = get_context(current_account_id, vec![], false);
        testing_env!(context);

        let symbol = String::from("WEL");
        let name = String::from("Welcome");
        let owner_id: ValidAccountId = "eduardogpg.testnet".try_into().unwrap();

        let contract = Contract::new(owner_id, name, symbol);
        
        assert_eq!("eduardogpg.testnet", contract.token.owner_id.to_string());
    }
}
