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
dfx canister call icrc7 mint '(record{                                  
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

# Transferring tokens
```bash
dfx canister call icrc7 icrc7_transfer '(vec{
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
})'
```