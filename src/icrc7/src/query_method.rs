use std::collections::HashMap;

use ic_cdk_macros::query;
use icrc_ledger_types::{icrc1::account::Account, icrc::generic_metadata_value::MetadataValue};

use crate::{
    state::COLLECTION,
    types::{Icrc7CollectionMetadata, Standard},
};

#[query]
pub fn icrc7_collection_metadata() -> Icrc7CollectionMetadata {
    COLLECTION.with(|c| c.borrow().icrc7_collection_metadata())
}

#[query]
pub fn icrc7_name() -> String {
    COLLECTION.with(|c| c.borrow().icrc7_name())
}

#[query]
pub fn icrc7_symbol() -> String {
    COLLECTION.with(|c| c.borrow().icrc7_symbol())
}

#[query]
pub fn icrc7_suggested_royalties() -> Option<u16> {
    COLLECTION.with(|c| c.borrow().icrc7_royalties())
}

#[query]
pub fn icrc7_royalty_recipient() -> Option<Account> {
    COLLECTION.with(|c| c.borrow().icrc7_royalty_recipient())
}

#[query]
pub fn icrc7_description() -> Option<String> {
    COLLECTION.with(|c| c.borrow().icrc7_description())
}

#[query]
pub fn icrc7_total_supply() -> u128 {
    COLLECTION.with(|c| c.borrow().icrc7_total_supply())
}

#[query]
pub fn icrc7_supply_cap() -> Option<u128> {
    COLLECTION.with(|c| c.borrow().icrc7_supply_cap())
}

#[query]
pub fn icrc7_metadata(id: u128) -> HashMap<String, MetadataValue>{
    COLLECTION.with(|c|{
        c.borrow().icrc7_token_metadata(&id)
    })
}

#[query]
pub fn icrc7_owner_of(id: u128) -> Option<Account> {
    COLLECTION.with(|c|{
        c.borrow().owner_of(&id)
    })
}

#[query]
pub fn icrc7_balance_of(account: Account) -> u128 {
    COLLECTION.with(|c|{
        c.borrow().balance_of(&account)
    })
}

#[query]
pub fn icrc7_tokens_of(account: Account) -> Vec<u128>{
    COLLECTION.with(|c|{
        c.borrow().icrc7_tokens_of(&account)
    })
}

#[query]
pub fn icrc7_supported_standards() -> Vec<Standard> {
    vec![Standard {
        name: "ICRC-7".to_string(),
        url: "https://github.com/dfinity/ICRC/ICRCs/ICRC-7".to_string(),
    }]
}
