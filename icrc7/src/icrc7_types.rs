use std::collections::HashMap;

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use icrc_ledger_types::{
    icrc::generic_metadata_value::MetadataValue,
    icrc1::account::{Account, Subaccount},
};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum TransactionType {
    Mint {
        tid: u128,
        from: Account,
        to: Account,
        meta: MetadataValue,
    },
    Burn {
        tid: u128,
        from: Account,
        to: Account,
    },
    Transfer {
        tid: u128,
        from: Account,
        to: Account,
    },
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub ts: u64,
    pub txn_id: u128,
    pub op: String,
    pub txn_type: TransactionType,
    pub memo: Option<Vec<u8>>,
}

impl Storable for Transaction {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Transaction {
    pub fn new(txn_id: u128, txn_type: TransactionType, ts: u64, memo: Option<Vec<u8>>) -> Self {
        let op = match &txn_type {
            TransactionType::Transfer {
                tid: _,
                from: _,
                to: _,
            } => "7xfer".into(),
            TransactionType::Mint {
                tid: _,
                from: _,
                to: _,
                meta: _,
            } => "7mint".into(),
            TransactionType::Burn {
                tid: _,
                from: _,
                to: _,
            } => "7burn".into(),
        };
        Self {
            op,
            txn_id,
            ts,
            txn_type,
            memo,
        }
    }
}

/*
TransferArg = record {
    from_subaccount: opt blob; // the subaccount to transfer the token from
    to : Account;
    token_id : nat;
    // type: leave open for now
    memo : opt blob;
    created_at_time : opt nat64;
};

type TransferResult = variant {
    Ok : nat; // Transaction indices for successful transfers
    Err : TransferError;
};

type TransferError = variant {
    NonExistingTokenId;
    InvalidRecipient;
    Unauthorized;
    TooOld;
    CreatedInFuture : record { ledger_time: nat64 };
    Duplicate : record { duplicate_of : nat };
    GenericError : record { error_code : nat; message : text };
    GenericBatchError : record { error_code : nat; message : text };
};
*/

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TransferError {
    NonExistingTokenId,
    InvalidRecipient,
    Unauthorized,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u128 },
    GenericError { error_code: u128, message: String },
    GenericBatchError { error_code: u128, message: String },
}

pub type TransferResult = Result<u128, TransferError>;

pub type Icrc7TokenMetadata = HashMap<String, MetadataValue>;

#[derive(CandidType, Deserialize, Clone)]
pub struct MintArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
    // if None, then the combination of Collection's symbol and token's id will be provided
    // for e.g.: "ICRC7 100"
    pub token_name: Option<String>,
    pub token_description: Option<String>,
    pub token_logo: Option<String>,
}

#[derive(CandidType, Clone)]
pub enum MintError {
    SupplyCapReached,
    Unauthorized,
    TokenIdAlreadyExist,
    GenericError { error_code: u128, message: String },
    GenericBatchError { error_code: u128, message: String },
}

pub type MintResult = Result<u128, MintError>;

#[derive(CandidType, Deserialize, Clone)]
pub struct BurnArg {
    pub from_subaccount: Option<Subaccount>,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
}

#[derive(CandidType, Clone)]
pub enum BurnError {
    Unauthorized,
    NonExistingTokenId,
    GenericError { error_code: u128, message: String },
    GenericBatchError { error_code: u128, message: String },
}

pub type BurnResult = Result<u128, BurnError>;

#[derive(CandidType, Deserialize)]
pub struct InitArg {
    pub minting_account: Option<Account>,
    pub icrc7_symbol: String,
    pub icrc7_name: String,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
    pub icrc7_max_query_batch_size: Option<u128>,
    pub icrc7_max_update_batch_size: Option<u128>,
    pub icrc7_max_take_value: Option<u128>,
    pub icrc7_default_take_value: Option<u128>,
    pub icrc7_max_memo_size: Option<u128>,
    pub icrc7_atomic_batch_transfers: Option<bool>,
    pub tx_window: Option<u64>,
    pub permitted_drift: Option<u64>,
}

#[derive(CandidType)]
pub struct Standard {
    pub name: String,
    pub url: String,
}
