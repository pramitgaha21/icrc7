use crate::memory::{get_state_memory, get_token_map, get_txn_log, Memory};
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{
    memory_manager::MemoryManager, storable::Bound, DefaultMemoryImpl, StableBTreeMap, StableCell,
    Storable,
};
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use icrc_nft_types::icrc7::transaction::Transaction;
use icrc_nft_types::icrc7::transfer::{TransferArg, TransferError};
use icrc_nft_types::{icrc7::metadata::Icrc7TokenMetadata, Account};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

pub type TokenMap = StableBTreeMap<u128, Token, Memory>;
pub type TxnLog = StableBTreeMap<u128, Transaction, Memory>;

/// The field of this structure is modifiable, you can add more fields according to your need
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: u128,
    pub owner: Account,
    pub name: String,
    pub logo: Option<String>,
    pub description: Option<String>,
}

impl Storable for Token {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Token {
    pub fn new(
        id: u128,
        owner: Account,
        name: String,
        logo: Option<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            id,
            owner,
            name,
            logo,
            description,
        }
    }

    pub fn transfer(&mut self, to: Account) {
        self.owner = to;
    }

    pub fn token_metadata(&self) -> Icrc7TokenMetadata {
        let mut metadata = HashMap::new();
        metadata.insert(
            "icrc7token:name".into(),
            MetadataValue::Text(self.name.clone()),
        );
        // TODO
        Icrc7TokenMetadata::from(metadata)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CollectionMetadata {
    pub minting_auth: Option<Account>, // when minting_auth is `None`: anyone can mint if allowed
    pub is_mintable: Option<bool>,
    pub is_burnable: Option<bool>,
    pub burn_account: Option<Account>,
    pub icrc7_name: Option<String>,
    pub icrc7_symbol: Option<String>,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
    pub txn_count: u128,
}

impl Storable for CollectionMetadata {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        ciborium::de::from_reader(&bytes[..]).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut buf = vec![];
        ciborium::ser::into_writer(self, &mut buf).expect("failed to Serialize");
        std::borrow::Cow::Owned(buf)
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for CollectionMetadata {
    fn default() -> Self {
        Self {
            minting_auth: None,
            is_mintable: None,
            is_burnable: None,
            burn_account: None,
            icrc7_name: None,
            icrc7_symbol: None,
            icrc7_description: None,
            icrc7_logo: None,
            icrc7_supply_cap: None,
            txn_count: 0,
        }
    }
}

pub fn query_metdata<R>(f: impl FnOnce(&CollectionMetadata) -> R) -> R {
    COLLECTION_METADATA.with_borrow(|s| f(s.get()))
}

pub fn get_txn_id() -> u128 {
    COLLECTION_METADATA.with_borrow_mut(|m| {
        let mut current_data = m.get().clone();
        let txn_id = current_data.txn_count;
        current_data.txn_count += 1;
        let _ = m.set(current_data);
        txn_id
    })
}

pub fn txn_deduplication_check(arg: TransferArg) -> Result<(), TransferError> {
    todo!()
}

pub fn log_transaction() {}

pub fn mock_transfer(arg: &TransferArg) -> Result<(), TransferError> {
    todo!()
}

pub fn execute_transfer(arg: TransferArg) -> u128 {
    todo!()
}

pub fn mock_mint() {}

pub fn execute_mint() {}

pub fn mock_burn() {}

pub fn execute_burn() {}

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static COLLECTION_METADATA: RefCell<StableCell<CollectionMetadata, Memory>> = RefCell::new(StableCell::new(get_state_memory(), CollectionMetadata::default()).unwrap());
    pub static TOKEN_MAP: RefCell<TokenMap> = RefCell::new(get_token_map());
    pub static TXN_LOG: RefCell<TxnLog> = RefCell::new(get_txn_log());
}
