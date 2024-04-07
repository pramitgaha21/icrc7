use icrc_nft_types::{Account, DEFAULT_SUBACCOUNT};

pub fn account_transformer(account: Account) -> Account {
    if account.subaccount.is_none() {
        Account {
            subaccount: Some(*DEFAULT_SUBACCOUNT),
            ..account
        }
    } else {
        account
    }
}
