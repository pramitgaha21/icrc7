pub mod memory;
pub mod state;
pub mod utils;

use ic_cdk::query;
use state::COLLECTION_METADATA;

pub fn init() {}

#[query]
pub fn icrc7_name() -> String {
    COLLECTION_METADATA.with_borrow(|s| s.get().icrc7_name.as_ref().unwrap().clone())
}

#[query]
pub fn icrc7_symbol() -> String {
    COLLECTION_METADATA.with_borrow(|s| s.get().icrc7_symbol.as_ref().unwrap().clone())
}
