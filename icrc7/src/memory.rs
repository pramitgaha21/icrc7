use crate::state::{TokenMap, TxnLog, MEMORY_MANAGER};
use ic_stable_structures::{
    memory_manager::{MemoryId, VirtualMemory},
    DefaultMemoryImpl,
};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

pub fn get_upgrade_memory() -> Memory {
    MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(0)))
}

pub fn get_state_memory() -> Memory {
    MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(1)))
}

pub fn get_token_map() -> TokenMap {
    TokenMap::init(MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(2))))
}

pub fn get_txn_log() -> TxnLog {
    TxnLog::init(MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(3))))
}
