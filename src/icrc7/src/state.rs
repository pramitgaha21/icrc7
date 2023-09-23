use std::{cell::RefCell, collections::HashMap};

use candid::{CandidType, Decode, Encode, Principal, Nat};
use ic_stable_structures::{
    memory_manager::{MemoryManager, VirtualMemory},
    storable::{Bound, Storable},
    DefaultMemoryImpl, StableBTreeMap,
};
use icrc_ledger_types::{icrc1::account::{Account, DEFAULT_SUBACCOUNT}, icrc::generic_metadata_value::MetadataValue};
use serde::{Deserialize, Serialize};

use crate::{
    types::{
        ApprovalArgs, ApprovalError, Icrc7CollectionMetadata, MintArgs, MintError, TransferArgs,
        TransferError, RevokeError, ApprovalType,
    },
    utils::default_account_from_principal, get_token_stable_memory, get_log_stable_memory, account_transformer,
};

type Memory = VirtualMemory<DefaultMemoryImpl>;

#[derive(CandidType, Serialize, Deserialize)]
pub enum TransactionType {
    Transfer {
        from: Account,
        spender: Account,
        to: Account,
    },
    Approval {
        spender: Account,
    },
    Mint {
        to: Account,
    },
}

impl Storable for TransactionType {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 2,
        is_fixed_size: true,
    };
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct Transaction {
    pub token_id: Vec<u128>,
    pub txn_type: TransactionType,
    pub created_at: u64,
    pub memo: Option<Vec<u8>>,
}

impl Storable for Transaction {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 24 + 2 + 8 + 64 + 64 + 64,
        is_fixed_size: true,
    };
}

#[derive(CandidType, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Approval {
    pub expires_at: Option<u64>,
    pub account: Account,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum ApprovalResult {
    Approved,
    Expired,
    NotApproved,
}

impl Storable for Approval {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 10 + 64,
        is_fixed_size: true,
    };
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct Token {
    pub owner: Account,
    pub id: u128,
    pub image: Option<String>,
    pub description: Option<String>,
    pub name: String,
    pub approvals: Vec<Approval>,
}

impl Storable for Token {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Token {
    pub fn approval_check(&self, account: &Account, current_time: u64) -> ApprovalResult {
        for approval in self.approvals.iter() {
            if approval.account == *account {
                if approval.expires_at.is_none() || approval.expires_at <= Some(current_time) {
                    return ApprovalResult::Approved;
                } else {
                    return ApprovalResult::Expired;
                }
            }
        }
        ApprovalResult::NotApproved
    }

    pub fn approve(&mut self, args: &ApprovalArgs) {
        if let Some(index) = self.approvals.iter().position(|a| a.account == args.spender){
            let approval = Approval{
                expires_at: args.expires_at,
                account: args.spender
            };
            self.approvals.insert(index, approval);
        }else{
            let approval = Approval{
                expires_at: args.expires_at,
                account: args.spender
            };
            self.approvals.push(approval);
        }
    }

    pub fn transfer(&mut self, to: Account) {
        self.owner = to;
        self.approvals.clear();
    }

