const test = require("tape");
const { Ed25519KeyIdentity } = require("@dfinity/identity");
const { Principal } = require("@dfinity/principal");
const { encodeIcrcAccount } = require("@dfinity/ledger");
const { getActor } = require("./actor.cjs");

const canister_ids = require("./../.dfx/local/canister_ids.json");

const factory = canister_ids.factory.local;

let factory_actors = {};

const subaccount = Uint8Array(32).fill(1);

let minter = Ed25519KeyIdentity.generate();
let user1 = Ed25519KeyIdentity.generate();
let user2 = Ed25519KeyIdentity.generate();
let user3 = Ed25519KeyIdentity.generate();
let user4 = Ed25519KeyIdentity.generate();
let user5 = Ed25519KeyIdentity.generate();

let minterAccount = encodeIcrcAccount({ owner: minter.getPrincipal(), subaccount: [subaccount ]});
let user1Account = encodeIcrcAccount({ owner: user1.getPrincipal(), subaccount: [subaccount ]});
let user2Account = encodeIcrcAccount({ owner: user2.getPrincipal(), subaccount: [subaccount ]});
let user3Account = encodeIcrcAccount({ owner: user3.getPrincipal(), subaccount: [subaccount ]});
let user4Account = encodeIcrcAccount({ owner: user4.getPrincipal(), subaccount: [subaccount ]});
let user5Account = encodeIcrcAccount({ owner: user5.getPrincipal(), subaccount: [subaccount ]});

test("Prints the name", async function(t){
    const response = await icrc7.icrc7_name();
    console.log(response);
})

test("Prints the symbol", async function(t){
    const response = await icrc7.icrc7_symbol();
    console.log(response);
})

test("Prints the ", async function(t){
    const response = await icrc7.icrc7_name();
    console.log(response);
})