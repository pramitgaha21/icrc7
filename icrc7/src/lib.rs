pub mod memory;
pub mod state;
pub mod utils;

use candid::{CandidType, Nat};
use ic_cdk::{init, query, update};
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use icrc_nft_types::{
    icrc7::{
        metadata::{Icrc7CollectionMetadata, Icrc7TokenMetadata},
        transfer::{TransferArg, TransferResult},
    },
    Account,
};
use itertools::Itertools;
use serde::Deserialize;
use state::{query_metadata, query_token_map};

#[derive(CandidType, Deserialize, Debug)]
pub struct InitArg {
    pub minting_auth: Option<Account>,
    pub icrc7_name: String,
    pub icrc7_symbol: String,
    pub icrc7_supply_cap: Option<u128>,
}

#[init]
pub fn init(arg: InitArg) {}

pub fn icrc7_collection_metadata() -> Icrc7CollectionMetadata {
    query_metadata(|metadata| {
        let mut map = Icrc7CollectionMetadata::new();
        map.insert(
            "icrc7:name".into(),
            MetadataValue::Text(metadata.icrc7_name.as_ref().unwrap().clone()),
        );
        map.insert(
            "icrc7:symbol".into(),
            MetadataValue::Text(metadata.icrc7_symbol.as_ref().unwrap().clone()),
        );
        if let Some(logo) = metadata.icrc7_logo.as_ref() {
            map.insert("icrc7:logo".into(), MetadataValue::Text(logo.clone()));
        }
        if let Some(description) = metadata.icrc7_description.as_ref() {
            map.insert(
                "icrc7:description".into(),
                MetadataValue::Text(description.clone()),
            );
        }
        map.insert(
            "icrc7:total_supply".into(),
            MetadataValue::Nat(Nat::from(query_token_map(|map| map.len() as u128))),
        );
        if let Some(supply_cap) = metadata.icrc7_supply_cap {
            map.insert(
                "icrc7:supply_cap".into(),
                MetadataValue::Nat(Nat::from(supply_cap)),
            );
        }
        if let Some(max_query_batch_size) = metadata.icrc7_max_query_batch_size {
            map.insert(
                "icrc7:max_query_batch_size".into(),
                MetadataValue::Nat(Nat::from(max_query_batch_size)),
            );
        }
        if let Some(max_update_batch_size) = metadata.icrc7_max_update_batch_size {
            map.insert(
                "icrc7:max_update_batch_size".into(),
                MetadataValue::Nat(Nat::from(max_update_batch_size)),
            );
        }
        if let Some(default_take_value) = metadata.icrc7_default_take_value {
            map.insert(
                "icrc7:default_take_value".into(),
                MetadataValue::Nat(Nat::from(default_take_value)),
            );
        }
        if let Some(max_take_value) = metadata.icrc7_max_take_value {
            map.insert(
                "icrc7:max_take_value".into(),
                MetadataValue::Nat(Nat::from(max_take_value)),
            );
        }
        if let Some(max_memo_size) = metadata.icrc7_max_memo_size {
            map.insert(
                "icrc7:max_memo_size".into(),
                MetadataValue::Nat(Nat::from(max_memo_size)),
            );
        }
        //if let Some(atomic_batch_transfer) = metadata.icrc7_atomic_batch_transfer{
        // TODO: can't figure out, how to have atomic_batch_transfer returned
        //map.insert("icrc7:atomic_batch_transfer".into(), MetadataValue)
        // }
        if let Some(permitted_drift) = metadata.icrc7_permitted_drift {
            map.insert(
                "icrc7:permitted_drift".into(),
                MetadataValue::Nat(Nat::from(permitted_drift)),
            );
        }
        if let Some(tx_window) = metadata.icrc7_permitted_drift {
            map.insert(
                "icrc7:tx_window".into(),
                MetadataValue::Nat(Nat::from(tx_window)),
            );
        }
        map
    })
}

#[query]
pub fn icrc7_name() -> String {
    query_metadata(|metadata| metadata.icrc7_name.as_ref().unwrap().clone())
}

#[query]
pub fn icrc7_symbol() -> String {
    query_metadata(|metadata| metadata.icrc7_symbol.as_ref().unwrap().clone())
}

