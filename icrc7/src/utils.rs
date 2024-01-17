use candid::Principal;
use icrc_ledger_types::icrc1::account::{Account, DEFAULT_SUBACCOUNT};

pub fn account_transformer(account: Account) -> Account {
    if let Some(_) = account.subaccount {
        account
    } else {
        Account {
            owner: account.owner,
            subaccount: Some(DEFAULT_SUBACCOUNT.clone()),
        }
    }
}

pub fn default_account(owner: &Principal) -> Account {
    Account {
        owner: owner.clone(),
        subaccount: Some(DEFAULT_SUBACCOUNT.clone()),
    }
}