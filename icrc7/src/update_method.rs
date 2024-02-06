use candid::Principal;
use ic_cdk_macros::update;

use crate::{
    state::STATE, BurnResult, Icrc7BurnArg, Icrc7MintArg, MintResult, TransferArg, TransferResult,
};

#[update]
pub fn icrc7_mint(arg: Icrc7MintArg) -> MintResult {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err(crate::MintError::GenericBatchError {
            error_code: 100,
            message: "Anonymous Identity".into(),
        });
    }
    STATE.with(|s| s.borrow_mut().icrc7_mint(&caller, arg))
}

#[update]
pub fn icrc7_transfer(args: Vec<TransferArg>) -> Vec<Option<TransferResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().icrc7_transfer(&caller, args))
}

#[update]
pub fn icrc7_burn(args: Vec<Icrc7BurnArg>) -> Vec<Option<BurnResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().icrc7_burn(&caller, args))
}
