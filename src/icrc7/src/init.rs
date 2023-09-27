use ic_cdk_macros::{init, pre_upgrade, post_upgrade};
use ic_stable_structures::{writer::Writer, Memory};

use crate::{types::InitArgs, state::COLLECTION};

#[init]
pub fn init(args: InitArgs) {
    COLLECTION.with(|c|{
        let mut c = c.borrow_mut();
        c.icrc7_name = args.icrc7_name;
        c.icrc7_description = args.icrc7_description;
        c.icrc7_logo = args.icrc7_logo;
        c.icrc7_royalties = args.icrc7_royalties;
        c.icrc7_royalty_recipient = args.icrc7_royalty_recipient;
        c.icrc7_supply_cap = args.icrc7_supply_cap;
        if let Some(auth) = args.minting_authority{
            c.minting_authority = auth;
        }else{
            c.minting_authority = ic_cdk::caller();
        }
        c.permitted_drift = args.permitted_drift * 60 * 1000_000_000;
        c.tx_window = args.tx_window * 60 * 60 * 1000_000_000;
    })
}

// A pre-upgrade hook for serializing the data stored on the heap.
#[pre_upgrade]
fn pre_upgrade() {
    // Serialize the state.
    // This example is using CBOR, but you can use any data format you like.
    let mut state_bytes = vec![];
    COLLECTION.with(|s| ciborium::ser::into_writer(&*s.borrow(), &mut state_bytes))
        .expect("failed to encode state");

    // Write the length of the serialized bytes to memory, followed by the
    // by the bytes themselves.
    let len = state_bytes.len() as u32;
    let mut memory = crate::memory::get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap();
}

// A post-upgrade hook for deserializing the data back into the heap.
#[post_upgrade]
fn post_upgrade() {
    let memory = crate::memory::get_upgrades_memory();

    // Read the length of the state bytes.
    let mut state_len_bytes = [0; 4];
    memory.read(0, &mut state_len_bytes);
    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    // Read the bytes
    let mut state_bytes = vec![0; state_len];
    memory.read(4, &mut state_bytes);

    // Deserialize and set the state.
    let state = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    COLLECTION.with(|s| {
        *s.borrow_mut() = state
    });
}