use candid::{Principal, CandidType, Deserialize, Encode};
use ic_cdk::api::management_canister::{main::{CreateCanisterArgument, InstallCodeArgument, CanisterInstallMode}, provisional::CanisterSettings};
use ic_cdk_macros::{update, query};
use icrc_ledger_types::icrc1::account::Account;

use crate::state::STATE;

const ICRC7_WASM: &[u8] = std::include_bytes!("./../../../wasm_files/icrc7.wasm");

#[derive(CandidType)]
pub enum MintCanisterError {
    AnonymousCaller,
    FailedToCreateCanister{ message: String },
    FailedToInstallWasm{ message: String },
}

#[derive(CandidType, Deserialize)]
pub struct Icrc7MintArgs {
    pub icrc7_name: String,
    pub icrc7_symbol: String,
    pub icrc7_royalties: Option<u16>,
    pub icrc7_royalty_recipient: Option<Account>,
    pub icrc7_description: Option<String>,
    pub icrc7_image: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
    pub permitted_drift: u64,
    pub tx_window: u64,
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs{
    pub minting_authority: Option<Principal>,
    pub icrc7_name: String,
    pub icrc7_symbol: String,
    pub icrc7_royalties: Option<u16>,
    pub icrc7_royalty_recipient: Option<Account>,
    pub icrc7_description: Option<String>,
    pub icrc7_image: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
    pub permitted_drift: u64,
    pub tx_window: u64,
}

impl From<(Principal, Icrc7MintArgs)> for InitArgs{
    fn from((minter, args): (Principal, Icrc7MintArgs)) -> Self {
        Self { minting_authority: Some(minter), icrc7_name: args.icrc7_name, icrc7_symbol: args.icrc7_symbol, icrc7_royalties: args.icrc7_royalties, icrc7_royalty_recipient: args.icrc7_royalty_recipient, icrc7_description: args.icrc7_description, icrc7_image: args.icrc7_image, icrc7_supply_cap: args.icrc7_supply_cap, permitted_drift: args.permitted_drift, tx_window: args.tx_window }
    }
}

#[update]
pub async fn mint_icrc7_canister(args: Icrc7MintArgs) -> Result<Principal, MintCanisterError> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous(){
        return Err(MintCanisterError::AnonymousCaller)
    }
    let create_canister_args: CreateCanisterArgument = CreateCanisterArgument { settings: Some(CanisterSettings{
        controllers: Some(vec![caller]),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
    }) };
    let canister_id = match ic_cdk::api::management_canister::main::create_canister(create_canister_args, 10_000_000_000_000).await{
        Ok((id,)) => id.canister_id,
        Err((error_code, message)) => return Err(MintCanisterError::FailedToCreateCanister{ message: format!("Error code: {:?}, Message: {}", error_code, message)})
    };
    if canister_id == Principal::anonymous(){
        return Err(MintCanisterError::FailedToCreateCanister{ message: format!("Error code: {:?}, Message: {}", 1, "Anonymous Principal".to_string() ) })
    }
    let canister_init_args = InitArgs::from((caller.clone(), args));
    let canister_install_args: InstallCodeArgument = InstallCodeArgument{
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: ICRC7_WASM.to_vec(),
        arg: Encode!(&canister_init_args).unwrap(),
    };
    if let Err((error_code, message)) = ic_cdk::api::management_canister::main::install_code(canister_install_args).await{
        return Err(MintCanisterError::FailedToInstallWasm{ message: format!("Error code: {:?}, Message: {}", error_code, message)})
    }
    STATE.with(|s|{
        s.borrow_mut().canisters.insert(caller, canister_id);
    });
    Ok(canister_id)
}

#[query]
pub fn get_canister() -> Option<Principal>{
    let caller = ic_cdk::caller();
    STATE.with(|s|{
        if let Some(canister) = s.borrow().canisters.get(&caller){
            Some(canister.clone())
        }else{
            None
        }
    })
}