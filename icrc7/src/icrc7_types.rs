use candid::CandidType;
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use serde::Deserialize;

use crate::state::Icrc7TokenId;

#[derive(CandidType, Deserialize)]
pub struct InitArgs{
    pub minting_authority: Option<Account>,
    pub tx_window: Option<u64>,
    pub permitted_drift: Option<u64>,
    pub icrc7_name: String,
    pub icrc7_symbol: String,
    pub icrc7_royalties: Option<u16>,
    pub icrc7_royalty_recipient: Option<Account>,
    pub icrc7_description: Option<String>,
    pub icrc7_image: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
}

#[derive(CandidType, Deserialize)]
pub struct MintArgs{
    pub subaccount: Option<Subaccount>,
    pub to: Option<Account>,
    pub token_ids: Vec<Icrc7TokenId>,
    pub token_name: String,
    pub token_description: Option<String>,
    pub image: Option<String>,
    pub is_atomic: Option<bool>,
    pub memo: Option<Vec<u8>>,
}

#[derive(CandidType)]
pub enum MintError{
    Unauthorized{ minting_authority: Option<Account> },
    SupplyCapReached{ cap: u128 },
    TokenIdExist{ token_ids: Vec<Icrc7TokenId> },
    GenericError{ error_code: u128, message: String },
}

#[derive(CandidType)]
pub struct Icrc7CollectionMetadata {
    pub icrc7_name: String,
    pub icrc7_symbol: String,
    pub icrc7_royalties: Option<u16>,
    pub icrc7_royalty_recipient: Option<Account>,
    pub icrc7_description: Option<String>,
    pub icrc7_image: Option<String>,
    pub icrc7_total_supply: u128,
    pub icrc7_supply_cap: Option<u128>,
}

/*

type ApprovalArgs = record {
    from_subaccount : opt blob;
    spender : Account;    // Approval is given to an ICRC Account
    token_ids : opt vec nat;            // TBD: change into variant?
    expires_at : opt nat64;
    memo : opt blob;
    created_at_time : opt nat64;
};

type ApprovalError = variant {
    Unauthorized : vec nat;
    TooOld;
    TemporarilyUnavailable;
    GenericError : record { error_code : nat; message : text };
};

*/

#[derive(CandidType, Deserialize)]
pub struct ApprovalArgs {
    pub from_subaccount: Option<Subaccount>,
    pub spender: Account,
    pub token_ids: Option<Vec<Icrc7TokenId>>,
    pub expires_at: Option<u64>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType)]
pub enum ApprovalError {
    Unauthorized(Vec<Icrc7TokenId>),
    TooOld,
    TemporarilyUnavailable,
    GenericError { error_code: u128, message: String },
}

/*
type TransferArgs = record {
    spender_subaccount: opt blob; // the subaccount of the caller (used to identify the spender)
    from : Account;
    to : Account;
    token_ids : vec nat;
    // type: leave open for now
    memo : opt blob;
    created_at_time : opt nat64;
    is_atomic : opt bool;
};

type TransferError = variant {
    Unauthorized: record { token_ids : vec nat };
    TooOld;
    CreatedInFuture : record { ledger_time: nat64 };
    Duplicate : record { duplicate_of : nat };
    TemporarilyUnavailable;
    GenericError : record { error_code : nat; message : text };
};
*/

#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    pub spender_subaccount: Option<Subaccount>,
    pub from: Account,
    pub to: Account,
    pub token_ids: Vec<Icrc7TokenId>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub is_atomic: Option<bool>,
}

#[derive(CandidType)]
pub enum TransferError {
    Unauthorized { token_ids: Vec<Icrc7TokenId> },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u128 },
    TemporarilyUnavailable,
    GenericError { error_code: u128, message: String },
}
