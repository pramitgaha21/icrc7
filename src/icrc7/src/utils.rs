use candid::Principal;
use icrc_ledger_types::icrc1::account::{Account, DEFAULT_SUBACCOUNT};

pub fn default_account_from_principal(owner: Principal) -> Account {
    Account {
        owner,
        subaccount: Some(DEFAULT_SUBACCOUNT.clone()),
    }
}
