use std::{collections::HashMap, cell::RefCell};

use candid::Principal;

#[derive(Default)]
pub struct State{
    pub canisters: HashMap<Principal, Principal>,
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}