#[query]
pub fn icrc7_total_supply() -> u128 {
    query_token_map(|map| map.len() as u128)
}

#[query]
pub fn icrc7_supply_cap() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_supply_cap)
}

#[query]
pub fn icrc7_description() -> Option<String> {
    query_metadata(|metadata| metadata.icrc7_description.clone())
}

#[query]
pub fn icrc7_logo() -> Option<String> {
    query_metadata(|metadata| metadata.icrc7_logo.clone())
}

#[query]
pub fn icrc7_max_query_batch_size() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_max_query_batch_size)
}

#[query]
pub fn icrc7_max_update_batch_size() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_max_update_batch_size)
}

#[query]
pub fn icrc7_default_take_value() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_default_take_value)
}

#[query]
pub fn icrc7_max_take_value() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_max_take_value)
}

#[query]
pub fn icrc7_max_memo_size() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_max_memo_size)
}

#[query]
pub fn icrc7_atomic_batch_transfer() -> Option<bool> {
    query_metadata(|metadata| metadata.icrc7_atomic_batch_transfer)
}

#[query]
pub fn icrc7_tx_window() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_tx_window)
}

#[query]
pub fn icrc7_permitted_drift() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_permitted_drift)
}

#[query]
pub fn icrc7_token_metadata(token_ids: Vec<u128>) -> Vec<Option<Icrc7TokenMetadata>> {
    query_token_map(|token_map| {
        token_ids
            .into_iter()
            .map(|id| {
                if let Some(token) = token_map.get(&id) {
                    Some(token.token_metadata())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[query]
pub fn icrc7_owner_of(token_ids: Vec<u128>) -> Vec<Option<Account>> {
    query_token_map(|token_map| {
        token_ids
            .into_iter()
            .map(|id| {
                if let Some(token) = token_map.get(&id) {
                    Some(token.owner)
                } else {
                    None
                }
            })
            .collect()
    })
}

#[query]
pub fn icrc7_balance_of(accounts: Vec<Account>) -> Vec<Nat> {
    query_token_map(|token_map| {
        accounts
            .into_iter()
            .map(|account| {
                let mut balance = Nat::from(0u128);
                token_map.iter().for_each(|(_k, v)| {
                    if v.owner == account {
                        balance += Nat::from(1u128);
                    }
                });
                balance
            })
            .collect()
    })
}

pub fn icrc7_tokens(prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
    query_token_map(|token_map| {
        token_map
            .iter()
            .map(|(_map_id, token)| token.id)
            .sorted()
            .filter(|token_id| {
                *token_id
                    > match prev {
                        Some(strat_token) => strat_token,
                        _ => 0,
                    }
            })
            .take(match take {
                Some(to_take) => to_take,
                _ => icrc7_default_take_value().unwrap(),
            } as usize)
            .collect::<Vec<u128>>()
    })
}

pub fn icrc7_tokens_of(account: Account, prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
    query_token_map(|token_map| {
        token_map
            .iter()
            .filter(|(_k, v)| v.owner == account)
            .map(|(_k, v)| v.id)
            .sorted()
            .filter(|token_id| {
                *token_id
                    > match prev {
                        Some(strat_token) => strat_token,
                        _ => 0,
                    }
            })
            .take(match take {
                Some(to_take) => to_take,
                _ => icrc7_default_take_value().unwrap(),
            } as usize)
            .collect::<Vec<u128>>()
    })
}

pub fn icrc7_transfer(args: Vec<TransferArg>) -> Vec<Option<TransferResult>> {
    todo!()
}

pub fn mint() {}

pub fn burn() {}

#[derive(CandidType, Debug)]
pub struct Standard {
    name: String,
    url: String,
}

#[query]
pub fn icrc10_supported_standards() -> Vec<Standard> {
    vec![
        Standard {
            name: "ICRC-7".into(),
            url: "https://github.com/dfinity/ICRC/ICRCs/ICRC-7".into(),
        },
        Standard {
            name: "ICRC-61".into(),
            url: "https://github.com/dfinity/ICRC/ICRCs/ICRC-61".into(),
        },
    ]
}

ic_cdk::export_candid!();
