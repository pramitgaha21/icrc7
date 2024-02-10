use ic_cdk_macros::init;
use icrc_ledger_types::icrc1::account::Account;

use crate::{icrc7_types::InitArg, state::STATE, utils::account_transformer};

#[init]
pub fn init(arg: InitArg) {
    let minting_authority = account_transformer(match arg.minting_account {
        None => {
            let caller = ic_cdk::caller();
            account_transformer(Account {
                owner: caller,
                subaccount: None,
            })
        }
        Some(acc) => account_transformer(acc),
    });
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.minting_authority = Some(minting_authority);
        s.icrc7_symbol = arg.icrc7_symbol;
        s.icrc7_name = arg.icrc7_name;
        s.icrc7_description = arg.icrc7_description;
        s.icrc7_logo = arg.icrc7_logo;
        s.icrc7_supply_cap = arg.icrc7_supply_cap;
        s.icrc7_max_query_batch_size = arg.icrc7_max_query_batch_size;
        s.icrc7_max_update_batch_size = arg.icrc7_max_update_batch_size;
        s.icrc7_max_take_value = arg.icrc7_max_take_value;
        s.icrc7_default_take_value = arg.icrc7_default_take_value;
        s.icrc7_max_memo_size = arg.icrc7_max_memo_size;
        s.icrc7_atomic_batch_transfers = arg.icrc7_atomic_batch_transfers;
        s.tx_window = arg.tx_window;
        s.permitted_drift = arg.permitted_drift;
    })
}