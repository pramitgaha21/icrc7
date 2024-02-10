use std::{cell::RefCell, collections::HashMap};

use crate::{
    icrc7_types::{
        BurnResult, MintArg, Icrc7TokenMetadata, MintError, MintResult, Transaction,
        TransactionType, TransferArg, TransferError, TransferResult,
    },
    memory::{get_log_memory, get_token_map_memory, Memory},
    utils::{account_transformer, burn_account},
    BurnError, BurnArg,
};
use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{
    memory_manager::MemoryManager, storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable,
};
use icrc_ledger_types::{icrc::generic_metadata_value::MetadataValue, icrc1::account::Account};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Icrc7Token {
    pub token_id: u128,
    pub token_name: String,
    pub token_description: Option<String>,
    pub token_logo: Option<String>,
    pub token_owner: Account,
}

impl Storable for Icrc7Token {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Icrc7Token {
    fn new(
        token_id: u128,
        token_name: String,
        token_description: Option<String>,
        token_logo: Option<String>,
        token_owner: Account,
    ) -> Self {
        Self {
            token_id,
            token_name,
            token_logo,
            token_owner,
            token_description,
        }
    }

    fn transfer(&mut self, to: Account) {
        self.token_owner = to;
    }

    fn token_metadata(&self) -> Icrc7TokenMetadata {
        let mut metadata = HashMap::<String, MetadataValue>::new();
        metadata.insert("Name".into(), MetadataValue::Text(self.token_name.clone()));
        metadata.insert(
            "Symbol".into(),
            MetadataValue::Text(self.token_name.clone()),
        );
        if let Some(ref description) = self.token_description {
            metadata.insert(
                "Description".into(),
                MetadataValue::Text(description.clone()),
            );
        }
        if let Some(ref logo) = self.token_logo {
            metadata.insert("logo".into(), MetadataValue::Text(logo.clone()));
        }
        metadata
    }

    fn burn(&mut self, burn_address: Account) {
        self.token_owner = burn_address;
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub minting_authority: Option<Account>,
    pub icrc7_symbol: String,
    pub icrc7_name: String,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_total_supply: u128,
    pub icrc7_supply_cap: Option<u128>,
    pub icrc7_max_query_batch_size: Option<u128>,
    pub icrc7_max_update_batch_size: Option<u128>,
    pub icrc7_max_take_value: Option<u128>,
    pub icrc7_default_take_value: Option<u128>,
    pub icrc7_max_memo_size: Option<u128>,
    pub icrc7_atomic_batch_transfers: Option<bool>,
    pub tx_window: Option<u64>,
    pub permitted_drift: Option<u64>,
    #[serde(skip, default = "get_token_map_memory")]
    pub tokens: StableBTreeMap<u128, Icrc7Token, Memory>,
    pub txn_count: u128,
    #[serde(skip, default = "get_log_memory")]
    pub txn_log: StableBTreeMap<u128, Transaction, Memory>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            minting_authority: None,
            icrc7_symbol: "ICRC7".into(),
            icrc7_name: "ICRC7 Collection".into(),
            icrc7_description: None,
            icrc7_logo: None,
            icrc7_total_supply: 0,
            icrc7_supply_cap: None,
            icrc7_max_query_batch_size: None,
            icrc7_max_update_batch_size: None,
            icrc7_max_take_value: None,
            icrc7_default_take_value: None,
            icrc7_max_memo_size: None,
            icrc7_atomic_batch_transfers: None,
            tx_window: None,
            permitted_drift: None,
            tokens: get_token_map_memory(),
            txn_count: 0,
            txn_log: get_log_memory(),
        }
    }
}

