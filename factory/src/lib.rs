use candid::{CandidType, Encode, Principal};
use ic_cdk::api::management_canister::{
    main::{create_canister, install_code, CreateCanisterArgument, InstallCodeArgument},
    provisional::CanisterSettings,
};
use ic_cdk_macros::{export_candid, update};
use icrc_ledger_types::icrc1::account::Account;
use serde::Deserialize;

pub const ICRC7_WASM: &[u8] = std::include_bytes!("./../../wasm_files/icrc7.wasm.gz");

#[derive(CandidType)]
pub struct InitArg {
    pub minting_authority: Option<Account>,
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

#[derive(CandidType, Deserialize)]
pub struct Arg {
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

impl From<(Account, Arg)> for InitArg {
    fn from((account, arg): (Account, Arg)) -> Self {
        Self {
            minting_authority: Some(account),
            icrc7_symbol: arg.icrc7_symbol,
            icrc7_name: arg.icrc7_name,
            icrc7_description: arg.icrc7_description,
            icrc7_logo: arg.icrc7_logo,
            icrc7_supply_cap: arg.icrc7_supply_cap,
            icrc7_max_query_batch_size: arg.icrc7_max_query_batch_size,
            icrc7_max_update_batch_size: arg.icrc7_max_update_batch_size,
            icrc7_max_take_value: arg.icrc7_max_take_value,
            icrc7_default_take_value: arg.icrc7_default_take_value,
            icrc7_max_memo_size: arg.icrc7_max_memo_size,
            icrc7_atomic_batch_transfers: arg.icrc7_atomic_batch_transfers,
            tx_window: arg.tx_window,
            permitted_drift: arg.permitted_drift,
        }
    }
}

#[update]
async fn mint_collection_canister(arg: Arg) -> Result<Principal, String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err("Anonymous Caller".into());
    }
    let account = Account {
        owner: caller.clone(),
        subaccount: None,
    };
    let principal = match create_canister(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(vec![ic_cdk::id(), caller.clone()]),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
            }),
        },
        10_000_000_000_000,
    )
    .await
    {
        Err((code, msg)) => return Err(format!("Rejection Code: {:?}, Message: {:?}", code, msg)),
        Ok((principal,)) => principal.canister_id,
    };
    let init_arg = InitArg::from((account, arg));
    let init_arg = Encode!(&init_arg).unwrap();
    match install_code(InstallCodeArgument {
        mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Install,
        canister_id: principal,
        wasm_module: ICRC7_WASM.to_vec(),
        arg: init_arg,
    })
    .await
    {
        Ok(()) => Ok(principal),
        Err((code, msg)) => Err(format!("Code: {:?}, Message: {:?}", code, msg)),
    }
}

export_candid!();