    pub fn token_metadata(&self) -> HashMap<String, MetadataValue>{
        let mut map = HashMap::new();
        map.insert("Id".to_string(), MetadataValue::Nat(Nat::from(self.id)));
        map.insert("Name".to_string(), MetadataValue::Text(self.name.clone()));
        if let Some(ref image) = self.image{
            map.insert("Image".to_string(), MetadataValue::Text(image.clone()));
        }
        if let Some(ref description) = self.description{
            map.insert("Description".to_string(), MetadataValue::Text(description.clone()));
        }
        map
    }
}

#[derive(Serialize, Deserialize)]
pub struct Collection {
    pub minting_authority: Principal,
    pub tx_window: u64,
    pub permitted_drift: u64,
    pub icrc7_name: String,
    pub icrc7_symbol: String,
    pub icrc7_royalties: Option<u16>,
    pub icrc7_royalty_recipient: Option<Account>,
    pub icrc7_description: Option<String>,
    pub icrc7_image: Option<String>,
    pub icrc7_total_supply: u128,
    pub icrc7_supply_cap: Option<u128>,
    #[serde(skip, default = "get_token_stable_memory")]
    pub tokens: StableBTreeMap<u128, Token, Memory>,
    pub tx_count: u128,
    #[serde(skip, default = "get_log_stable_memory")]
    pub tx_log: StableBTreeMap<u128, Transaction, Memory>,
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            minting_authority: Principal::from_slice(&[]),
            permitted_drift: 24_000,
            tx_window: 1_000,
            icrc7_name: "Icrc7 Token".to_string(),
            icrc7_symbol: "ICRC7".to_string(),
            icrc7_description: Some("Icrc7 Token".to_string()),
            icrc7_image: None,
            icrc7_royalties: None,
            icrc7_royalty_recipient: None,
            icrc7_total_supply: 0,
            icrc7_supply_cap: None,
            tokens: get_token_stable_memory(),
            tx_count: 0,
            tx_log: get_log_stable_memory(),
        }
    }
}

pub enum TransactionDeduplicationType {
    NotFound,
    DuplicateOf { index: u128 },
}

impl Collection {
    pub fn icrc7_collection_metadata(&self) -> Icrc7CollectionMetadata {
        Icrc7CollectionMetadata {
            icrc7_name: self.icrc7_name.clone(),
            icrc7_symbol: self.icrc7_symbol.clone(),
            icrc7_royalties: self.icrc7_royalties,
            icrc7_royalty_recipient: self.icrc7_royalty_recipient.clone(),
            icrc7_description: self.icrc7_description.clone(),
            icrc7_image: self.icrc7_image.clone(),
            icrc7_total_supply: self.icrc7_total_supply,
            icrc7_supply_cap: self.icrc7_supply_cap,
        }
    }

    pub fn icrc7_name(&self) -> String {
        self.icrc7_name.clone()
    }

    pub fn icrc7_symbol(&self) -> String {
        self.icrc7_symbol.clone()
    }

    pub fn icrc7_description(&self) -> Option<String> {
        self.icrc7_description.clone()
    }

    pub fn icrc7_image(&self) -> Option<String> {
        self.icrc7_image.clone()
    }

    pub fn icrc7_royalties(&self) -> Option<u16> {
        self.icrc7_royalties.clone()
    }

    pub fn icrc7_royalty_recipient(&self) -> Option<Account> {
        self.icrc7_royalty_recipient.clone()
    }

    pub fn icrc7_total_supply(&self) -> u128 {
        self.icrc7_total_supply
    }

    pub fn icrc7_supply_cap(&self) -> Option<u128> {
        self.icrc7_supply_cap
    }

    fn log_transaction(
        &mut self,
        token_id: Vec<u128>,
        txn_type: TransactionType,
        memo: Option<Vec<u8>>,
    ) -> u128 {
        let time = ic_cdk::api::time();
        ic_cdk::println!("{}", time);
        self.tx_count += 1;
        let txn = Transaction {
            token_id,
            txn_type,
            created_at: time,
            memo,
        };
        self.tx_log.insert(self.tx_count, txn);
        self.tx_count
    }

    fn transfer_txn_deduplication_check(
        &self,
        current_time: u64,
        created_at_time: u64,
        token_id: &Vec<u128>,
        _from: Account,
        _spender: Account,
        _to: Account,
        memo: &Option<Vec<u8>>,
    ) -> TransactionDeduplicationType {
        let allowed_past_time = current_time - self.tx_window;
        let mut txn_index = self.tx_count;
        while txn_index != 0 {
            let txn = self.tx_log.get(&txn_index).unwrap();
            if txn.token_id != *token_id {
                txn_index -= 1;
                continue;
            }
            if txn.created_at < allowed_past_time {
                return TransactionDeduplicationType::NotFound;
            }
            match txn.txn_type {
                TransactionType::Transfer { from, spender, to } => {
                    if txn.created_at == created_at_time
                        && &txn.memo == memo
                        && from == _from
                        && spender == _spender
                        && to == _to
                    {
                        return TransactionDeduplicationType::DuplicateOf { index: txn_index };
                    }
                    txn_index -= 1;
                    continue;
                }
                _ => {
                    txn_index -= 1;
                    continue;
                }
            }
        }
        TransactionDeduplicationType::NotFound
    }

