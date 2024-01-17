use ic_cdk_macros::init;

use crate::{icrc7_types::InitArgs, utils::{account_transformer, default_account}, state::ICRC7_COLLECTION};

#[init]
pub fn init(args: InitArgs) {
    let minting_authority = match args.minting_authority{
        Some(account) => account_transformer(account),
        None => {
            let caller = ic_cdk::caller();
            default_account(&caller)
        }
    };
    ICRC7_COLLECTION.with(|c|{
        let mut c = c.borrow_mut();
        c.minting_authority = Some(minting_authority);
        c.icrc7_name = args.icrc7_name;
        c.icrc7_symbol = args.icrc7_symbol;
        c.icrc7_royalties = args.icrc7_royalties;
        c.icrc7_royalty_recipient = args.icrc7_royalty_recipient;
        c.icrc7_description = args.icrc7_description;
        c.icrc7_image = args.icrc7_image;
        c.icrc7_supply_cap = args.icrc7_supply_cap;
        if let Some(tx_window) = args.tx_window{
            c.tx_window = tx_window * 60 * 60 * 1000_000_000;
        }
        if let Some(permitted_drift) = args.permitted_drift{
            c.permitted_drift = permitted_drift * 60 * 1000_000_000;
        }
    });
}
