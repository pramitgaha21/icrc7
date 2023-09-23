use ic_cdk_macros::update;

use crate::{types::{TransferArgs, TransferError, ApprovalArgs, ApprovalError, MintArgs, MintError}, state::COLLECTION};

#[update]
pub fn icrc7_transfer(args: TransferArgs) -> Result<u128, TransferError>{
    let caller = ic_cdk::caller();
    COLLECTION.with(|c|{
        c.borrow_mut().icrc7_transfer(&caller, args)
    })
}

#[update]
pub fn icrc7_approve(args: ApprovalArgs) -> Result<u128, ApprovalError>{
    let caller = ic_cdk::caller();
    COLLECTION.with(|c|{
        c.borrow_mut().icrc7_approve(&caller, args)
    })
}

#[update]
pub fn icrc7_mint(args: MintArgs) -> Result<u128, MintError>{
    let caller = ic_cdk::caller();
    COLLECTION.with(|c|{
        c.borrow_mut().icrc7_mint(&caller, args)
    })
}