impl State {
    pub const DEFAULT_MAX_QUERY_BATCH_SIZE: u128 = 32;
    pub const DEFAULT_MAX_UPDATE_BATCH_SIZE: u128 = 32;
    pub const DEFAULT_TAKE_VALUE: u128 = 32;
    pub const DEFAULT_MAX_TAKE_VALUE: u128 = 32;
    pub const DEFAULT_MAX_MEMO_SIZE: u128 = 32;
    pub const DEFAULT_TX_WINDOW: u64 = 24 * 60 * 60 * 1000_000_000;
    pub const DEFAULT_PERMITTED_DRIFT: u64 = 2 * 60 * 1000_000_000;

    pub fn icrc7_symbol(&self) -> String {
        self.icrc7_symbol.clone()
    }

    pub fn icrc7_name(&self) -> String {
        self.icrc7_name.clone()
    }

    pub fn icrc7_description(&self) -> Option<String> {
        self.icrc7_description.clone()
    }

    pub fn icrc7_total_supply(&self) -> u128 {
        self.icrc7_total_supply
    }

    pub fn icrc7_supply_cap(&self) -> Option<u128> {
        self.icrc7_supply_cap
    }

    pub fn icrc7_logo(&self) -> Option<String> {
        self.icrc7_logo.clone()
    }

    pub fn icrc7_max_query_batch_size(&self) -> Option<u128> {
        self.icrc7_max_query_batch_size
    }

    pub fn icrc7_max_update_batch_size(&self) -> Option<u128> {
        self.icrc7_max_update_batch_size
    }

    pub fn icrc7_default_take_value(&self) -> Option<u128> {
        self.icrc7_default_take_value
    }

    pub fn icrc7_max_take_value(&self) -> Option<u128> {
        self.icrc7_max_take_value
    }

    pub fn icrc7_max_memo_size(&self) -> Option<u128> {
        self.icrc7_max_memo_size
    }

    pub fn icrc7_atomic_batch_transfers(&self) -> Option<bool> {
        self.icrc7_atomic_batch_transfers
    }

    pub fn icrc7_owner_of(&self, token_id: &[u128]) -> Vec<Option<Account>> {
        let mut res = vec![None; token_id.len()];
        for (index, id) in token_id.iter().enumerate() {
            if let Some(ref token) = self.tokens.get(id) {
                res.insert(index, Some(token.token_owner))
            }
        }
        res
    }

