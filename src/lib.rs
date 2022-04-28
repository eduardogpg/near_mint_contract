// DOCUMENTATION
// NEP - 177 Standar
// https://nomicon.io/Standards/Tokens/NonFungibleToken/Metadata

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;

use near_sdk::json_types::Base64VecU8;
use near_sdk::json_types::ValidAccountId;

use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, PromiseOrValue,  
};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

near_contract_standards::impl_non_fungible_token_core!(Contract, token);

const ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M18.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

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
                })
            ),
        }
    }
    
    #[payable]
    pub fn nft_mint(&mut self, token_id: TokenId, receiver_id: ValidAccountId, 
        title: String, description: String, media: String, hash: Base64VecU8) -> Token {

        let token_metadata = generate_token_meta_data(title, description, media, hash, 1);
        self.token.mint(token_id, receiver_id, Some(token_metadata))
    }
}

fn generate_token_meta_data(title: String, description: String, media: String, hash: Base64VecU8, copies: u64) -> TokenMetadata{
    
    TokenMetadata {
        title: Some(title),
        description: Some(description),
        media: Some(media),
        media_hash: Some(hash),
        copies: Some(copies),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None, 
        reference: None,
        reference_hash: None 
    }
}