    pub fn icrc7_mint(&mut self, caller: &Principal, args: MintArgs) -> Result<u128, MintError> {
        // checking if there is supply cap
        if let Some(supply) = self.icrc7_supply_cap {
            // checking if the supply cap is reached or not
            if supply == self.icrc7_total_supply {
                return Err(MintError::GenericError {
                    error_code: 1,
                    message: "Supply cap Reached".to_string(),
                });
            }
        }
        // checking if the caller is the minting authority
        if *caller != self.minting_authority {
            return Err(MintError::Unauthorized);
        }
        let owner = match args.to {
            Some(receiver) => account_transformer(receiver),
            None => default_account_from_principal(caller.clone()),
        };
        for id in args.token_ids.iter() {
            // checking if the token id is already minted or not
            if let Some(_) = self.tokens.get(id) {
                return Err(MintError::GenericError {
                    error_code: 2,
                    message:  "Token Id Already Minted".to_string(),
                });
            }

            let token = Token {
                owner: owner.clone(),
                id: id.clone(),
                image: args.image.clone(),
                description: args.description.clone(),
                name: args.name.clone(),
                approvals: vec![]
            };
            self.tokens.insert(id.clone(), token);
        }
        let txn_id = self.log_transaction(
            args.token_ids,
            TransactionType::Mint { to: owner.clone() },
            args.memo,
        );
        self.icrc7_total_supply += 1;
        Ok(txn_id)
    }

    pub fn owner_of(&self, token_id: &u128) -> Option<Account> {
        if let Some(token) = self.tokens.get(token_id) {
            Some(token.owner)
        } else {
            None
        }
    }

    pub fn balance_of(&self, account: &Account) -> u128 {
        let mut count = 0;
        for (_, ref token) in self.tokens.iter() {
            if token.owner == *account {
                count += 1;
            }
        }
        count
    }

    pub fn icrc7_tokens_of(&self, account: &Account) -> Vec<u128>{
        let mut tokens = vec![];
        for (_, ref token) in self.tokens.iter(){
            if token.owner == *account{
                tokens.push(token.id)
            }
        }
        tokens
    }

    pub fn icrc7_token_metadata(&self, id: &u128) -> HashMap<String, MetadataValue>{
        match self.tokens.get(id){
            None => HashMap::new(),
            Some(ref token) => token.token_metadata()
        }
    }

