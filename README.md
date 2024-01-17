#ICRC7

```bash
# starts replica in background
dfx start --clean --background

chmod +x gen_candid.sh
./gen_candid.sh
```

##### Deploying icrc7
```bash
dfx deploy icrc7 --argument '(record{
    minting_authority= null;
    tx_window=  null;
    permitted_drift= null;
    icrc7_name= "ICP Flower";
    icrc7_symbol= "ICFL";
    icrc7_royalties= null;
    icrc7_royalty_recipient= null;
    icrc7_description= opt "ICP Flower Collection";
    icrc7_image= null;
    icrc7_supply_cap= opt 10000;
})'
```

#### Minting Tokens
```bash
dfx canister call icrc7 icrc7_mint '(record{
    to= opt record{
        owner= principal "xt6yz-cz4ec-kpgmw-ppyge-xjob6-v6hjw-dafbf-rxa3r-5z3ab-4ii45-sqe";
        subaccount= null
    };
    memo= null;
    is_atomic= null;
    token_description= opt "Secret 1";
    token_ids= vec {1; 2; 3};
    image= null;
    token_name= "Super Secret";
})'
```

#### Transferring Tokens
```bash
dfx canister call icrc7 icrc7_transfer '(record{
    to= record{
        owner= principal "uvvrl-hlaqd-wj2ls-u2x5d-y5mnv-cjhh4-f4gqu-tud6c-2dire-6zdmx-3ae";
        subaccount= null;
    };
    spender_subaccount= null;
    from= record{
        owner= principal "xt6yz-cz4ec-kpgmw-ppyge-xjob6-v6hjw-dafbf-rxa3r-5z3ab-4ii45-sqe";
        subaccount= null;
    };
    is_atomic= null;
    token_ids= vec{ 1 };
    created_at_time= null;
})'
```