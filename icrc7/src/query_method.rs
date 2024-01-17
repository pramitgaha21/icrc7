use ic_cdk_macros::query;
use icrc_ledger_types::icrc1::account::Account;

use crate::{
    icrc7_types::Icrc7CollectionMetadata,
    state::{Icrc7TokenId, Icrc7TokenMetadata, ICRC7_COLLECTION},
};

#[query]
pub fn icrc7_collection_metadata() -> Icrc7CollectionMetadata {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_collection_metadata())
}

#[query]
pub fn icrc7_name() -> String {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_name())
}

#[query]
pub fn icrc7_symbol() -> String {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_symbol())
}

#[query]
pub fn icrc7_royalties() -> Option<u16> {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_royalties())
}

#[query]
pub fn icrc7_royalty_recipient() -> Option<Account> {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_royalty_recipient())
}

#[query]
pub fn icrc7_description() -> Option<String> {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_description())
}

#[query]
pub fn icrc7_image() -> Option<String> {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_image())
}

#[query]
pub fn icrc7_total_supply() -> u128 {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_total_supply())
}

#[query]
pub fn icrc7_supply_cap() -> Option<u128> {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_supply_cap())
}

#[query]
pub fn icrc7_metadata(id: Icrc7TokenId) -> Icrc7TokenMetadata {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_metadata(&id))
}

#[query]
pub fn icrc7_owner_of(id: Icrc7TokenId) -> Account {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_owner_of(&id))
}

#[query]
pub fn icrc7_balance_of(account: Account) -> u128 {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_balance_of(&account))
}

#[query]
pub fn icrc7_tokens_of(account: Account) -> Vec<Icrc7TokenId> {
    ICRC7_COLLECTION.with(|c| c.borrow().icrc7_tokens_of(&account))
}
