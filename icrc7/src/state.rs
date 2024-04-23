use crate::memory::{get_state_memory, get_token_map, get_txn_log, Memory};
use candid::{CandidType, Decode, Encode, Principal};
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
    pub mintable_flag: Option<bool>,
    pub burnable_flag: Option<bool>,
    pub burn_account: Option<Account>,
    pub icrc7_name: Option<String>,
    pub icrc7_symbol: Option<String>,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
    pub icrc7_max_query_batch_size: Option<u128>,
    pub icrc7_max_update_batch_size: Option<u128>,
    pub icrc7_default_take_value: Option<u128>,
    pub icrc7_max_take_value: Option<u128>,
    pub icrc7_max_memo_size: Option<u128>,
    pub icrc7_atomic_batch_transfer: Option<bool>, // defaults to false
    pub icrc7_tx_window: Option<u128>,
    pub icrc7_permitted_drift: Option<u128>,
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
            mintable_flag: None,
            burnable_flag: None,
            burn_account: None,
            icrc7_name: None,
            icrc7_symbol: None,
            icrc7_description: None,
            icrc7_logo: None,
            icrc7_supply_cap: None,
            icrc7_max_query_batch_size: None,
            icrc7_max_update_batch_size: None,
            icrc7_default_take_value: None,
            icrc7_max_take_value: None,
            icrc7_max_memo_size: None,
            icrc7_atomic_batch_transfer: None,
            icrc7_tx_window: None,
            icrc7_permitted_drift: None,
            txn_count: 0,
        }
    }
}

pub fn query_metadata<R>(f: impl FnOnce(&CollectionMetadata) -> R) -> R {
    COLLECTION_METADATA.with_borrow(|s| f(s.get()))
}

pub fn query_token_map<R>(f: impl FnOnce(&TokenMap) -> R) -> R {
    TOKEN_MAP.with_borrow(|map| f(map))
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

pub fn txn_deduplication_check(
    allowed_past_time: &u64,
    caller: &Account,
    arg: &TransferArg,
) -> Result<(), TransferError> {
    TXN_LOG.with_borrow(|log| {
        // getting the last recorded txn_id
        let mut count = query_metadata(|m| m.txn_count - 1);
        while count != 0 {
            let txn = log.get(&count).unwrap();
            // checking if the timestamp falls between allowed past time
            if txn.ts <= *allowed_past_time {
                return Ok(());
            }
            // checking if the timestamp and memo of the transaction are equal
            if Some(txn.ts) != arg.created_at_time && arg.memo != txn.memo {
                count -= 1;
                continue; // skipping to next iteration
            }
            if let Some(transfer) = txn.transfer {
                if arg.token_id == transfer.tid && arg.to == transfer.to && *caller == transfer.from
                {
                    return Err(TransferError::Duplicate {
                        duplicate_of: txn.txn_id,
                    });
                } else {
                    count -= 1;
                    continue; // skipping to next iteration
                }
            } else {
                count -= 1;
                continue;
            }
        }
        Ok(())
    })
}

pub fn log_transaction() -> u128 {
    let txn_id = get_txn_id();
    // TODO: logging of transaction
    txn_id
}

pub fn mock_transfer(caller: &Account, arg: &TransferArg) -> Result<(), TransferError> {
    if let Some(ref memo) = arg.memo {
        if memo.len() > 32 {
            return Err(TransferError::GenericError {
                error_code: 112,
                message: "Exceeds Max Memo Size".into(),
            });
        }
    }
    if let Some(time) = arg.created_at_time {
        // TODO: checking for time
        txn_deduplication_check(&0, caller, arg)?
    }
    if *caller == arg.to {
        return Err(TransferError::InvalidRecipient);
    }
    query_token_map(|map| {
        let token = match map.get(&arg.token_id) {
            None => return Err(TransferError::NonExistingTokenId),
            Some(token) => token,
        };
        if token.owner != *caller {
            return Err(TransferError::Unauthorized);
        }
        Ok(())
    })
}

pub fn execute_transfer(caller: Account, arg: TransferArg) -> u128 {
    todo!()
}

pub fn mock_mint() {}

pub fn execute_mint() {}

pub fn mock_burn() {}

pub fn execute_burn() {}

pub fn check_and_scale_storage_if_needed() {}

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static COLLECTION_METADATA: RefCell<StableCell<CollectionMetadata, Memory>> = RefCell::new(StableCell::new(get_state_memory(), CollectionMetadata::default()).unwrap());
    pub static TOKEN_MAP: RefCell<TokenMap> = RefCell::new(get_token_map());
    pub static TXN_LOG: RefCell<TxnLog> = RefCell::new(get_txn_log());
}