    fn txn_deduplication_check(
        &self,
        allowed_past_time: &u64,
        caller: &Account,
        args: &TransferArg,
    ) -> Result<(), TransferError> {
        let mut count = self.txn_count;
        while count != 0 {
            let txn = self.txn_log.get(&count).unwrap();
            if txn.ts < *allowed_past_time {
                return Ok(());
            }
            match txn.txn_type {
                TransactionType::Transfer {
                    ref tid,
                    ref from,
                    ref to,
                } => {
                    if &args.token_id == tid
                        && caller == from
                        && &args.to == to
                        && args.memo == txn.memo
                        && args.created_at_time == Some(txn.ts)
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

    fn get_txn_id(&mut self) -> u128 {
        self.txn_count += 1;
        self.txn_count
    }

    fn log_transaction(
        &mut self,
        txn_type: TransactionType,
        ts: u64,
        memo: Option<Vec<u8>>,
    ) -> u128 {
        let txn_id = self.get_txn_id();
        let txn = Transaction::new(txn_id, txn_type, ts, memo);
        self.txn_log.insert(txn_id, txn);
        txn_id
    }

    fn mock_transfer(
        &self,
        current_time: &u64,
        caller: &Account,
        arg: &TransferArg,
    ) -> Result<(), TransferError> {
        if let Some(time) = arg.created_at_time {
            let allowed_past_time = *current_time
                - self.tx_window.unwrap_or(State::DEFAULT_TX_WINDOW)
                - self
                    .permitted_drift
                    .unwrap_or(State::DEFAULT_PERMITTED_DRIFT);
            let allowed_future_time = *current_time
                + self
                    .permitted_drift
                    .unwrap_or(State::DEFAULT_PERMITTED_DRIFT);
            if time < allowed_past_time {
                return Err(TransferError::TooOld);
            } else if time > allowed_future_time {
                return Err(TransferError::CreatedInFuture {
                    ledger_time: current_time.clone(),
                });
            }
            self.txn_deduplication_check(&allowed_past_time, caller, arg)?;
        }
        // checking is token for the corresponding ID exists or not
        if let None = self.tokens.get(&arg.token_id) {
            return Err(TransferError::NonExistingTokenId);
        }
        if let Some(ref memo) = arg.memo {
            let max_memo_size = self
                .icrc7_max_memo_size
                .unwrap_or(State::DEFAULT_MAX_MEMO_SIZE);
            if memo.len() as u128 > max_memo_size {
                return Err(TransferError::GenericError {
                    error_code: 3,
                    message: "Exceeds Max Memo Size".into(),
                });
            }
        }
        // checking if receiver and sender have same address
        if arg.to == *caller {
            return Err(TransferError::InvalidRecipient);
        }
        let token = self.tokens.get(&arg.token_id).unwrap();
        // checking if the caller is authorized to make transaction
        if token.token_owner != *caller {
            return Err(TransferError::Unauthorized);
        }
        Ok(())
    }

    pub fn icrc7_transfer(
        &mut self,
        caller: &Principal,
        mut args: Vec<TransferArg>,
    ) -> Vec<Option<TransferResult>> {
        // checking if the argument length in 0
        if args.len() == 0 {
            return vec![Some(Err(TransferError::GenericBatchError {
                error_code: 1,
                message: "No Arguments Provided".into(),
            }))];
        }
        let max_update_batch_size = self
            .icrc7_max_query_batch_size
            .unwrap_or(State::DEFAULT_MAX_UPDATE_BATCH_SIZE);
        let mut txn_results = vec![None; args.len()];
        if args.len() as u128 > max_update_batch_size {
            txn_results[0] = Some(Err(TransferError::GenericBatchError {
                error_code: 2,
                message: "Exceed Max allowed Update Batch Size".into(),
            }));
            return txn_results;
        }
        if *caller == Principal::anonymous() {
            txn_results[0] = Some(Err(TransferError::GenericBatchError {
                error_code: 100,
                message: "Anonymous Identity".into(),
            }));
            return txn_results;
        }
        let current_time = ic_cdk::api::time();
        for (index, arg) in args.iter_mut().enumerate() {
            let caller_account = account_transformer(Account {
                owner: caller.clone(),
                subaccount: arg.from_subaccount,
            });
            arg.to = account_transformer(arg.to);
            if let Err(e) = self.mock_transfer(&current_time, &caller_account, &arg) {
                txn_results[index] = Some(Err(e));
            }
        }
        if let Some(true) = self.icrc7_atomic_batch_transfers {
            if txn_results
                .iter()
                .any(|res| res.is_some() && res.as_ref().unwrap().is_err())
            {
                return txn_results;
            }
        }
        for (index, arg) in args.iter().enumerate() {
            let caller_account = account_transformer(Account {
                owner: caller.clone(),
                subaccount: arg.from_subaccount,
            });
            let time = arg.created_at_time.unwrap_or(current_time);
            if let Some(Err(e)) = txn_results.get(index).unwrap() {
                match e {
                    TransferError::GenericBatchError {
                        error_code: _,
                        message: _,
                    } => return txn_results,
                    _ => continue,
                }
            }
            let mut token = self.tokens.get(&arg.token_id).unwrap();
            token.transfer(arg.to.clone());
            self.tokens.insert(arg.token_id, token);
            let txn_id = self.log_transaction(
                TransactionType::Transfer {
                    tid: arg.token_id,
                    from: caller_account.clone(),
                    to: arg.to.clone(),
                },
                time,
                arg.memo.clone(),
            );
            txn_results[index] = Some(Ok(txn_id));
        }
        txn_results
    }

    fn mock_mint(&self, caller: &Account, arg: &MintArg) -> Result<(), MintError> {
        if let Some(cap) = self.icrc7_supply_cap {
            if cap == self.icrc7_total_supply {
                return Err(MintError::SupplyCapReached);
            }
        }
        if let None = self.minting_authority {
            return Err(MintError::GenericBatchError {
                error_code: 6,
                message: "Minting Authority Not Set".into(),
            });
        }
        if Some(*caller) != self.minting_authority {
            return Err(MintError::Unauthorized);
        }
        if let Some(ref memo) = arg.memo {
            let allowed_memo_length = self
                .icrc7_max_memo_size
                .unwrap_or(State::DEFAULT_MAX_MEMO_SIZE);
            if memo.len() as u128 > allowed_memo_length {
                return Err(MintError::GenericError {
                    error_code: 7,
                    message: "Exceeds Allowed Memo Length".into(),
                });
            }
        }
        if let Some(_) = self.tokens.get(&arg.token_id) {
            return Err(MintError::TokenIdAlreadyExist);
        }
        Ok(())
    }

    pub fn mint(&mut self, caller: &Principal, mut arg: MintArg) -> MintResult {
        let caller = account_transformer(Account {
            owner: caller.clone(),
            subaccount: arg.from_subaccount,
        });
        arg.to = account_transformer(arg.to);
        self.mock_mint(&caller, &arg)?;
        let token_name = arg.token_name.unwrap_or_else(|| {
            let name = format!("{} {}", self.icrc7_symbol, arg.token_id);
            name
        });
        let token = Icrc7Token::new(
            arg.token_id,
            token_name.clone(),
            arg.token_description.clone(),
            arg.token_logo,
            arg.to.clone(),
        );
        self.tokens.insert(arg.token_id, token);
        let txn_id = self.log_transaction(
            TransactionType::Mint {
                tid: arg.token_id,
                from: caller,
                to: arg.to,
                meta: MetadataValue::Text(arg.token_description.unwrap_or(token_name)),
            },
            ic_cdk::api::time(),
            arg.memo,
        );
        Ok(txn_id)
    }

    fn mock_burn(&self, caller: &Account, arg: &BurnArg) -> Result<(), BurnError> {
        if let Some(ref memo) = arg.memo {
            if memo.len() as u128
                > self
                    .icrc7_max_memo_size
                    .unwrap_or(State::DEFAULT_MAX_MEMO_SIZE)
            {
                return Err(BurnError::GenericError {
                    error_code: 3,
                    message: "Exceeds Max Memo Length".into(),
                });
            }
        }
        match self.tokens.get(&arg.token_id) {
            None => Err(BurnError::NonExistingTokenId),
            Some(ref token) => {
                if token.token_owner != *caller {
                    return Err(BurnError::Unauthorized);
                }
                Ok(())
            }
        }
    }

    pub fn burn(
        &mut self,
        caller: &Principal,
        mut args: Vec<BurnArg>,
    ) -> Vec<Option<BurnResult>> {
        if args.len() == 0 {
            return vec![Some(Err(BurnError::GenericBatchError {
                error_code: 1,
                message: "No Arguments Provided".into(),
            }))];
        }
        let mut txn_results = vec![None; args.len()];
        if *caller == Principal::anonymous() {
            txn_results[0] = Some(Err(BurnError::GenericBatchError {
                error_code: 100,
                message: "Anonymous Identity".into(),
            }));
            return txn_results;
        }
        for (index, arg) in args.iter_mut().enumerate() {
            let caller = account_transformer(Account {
                owner: caller.clone(),
                subaccount: arg.from_subaccount,
            });
            if let Err(e) = self.mock_burn(&caller, arg) {
                txn_results.insert(index, Some(Err(e)))
            }
        }
        if let Some(true) = self.icrc7_atomic_batch_transfers {
            if txn_results
                .iter()
                .any(|res| res.is_some() && res.as_ref().unwrap().is_err())
            {
                return txn_results;
            }
        }
        for (index, arg) in args.iter().enumerate() {
            let caller = account_transformer(Account {
                owner: caller.clone(),
                subaccount: arg.from_subaccount,
            });
            let burn_address = burn_account();
            if let Some(Err(e)) = txn_results.get(index).unwrap() {
                match e {
                    BurnError::GenericBatchError {
                        error_code: _,
                        message: _,
                    } => return txn_results,
                    _ => continue,
                }
            }
            let mut token = self.tokens.get(&arg.token_id).unwrap();
            token.burn(burn_address.clone());
            let tid = self.log_transaction(
                TransactionType::Burn {
                    tid: arg.token_id,
                    from: caller,
                    to: burn_address,
                },
                ic_cdk::api::time(),
                arg.memo.clone(),
            );
            txn_results.insert(index, Some(Ok(tid)))
        }
        txn_results
    }

    pub fn icrc7_token_metadata(&self, token_ids: &[u128]) -> Vec<Option<Icrc7TokenMetadata>> {
        if token_ids.len() as u128
            > self
                .icrc7_max_query_batch_size
                .unwrap_or(State::DEFAULT_MAX_QUERY_BATCH_SIZE)
        {
            ic_cdk::trap("Exceeds Max Query Batch Size")
        }
        let mut metadata_list = vec![None; token_ids.len()];
        for (index, tid) in token_ids.iter().enumerate() {
            if let Some(ref token) = self.tokens.get(tid) {
                metadata_list.insert(index, Some(token.token_metadata()))
            }
        }
        metadata_list
    }

    pub fn icrc7_balance_of(&self, accounts: &[Account]) -> Vec<u128> {
        let mut count_list = vec![0; accounts.len()];
        accounts.iter().enumerate().for_each(|(index, account)| {
            self.tokens.iter().for_each(|(_id, ref token)| {
                if token.token_owner == *account {
                    let current_count = count_list[index];
                    count_list[index] = current_count + 1;
                }
            })
        });
        count_list
    }

    pub fn icrc7_tokens(&self, prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
        let take = take.unwrap_or(State::DEFAULT_TAKE_VALUE);
        if take > State::DEFAULT_MAX_TAKE_VALUE {
            ic_cdk::trap("Exceeds Max Take Value")
        }
        let mut list: Vec<u128> = self.tokens.iter().map(|(k, _)| k).collect();
        list.sort();
        match prev {
            Some(prev) => match list.iter().position(|id| *id == prev) {
                None => vec![],
                Some(index) => list
                    .iter()
                    .map(|id| *id)
                    .skip(index)
                    .take(take as usize)
                    .collect(),
            },
            None => list[0..take as usize].to_vec(),
        }
    }

    pub fn icrc7_tokens_of(
        &self,
        account: Account,
        prev: Option<u128>,
        take: Option<u128>,
    ) -> Vec<u128> {
        let take = take.unwrap_or(State::DEFAULT_TAKE_VALUE);
        if take > State::DEFAULT_MAX_TAKE_VALUE {
            ic_cdk::trap("Exceeds Max Take Value")
        }
        let mut owned_tokens = vec![];
        for (id, token) in self.tokens.iter() {
            if token.token_owner == account {
                owned_tokens.push(id);
            }
        }
        owned_tokens.sort();
        match prev {
            None => owned_tokens[0..=take as usize].to_vec(),
            Some(prev) => match owned_tokens.iter().position(|id| *id == prev) {
                None => vec![],
                Some(index) => owned_tokens
                    .iter()
                    .map(|id| *id)
                    .skip(index)
                    .take(take as usize)
                    .collect(),
            },
        }
    }
}

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static STATE: RefCell<State> = RefCell::default();
}
