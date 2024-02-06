#ICRC7

```bash
# starts replica in background
dfx start --clean --background

chmod +x gen_candid.sh
./gen_candid.sh
```

#### Deploying Factory Canister

```bash
dfx deploy factory --with-cycles 90000000000000
```

#### Deploying Icrc7 Canister
```bash
 dfx deploy icrc7 --argument '(record{                                  
minting_account= opt record {
    owner = principal "zpxxt-vkthd-jg62u-t6yfs-fo2nu-mbtcj-loqyi-yjrsn-yycfj-5ah6j-vae";                                     
    subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
  };                  
icrc7_supply_cap= null;
icrc7_description= opt "ICP Flower Collection";
tx_window= null;                        
permitted_drift= null;                  
icrc7_max_take_value= null;
icrc7_max_memo_size= null;
icrc7_symbol= "ICFL";
icrc7_max_update_batch_size= null;
icrc7_max_query_batch_size= null;
icrc7_atomic_batch_transfers= null;
icrc7_default_take_value= null;
icrc7_logo= null;
icrc7_name= "ICP Flower"
})'
```

#### Minting NFT
```bash
dfx canister call icrc7 icrc7_mint '(record{                                  
to= record {
    owner = principal "4cu2l-slkj7-mo7ap-onxrm-ppr32-cidse-pln24-3dnaj-wtc7b-tn7dm-dae";                                     
    subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
  };          
token_id= 1;
memo= null;
from_subaccount= null;                  
token_description= opt "Token Number 1";
token_logo= null;
token_name= null
})'
```

<!-- dfx canister call icrc7 icrc7_transfer '(vec{
record{
to=record {
owner = principal "t4egw-clf4w-qbpli-svryg-7yqq6-jt2yj-7v755-mabir-zmx6i-vp4fr-fqe";
subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
};
token_id= 1;
from_subaccount= null;
memo= null;
created_at_time= opt 1707100000000000000
};
record{
to=record {
owner = principal "t4egw-clf4w-qbpli-svryg-7yqq6-jt2yj-7v755-mabir-zmx6i-vp4fr-fqe";
subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00";
};
token_id= 100;
from_subaccount= null;
memo= null;
created_at_time= opt 1707100000000000000
}
})' -->

<!-- ##### Deploying icrc7
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
``` -->
