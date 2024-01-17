use std::{cell::RefCell, collections::HashMap};

use crate::{
    icrc7_types::{
        ApprovalArgs, ApprovalError, Icrc7CollectionMetadata, TransferArgs, TransferError, MintArgs, MintError,
    },
    memory::{get_log_memory, get_token_map_memory, Memory},
    utils::account_transformer,
};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{
    memory_manager::MemoryManager, storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable,
};
use icrc_ledger_types::{icrc::generic_metadata_value::MetadataValue, icrc1::account::Account};
use serde::Serialize;

pub type Icrc7TokenId = u128;
pub type Icrc7TokenMetadata = HashMap<String, MetadataValue>;

#[derive(CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Icrc7TokenApproval {
    pub approved_for: Account,
    pub expires_at: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct Icrc7Token {
    pub token_owner: Account,
    pub token_id: Icrc7TokenId,
    pub token_name: String,
    pub token_image: Option<String>,
    pub token_description: Option<String>,
    pub approvals: Vec<Icrc7TokenApproval>,
}

impl Storable for Icrc7Token {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(&self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Icrc7Token {
    pub fn new(
        token_owner: Account,
        token_id: Icrc7TokenId,
        token_name: String,
        token_image: Option<String>,
        token_description: Option<String>,
    ) -> Self {
        Self {
            token_owner,
            token_id,
            token_name,
            token_image,
            token_description,
            approvals: Vec::new(),
        }
    }

    pub fn clear_expired_approvals(&mut self, current_time: &u64) {
        self.approvals.retain(|approval| {
            approval.expires_at == None || approval.expires_at >= Some(current_time.clone())
        })
    }

    pub fn approve(&mut self, args: &ApprovalArgs) {
        let spender = account_transformer(args.spender.clone());
        if let Some(index) = self
            .approvals
            .iter()
            .position(|a| a.approved_for == spender)
        {
            let approval = Icrc7TokenApproval {
                expires_at: args.expires_at,
                approved_for: spender,
            };
            self.approvals.insert(index, approval);
        } else {
            let approval = Icrc7TokenApproval {
                expires_at: args.expires_at,
                approved_for: spender,
            };
            self.approvals.push(approval);
        }
    }

    pub fn approval_check(&self, approved_for: &Account) -> bool {
        self.approvals
            .iter()
            .any(|approval| approval.approved_for == *approved_for)
    }

    pub fn transfer(&mut self, to: &Account) {
        self.approvals.clear();
        self.token_owner = to.clone();
    }

    pub fn token_metadata(&self) -> Icrc7TokenMetadata {
        let mut metadata = HashMap::<String, MetadataValue>::new();
        metadata.insert("Name".into(), MetadataValue::Text(self.token_name.clone()));
        metadata.insert(
            "Symbol".into(),
            MetadataValue::Text(self.token_name.clone()),
        );
        metadata
    }
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum TransactionType {
    Transfer {
        from: Account,
        spender: Account,
        to: Account,
    },
    Approval {
        from: Account,
        spender: Account,
    },
    Mint {
        to: Account,
    },
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct Transaction {
    pub txn_type: TransactionType,
    pub token_ids: Vec<Icrc7TokenId>,
    pub created_at_time: u64,
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
    pub fn new(
        txn_type: TransactionType,
        token_ids: Vec<Icrc7TokenId>,
        created_at_time: u64,
        memo: Option<Vec<u8>>,
    ) -> Self {
        Self {
            txn_type,
            token_ids,
            created_at_time,
            memo,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Icrc7Collection {
    pub minting_authority: Option<Account>,
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
    #[serde(skip, default = "get_token_map_memory")]
    pub icrc7_tokens: StableBTreeMap<Icrc7TokenId, Icrc7Token, Memory>,
    pub txn_count: u128,
    #[serde(skip, default = "get_log_memory")]
    pub txn_log: StableBTreeMap<u128, Transaction, Memory>,
}

impl Default for Icrc7Collection {
    fn default() -> Self {
        Self {
            tx_window: 24 * 60 * 60 * 60 * 1000_000_000,
            permitted_drift: 2 * 60 * 1000_000_000,
            minting_authority: None,
            icrc7_name: "Icrc7 Collection".into(),
            icrc7_symbol: "ICRC7".into(),
            icrc7_royalties: None,
            icrc7_royalty_recipient: None,
            icrc7_description: None,
            icrc7_image: None,
            icrc7_total_supply: 0,
            icrc7_supply_cap: None,
            icrc7_tokens: get_token_map_memory(),
            txn_count: 1,
            txn_log: get_log_memory(),
        }
    }
}

impl Icrc7Collection {
    fn transfer_deduplication_check(
        &self,
        current_time: &u64,
        caller: &Account,
        args: &TransferArgs,
    ) -> Result<(), TransferError> {
        let mut count = self.txn_count - 1;
        let allowed_past_time = *current_time - self.tx_window - self.permitted_drift;
        while count != 0 {
            let txn = self.txn_log.get(&count).unwrap();
            if txn.created_at_time < allowed_past_time{
                return Ok(())
            }
            match txn.txn_type {
                TransactionType::Transfer { from, spender, to } => {
                    if args.token_ids == txn.token_ids
                        && args.memo == txn.memo
                        && args.from == from
                        && args.to == to
                        && *caller == spender
                        && args.created_at_time == Some(txn.created_at_time)
                    {
                        return Err(TransferError::Duplicate {
                            duplicate_of: count,
                        });
                    } else {
                        count -= 1;
                        continue;
                    }
                }
                _ => {
                    count -= 1;
                    continue;
                }
            }
        }
        Ok(())
    }

    fn log_transaction(&mut self, txn: Transaction) -> u128 {
        ic_cdk::println!("transaction time: {}", txn.created_at_time);
        let txn_id = self.txn_count;
        self.txn_log.insert(txn_id, txn);
        self.txn_count += 1;
        txn_id
    }

    pub fn icrc7_collection_metadata(&self) -> Icrc7CollectionMetadata {
        Icrc7CollectionMetadata {
            icrc7_name: self.icrc7_name.clone(),
            icrc7_symbol: self.icrc7_symbol.clone(),
            icrc7_royalties: self.icrc7_royalties.clone(),
            icrc7_royalty_recipient: self.icrc7_royalty_recipient.clone(),
            icrc7_description: self.icrc7_description.clone(),
            icrc7_image: self.icrc7_image.clone(),
            icrc7_total_supply: self.icrc7_total_supply.clone(),
            icrc7_supply_cap: self.icrc7_supply_cap.clone(),
        }
    }

    pub fn icrc7_name(&self) -> String {
        self.icrc7_name.clone()
    }

    pub fn icrc7_symbol(&self) -> String {
        self.icrc7_symbol.clone()
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

    pub fn icrc7_description(&self) -> Option<String> {
        self.icrc7_description.clone()
    }

    pub fn icrc7_total_supply(&self) -> u128 {
        self.icrc7_total_supply.clone()
    }

    pub fn icrc7_supply_cap(&self) -> Option<u128> {
        self.icrc7_supply_cap.clone()
    }

    pub fn icrc7_metadata(&self, id: &Icrc7TokenId) -> Icrc7TokenMetadata {
        if let Some(token) = self.icrc7_tokens.get(id) {
            token.token_metadata()
        } else {
            ic_cdk::trap("Invalid Token Id")
        }
    }

    pub fn icrc7_owner_of(&self, id: &Icrc7TokenId) -> Account {
        if let Some(token) = self.icrc7_tokens.get(id) {
            token.token_owner.clone()
        } else {
            ic_cdk::trap("Invalid Token Id")
        }
    }

    pub fn icrc7_tokens_of(&self, account: &Account) -> Vec<Icrc7TokenId> {
        let mut ids = vec![];
        for (id, ref token) in self.icrc7_tokens.iter() {
            if token.token_owner == *account {
                ids.push(id)
            }
        }
        ids
    }

    pub fn icrc7_balance_of(&self, account: &Account) -> u128 {
        self.icrc7_tokens_of(account).len() as u128
    }

    pub fn icrc7_mint(&mut self, caller: &Principal, args: MintArgs) -> Result<u128, MintError>{
        if args.token_ids.len() == 0{
            return Err(MintError::GenericError { error_code: 1, message: "Empty Token Ids".into() })
        }
        let caller = account_transformer(Account{
            owner: caller.clone(),
            subaccount: args.subaccount,
        });
        if let Some(ref auth) = self.minting_authority{
            if *auth != caller{
                return Err(MintError::Unauthorized { minting_authority: self.minting_authority.clone() })
            }
        }
        if let Some(ref cap) = self.icrc7_supply_cap{
            if *cap == self.icrc7_total_supply{
                return Err(MintError::SupplyCapReached { cap: cap.clone() })
            }
        }
        let mut existing_tokens = vec![];
        for id in args.token_ids.iter(){
            if let Some(_) = self.icrc7_tokens.get(id){
                existing_tokens.push(id.clone());
            }
        }
        let to = match args.to{
            None => caller.clone(),
            Some(account) => {
                if account.owner == Principal::anonymous(){
                    return Err(MintError::GenericError { error_code: 4, message: "Anonymous Identity Provided".into() })
                }
                account_transformer(account)
            }
        };
        let time = ic_cdk::api::time();
        if let Some(true) | None = args.is_atomic{
            if existing_tokens.len() != 0{
                return Err(MintError::TokenIdExist { token_ids: existing_tokens })
            }
            for id in args.token_ids.iter(){
                let token = Icrc7Token{
                    token_id: id.clone(),
                    token_owner: to.clone(),
                    token_name: args.token_name.clone(),
                    token_image: args.image.clone(),
                    token_description: args.token_description.clone(),
                    approvals: vec![],
                };
                self.icrc7_tokens.insert(id.clone(), token);
                self.icrc7_total_supply += 1;
            }
            let txn_id = self.log_transaction(Transaction::new(
                TransactionType::Mint { to },
                args.token_ids.clone(),
                time,
                args.memo.clone(),
            ));
            return Ok(txn_id)
        }else{
            for id in args.token_ids.iter(){
                if existing_tokens.contains(id){
                    continue;
                }
                let token = Icrc7Token{
                    token_id: id.clone(),
                    token_owner: to.clone(),
                    token_name: args.token_name.clone(),
                    token_image: args.image.clone(),
                    token_description: args.token_description.clone(),
                    approvals: vec![],
                };
                self.icrc7_tokens.insert(id.clone(), token);
                self.icrc7_total_supply += 1;
            }
            if existing_tokens.len() != 0{
                return Err(MintError::TokenIdExist { token_ids: existing_tokens.clone() })
            }
            let txn_id = self.log_transaction(Transaction::new(
                TransactionType::Mint { to },
                args.token_ids.clone(),
                time,
                args.memo.clone(),
            ));
            return Ok(txn_id)
        }
    }

    pub fn icrc7_approve(
        &mut self,
        caller: &Principal,
        mut args: ApprovalArgs,
    ) -> Result<u128, ApprovalError> {
        let caller = Account {
            owner: caller.clone(),
            subaccount: args.from_subaccount,
        };
        let caller = account_transformer(caller);
        if args.spender.owner == Principal::anonymous(){
            return Err(ApprovalError::GenericError { error_code: 4, message: "Anonymous Identity Provided".into() })
        }
        args.spender = account_transformer(args.spender.clone());
        let owned_icrc7_tokens_by_caller = self.icrc7_tokens_of(&caller);
        match args.token_ids {
            None => {
                for id in owned_icrc7_tokens_by_caller.iter() {
                    let mut token = self.icrc7_tokens.get(id).unwrap();
                    token.approve(&args);
                    self.icrc7_tokens.insert(*id, token);
                }
                let txn_id = self.log_transaction(Transaction::new(
                    TransactionType::Approval {
                        from: caller.clone(),
                        spender: args.spender.clone(),
                    },
                    owned_icrc7_tokens_by_caller.clone(),
                    ic_cdk::api::time(),
                    args.memo.clone(),
                ));
                Ok(txn_id)
            }
            Some(ref ids) => {
                if ids.len() == 0 {
                    return Err(ApprovalError::GenericError {
                        error_code: 1,
                        message: "Empty Token Ids".into(),
                    });
                }
                let mut unauthorized = vec![];
                for id in ids.iter() {
                    if let None = self.icrc7_tokens.get(id) {
                        unauthorized.push(id.clone());
                        continue;
                    }
                    if !owned_icrc7_tokens_by_caller.contains(id) {
                        unauthorized.push(id.clone());
                        continue;
                    }
                    let mut token = self.icrc7_tokens.get(id).unwrap();
                    token.approve(&args);
                    self.icrc7_tokens.insert(*id, token);
                }
                if unauthorized.len() != 0 {
                    return Err(ApprovalError::Unauthorized(unauthorized));
                }
                let txn_id = self.log_transaction(Transaction::new(
                    TransactionType::Approval {
                        from: caller.clone(),
                        spender: args.spender.clone(),
                    },
                    owned_icrc7_tokens_by_caller.clone(),
                    ic_cdk::api::time(),
                    args.memo.clone(),
                ));
                Ok(txn_id)
            }
        }
    }

    pub fn icrc7_transfer(
        &mut self,
        caller: &Principal,
        mut args: TransferArgs,
    ) -> Result<u128, TransferError> {
        if args.token_ids.len() == 0 {
            return Err(TransferError::GenericError {
                error_code: 1,
                message: "Empty Token Ids".into(),
            });
        }
        if args.from.owner == Principal::anonymous(){
            return Err(TransferError::GenericError { error_code: 4, message: "Anonymous Identity Found".into() })
        }
        if args.to.owner == Principal::anonymous(){
            return Err(TransferError::GenericError { error_code: 4, message: "Anonymous Identity Found".into() })
        }
        args.token_ids.sort();
        let caller = Account {
            owner: caller.clone(),
            subaccount: args.spender_subaccount,
        };
        let caller = account_transformer(caller);
        args.from = account_transformer(args.from);
        args.to = account_transformer(args.to);
        let current_time = ic_cdk::api::time();
        if let Some(time) = args.created_at_time {
            if time < (current_time - self.tx_window - self.permitted_drift) {
                return Err(TransferError::TooOld);
            } else if time > (current_time + self.permitted_drift) {
                return Err(TransferError::CreatedInFuture {
                    ledger_time: current_time,
                });
            }
            if let Err(e) = self.transfer_deduplication_check(&current_time, &caller, &args) {
                return Err(e);
            }
        }
        let mut unauthorized = vec![];
        for id in args.token_ids.iter() {
            if let None = self.icrc7_tokens.get(id) {
                unauthorized.push(id.clone());
                continue;
            }
            let token = self.icrc7_tokens.get(id).unwrap();
            if !token.approval_check(&caller) && token.token_owner != caller{
                unauthorized.push(id.clone());
                continue;
            }else if token.approval_check(&caller) && token.token_owner != args.from{
                unauthorized.push(id.clone());
                continue;
            }
        }
        if let None | Some(true) = args.is_atomic {
            if unauthorized.len() != 0 {
                return Err(TransferError::Unauthorized {
                    token_ids: unauthorized,
                });
            } else {
                for id in args.token_ids.iter() {
                    let mut token = self.icrc7_tokens.get(id).unwrap();
                    token.transfer(&args.to);
                    self.icrc7_tokens.insert(id.clone(), token);
                }
                let tx_id = self.log_transaction(Transaction::new(
                    TransactionType::Transfer {
                        from: args.from.clone(),
                        spender: caller,
                        to: args.to.clone(),
                    },
                    args.token_ids.clone(),
                    current_time,
                    args.memo.clone(),
                ));
                return Ok(tx_id);
            }
        } else {
            for id in args.token_ids.iter() {
                if unauthorized.contains(id) {
                    continue;
                }
                let mut token = self.icrc7_tokens.get(id).unwrap();
                token.transfer(&args.to);
                self.icrc7_tokens.insert(id.clone(), token);
            }
            if unauthorized.len() != 0 {
                return Err(TransferError::Unauthorized {
                    token_ids: unauthorized,
                });
            } else {
                let tx_id = self.log_transaction(Transaction::new(
                    TransactionType::Transfer {
                        from: args.from.clone(),
                        spender: caller,
                        to: args.to.clone(),
                    },
                    args.token_ids.clone(),
                    current_time,
                    args.memo.clone(),
                ));
                return Ok(tx_id);
            }
        }
    }
}

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static ICRC7_COLLECTION: RefCell<Icrc7Collection> = RefCell::default();
}