    pub fn icrc7_transfer(
        &mut self,
        caller: &Principal,
        mut args: TransferArgs,
    ) -> Result<u128, TransferError> {
        if args.token_ids.len() == 0 {
            return Err(TransferError::GenericError {
                error_code: 2,
                message: "Empty Token Ids".to_string(),
            });
        }
        args.token_ids.sort();
        args.from = account_transformer(args.from);
        args.to = account_transformer(args.to);
        let caller = Account {
            owner: caller.clone(),
            subaccount: match args.spender_subaccount{
                None => Some(DEFAULT_SUBACCOUNT.clone()),
                Some(subaccount) => Some(subaccount)
            },
        };
        let current_time = ic_cdk::api::time();
        if let Some(time) = args.created_at_time {
            if time < (current_time - self.tx_window - self.permitted_drift) {
                return Err(TransferError::TooOld);
            } else if time > (current_time + self.permitted_drift) {
                return Err(TransferError::CreatedInFuture {
                    ledger_time: current_time,
                });
            }
            // transaction deduplication check
            if let TransactionDeduplicationType::DuplicateOf { index } = self
                .transfer_txn_deduplication_check(
                    current_time,
                    time,
                    &args.token_ids,
                    args.from,
                    caller,
                    args.to,
                    &args.memo,
                )
            {
                return Err(TransferError::Duplicate {
                    duplicate_of: index,
                });
            }
        }
        let mut unauthorized = vec![];
        for id in args.token_ids.iter() {
            if let Some(token) = self.tokens.get(&id) {
                let approval_check;
                match token.approval_check(&caller, current_time) {
                    ApprovalResult::Approved => {
                        approval_check = true;
                    },
                    _ => {
                        approval_check = false;
                    },
                }
                if token.owner != args.from && !approval_check {
                    unauthorized.push(id.clone())
                }
            } else {
                unauthorized.push(id.clone())
            }
        }
        if let Some(false) = args.is_atomic {
            for id in args.token_ids.iter(){
                if unauthorized.contains(id){
                    return Err(TransferError::Unauthorized { token_ids: unauthorized })
                }
                let mut token = self.tokens.get(id).unwrap();
                token.transfer(args.to);
                self.tokens.insert(id.clone(), token);
            }
            let txn_id = self.log_transaction(args.token_ids.clone(), TransactionType::Transfer { from: args.from, spender: caller, to: args.to }, args.memo.clone());
            return Ok(txn_id)
        } else {
            if unauthorized.len() > 0 {
                return Err(TransferError::Unauthorized {
                    token_ids: unauthorized,
                });
            }
            for id in args.token_ids.iter(){
                let mut token = self.tokens.get(id).unwrap();
                token.transfer(args.to);
                self.tokens.insert(id.clone(), token);
            }
            let txn_id = self.log_transaction(args.token_ids.clone(), TransactionType::Transfer { from: args.from, spender: caller, to: args.to }, args.memo.clone());
            return Ok(txn_id)
        }
    }

    pub fn icrc7_approve(&mut self, caller: &Principal, mut args: ApprovalArgs) -> Result<u128, ApprovalError> {
        // generating the caller account combining the caller's principal with subaccount provided in the args
        let caller_account = Account {
            owner: caller.clone(),
            subaccount: match args.from_subaaccount{ // checking if the variant is None, if it's None then changing it to DEFAULT SUBACCOUNT
                None => Some(DEFAULT_SUBACCOUNT.clone()),
                Some(subaccount) => Some(subaccount)
            },
        };
        args.spender = account_transformer(args.spender);
        // collecting the tokens owned by the caller's account
        let mut tokens = self.icrc7_tokens_of(&caller_account);
        let mut unauthorized = vec![];
        // checking if the caller's account owns any token in the collection
        if tokens.len() == 0{
            return Err(ApprovalError::GenericError { error_code: 3, message: "No Token Owned".to_string() })
        }
        match args.token_ids{
            ApprovalType::Collection => {
                for id in tokens.iter(){
                    let mut token = self.tokens.get(&id).unwrap();
                    token.approve(&args);
                    self.tokens.insert(*id, token);
                }
            },
            ApprovalType::TokenIds(ref ids) => {
                let mut ids = ids.clone();
                ids.sort();
                for id in ids.iter(){
                    if !tokens.contains(&id){
                        unauthorized.push(*id);
                        continue;
                    }
                    if let Some(mut token) = self.tokens.get(&id){
                        token.approve(&args);
                        self.tokens.insert(*id, token);
                    }else{
                        unauthorized.push(*id)
                    }
                }
                tokens = ids.clone();
            }
        }
        if unauthorized.len() != 0{
            return Err(ApprovalError::Unauthorized { token_ids: unauthorized })
        }
        let txn_id = self.log_transaction(tokens, TransactionType::Approval { spender: args.spender }, args.memo.clone());
        Ok(txn_id)
    }

    pub fn revoke_approval(&mut self, caller: &Principal) -> Result<u128, RevokeError>{
        todo!()
    }
}

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static COLLECTION: RefCell<Collection> = RefCell::default();
}
