use ic_cdk_macros::init;

use crate::{types::InitArgs, state::COLLECTION};

#[init]
pub fn init(args: InitArgs) {
    COLLECTION.with(|c|{
        let mut c = c.borrow_mut();
        c.icrc7_name = args.icrc7_name;
        c.icrc7_description = args.icrc7_description;
        c.icrc7_image = args.icrc7_image;
        c.icrc7_royalties = args.icrc7_royalties;
        c.icrc7_royalty_recipient = args.icrc7_royalty_recipient;
        c.icrc7_supply_cap = args.icrc7_supply_cap;
        if let Some(auth) = args.minting_authority{
            c.minting_authority = auth;
        }else{
            c.minting_authority = ic_cdk::caller();
        }
    })
}
