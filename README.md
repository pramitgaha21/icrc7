# icrc7

## Guide for Deploying the canister locally

#### Building the canisters
```bash
chmod +x scripts/build.sh
./scripts/build.sh
```

#### Deploying canisters
```bash
# starts the local replica in the background
dfx start --clean --background

# deploys the factory canister
dfx deploy factory --with-cycles 50000000000000

# deploys the icrc7 canister
dfx deploy icrc7 --argument '(record{
  icrc7_supply_cap= null;
  icrc7_description= opt "Flower NFT Collection";
  tx_window= 24;
  permitted_drift= 5;
  icrc7_royalty_recipient= null;
  minting_authority= null;
  icrc7_royalties= null;
  icrc7_symbol= "FLOW";
  icrc7_image= null;
  icrc7_name= "Flower NFT"
})'
```

#### Interacting with the canisters

##### Factory canister

`mint_icrc7_canister : (Icrc7MintArgs) -> (Result);`
This function is use to mint new icrc7 canister

`e.g.` calling the function
```bash
dfx canister call factory mint_icrc7_canister '(record{
icrc7_supply_cap= null;
icrc7_description= null;
tx_window= 24;
permitted_drift= 2;
icrc7_royalty_recipient= null;
icrc7_royalties= null;
icrc7_symbol= "ICFL";
icrc7_name= "ICP FLOWER"
})'
```

`get_canister : () -> (opt principal) query;`
This function returns the icrc7 canister's principal minted by the caller

`e.g.` calling the function
```bash
dfx canister call factory get_canister
```

##### icrc7 canister
`icrc7_mint : (MintArgs) -> (Result_1);`
This function is use to mint NFT

`.e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_mint '(record{
to= null;
memo= null;
name= "ICP FLOWER X";
description= null;
token_ids= vec{ 1: nat; 3: nat; 5: nat; 7: nat; 9: nat};
image= null;
})'
```

`icrc7_approve : (ApprovalArgs) -> (Result);`
This function is use for approving

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_approve '(record{
from_subaccount= null;
memo= null;
token_ids= variant { Collection= null };
created_at_time= null;
expires_at= null;
spender= record{
owner= principal "b3rxd-5hvkm-rtslf-wotta-lpn2a-krcqh-ipnd5-d2jyy-wa6vr-6gxmf-xae";
subaccount= null;
};
})'
```

`icrc7_transfer : (TransferArgs) -> (Result_2);`
This function is use for transfer

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_transfer '(record{
to= record{
owner= principal "o2ivq-5dsz3-nba5d-pwbk2-hdd3i-vybeq-qfz35-rqg27-lyesf-xghzc-3ae";
subaccount= null;
};
spender_subaccount= null;
from= record{
owner= principal "5jdft-s4ak7-m5iln-x3c75-v67oi-554v3-qllyh-ovnrl-5pxwe-jlbl7-tae";
subaccount = null;
};
memo= null;
is_atomic= null;
token_ids= vec{ 3: nat; 5: nat};
created_at_time= null;
})'
```

`icrc7_collection_metadata : () -> (Icrc7CollectionMetadata) query;`
This function returns Icrc7 colletion's metadata

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_collection_metadata
```

`icrc7_balance_of : (Account) -> (nat) query;`
This function returns balance of the user

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_balance_of '(record{
  owner= principal "xxxx-yyyy-zzzz";
  subaccount= null
})'
```

`icrc7_description : () -> (opt text) query;`
This function returns description of the icrc7 canister

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_balance_of '(record{
  owner= principal "xxxx-yyyy-zzzz";
  subaccount= null
})'
```

`icrc7_metadata : (nat) -> (vec record { text; MetadataValue }) query;`
This function returns metadata of the token

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_balance_of '(record{
  owner= principal "xxxx-yyyy-zzzz";
  subaccount= null
})'
```

`icrc7_name : () -> (text) query;`
This function returns name of icrc7 canister

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_name
```

`icrc7_owner_of : (nat) -> (opt Account) query;`
This function returns owner of the token

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_owner_of '(1: nat)'
```

`icrc7_royalty_recipient : () -> (opt Account) query;`
This function returns royalty recipient

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_royalty_recipient
```

`icrc7_suggested_royalties : () -> (opt nat16) query;`
This function returns royalties

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_suggested_royalties
```

`icrc7_supply_cap : () -> (opt nat) query;`
This function returns supply cap

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_supply_cap
```

`icrc7_supported_standards : () -> (vec Standard) query;`
This function returns standards

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_supported_standards
```

`icrc7_symbol : () -> (text) query;`
This function returns symbol of icrc7 canister

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_symbol
```

`icrc7_tokens_of : (Account) -> (vec nat) query;`
This function returns list of token owned by account

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_tokens_of '(record{
  owner= principal "xxxx-yyyy-zzzz";
  subaccount= null
})'
```

`icrc7_total_supply : () -> (nat) query;`
This function returns total supply

`e.g.` calling the function
```bash
dfx canister call icrc7 icrc7_total_supply
```
