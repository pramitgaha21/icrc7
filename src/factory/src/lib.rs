use ic_cdk_macros::export_candid;
use candid::Principal;

pub mod init;
pub mod icrc7_factory;
pub mod state;

pub use icrc7_factory::*;

export_candid!();