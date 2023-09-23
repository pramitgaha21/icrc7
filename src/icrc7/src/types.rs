use candid::{CandidType, Deserialize};
use icrc_ledger_types::icrc1::account::{Account, Subaccount};

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
*/

#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    pub spender_subaccount: Option<Subaccount>,
    pub from: Account,
    pub to: Account,
    pub token_ids: Vec<u128>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub is_atomic: Option<bool>,
}

/*
type TransferError = variant {
    Unauthorized;
    TooOld;
    CreatedInFuture : record { ledger_time: nat64 };
    Duplicate : record { duplicate_of : nat };
    TemporarilyUnavailable;
    GenericError : record { error_code : nat; message : text };
};
*/

#[derive(CandidType)]
pub enum TransferError {
    Unauthorized { token_ids: Vec<u128> },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u128 },
    TemporarilyUnavailable,
    GenericError { error_code: u128, message: String },
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

#[derive(CandidType)]
pub struct Standard {
    pub name: String,
    pub url: String,
}

/*
type ApprovalArgs = record {
    from_subaccount : opt blob;
    spender : Account;    // Approval is given to an ICRC Account
    token_ids : variant { Collection; TokenIds: vec nat };
    expires_at : opt nat64;
    memo : opt blob;
    created_at_time : opt nat64;
};
*/

#[derive(CandidType, Deserialize)]
pub enum ApprovalType {
    Collection,
    TokenIds(Vec<u128>),
}

#[derive(CandidType, Deserialize)]
pub struct ApprovalArgs {
    pub from_subaaccount: Option<Subaccount>,
    pub spender: Account,
    pub token_ids: ApprovalType,
    pub expires_at: Option<u64>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

/*
type ApprovalError = variant {  // TO REVIEW
    Unauthorized;
    TooOld;
    TemporarilyUnavailable;
    GenericError : record { error_code : nat; message : text };
};
*/

#[derive(CandidType)]
pub enum ApprovalError {
    Unauthorized{ token_ids: Vec<u128> },
    TooOld,
    TemporarilyUnavailable,
    GenericError { error_code: u128, message: String },
}

/*
type RevokeError = variant {
    Unauthorized;
    ApprovalDoesNotExist;  // TBD: Ok or Error?
    ApprovalExpired;       // TBD: Ok or Error?
    TooOld;
    TemporarilyUnavailable;
    GenericError : record { error_code : nat; message : text };
};
*/

#[derive(CandidType)]
pub enum RevokeError {
    Unauthorized,
    ApprovalDoesNotExist,
    ApprovalExpired,
    TooOld,
    TemporarilyUnavailable,
    GenericError { error_code: u128, message: String },
}

#[derive(CandidType, Deserialize)]
pub struct MintArgs {
    pub token_ids: Vec<u128>,
    pub name: String,
    pub image: Option<String>,
    pub description: Option<String>,
    pub to: Option<Account>,
    pub memo: Option<Vec<u8>>,
}

#[derive(CandidType)]
pub enum MintError {
    Unauthorized,
    GenericError { error_code: u128, message: String },
}
