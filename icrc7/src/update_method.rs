use candid::Principal;
use ic_cdk_macros::update;

use crate::{
    icrc7_types::{ApprovalArgs, ApprovalError, TransferArgs, TransferError, MintArgs, MintError},
    state::ICRC7_COLLECTION,
};

#[update]
pub fn icrc7_transfer(args: TransferArgs) -> Result<u128, TransferError> {
    if let Some(ref memo) = args.memo{
        if memo.len() > 32{
            return Err(TransferError::GenericError { error_code: 2, message: "Memo length Too Large".into() })
        }
    }
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous(){
        return Err(TransferError::GenericError { error_code: 3, message: "Anonymous Caller".into() })
    }
    ICRC7_COLLECTION.with(|c| c.borrow_mut().icrc7_transfer(&caller, args))
}

#[update]
pub fn icrc7_approve(args: ApprovalArgs) -> Result<u128, ApprovalError> {
    if let Some(ref memo) = args.memo{
        if memo.len() > 32{
            return Err(ApprovalError::GenericError { error_code: 2, message: "Memo length Too Large".into() })
        }
    }
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous(){
        return Err(ApprovalError::GenericError { error_code: 3, message: "Anonymous Caller".into() })
    }
    ICRC7_COLLECTION.with(|c| c.borrow_mut().icrc7_approve(&caller, args))
}

#[update]
pub fn icrc7_mint(args: MintArgs) -> Result<u128, MintError>{
    if let Some(ref memo) = args.memo{
        if memo.len() > 32{
            return Err(MintError::GenericError { error_code: 2, message: "Memo length Too Large".into() })
        }
    }
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous(){
        return Err(MintError::GenericError { error_code: 3, message: "Anonymous Caller".into() })
    }
    ICRC7_COLLECTION.with(|c| c.borrow_mut().icrc7_mint(&caller, args))
}