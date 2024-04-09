use candid::{CandidType, Principal};
use ic_cdk::query;
use ic_stable_structures::{
    memory_manager::VirtualMemory, storable::Bound, DefaultMemoryImpl, RestrictedMemory,
    StableCell, StableLog, Storable,
};
use icrc_ledger_types::icrc3::{
    archive::ArchiveInfo,
    blocks::{DataCertificate, GetBlocksRequest},
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

const WASM_PAGE_SIZE: u64 = 65536;

const GIB: u64 = 1024 * 1024 * 1024;

pub type Memory = RestrictedMemory<DefaultMemoryImpl>;
pub type ConfigCell = StableCell<ArchiveConfig, Memory>;
pub type BLockLog = StableLog<Vec<u8>, VirtualMemory<Memory>, VirtualMemory<Memory>>;

pub fn config_memory() -> Memory {
    RestrictedMemory::new(DefaultMemoryImpl::default(), 0..1)
}

#[derive(Serialize, Deserialize, Default)]
pub struct ArchiveConfig {
    /// The maximum number of bytes archive can use to store encoded blocks.
    max_memory_size_bytes: u64,
    /// The index of the first block in the archive.
    block_index_offset: u64,
    /// The principal of the ledger canister that created this archive.
    /// The archive will accept blocks only from this principal.
    ledger_id: Option<Principal>,
    /// The maximum number of transactions returned by [get_transactions].
    max_transactions_per_response: u64,
}

impl Storable for ArchiveConfig {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut buf = vec![];
        ciborium::ser::into_writer(self, &mut buf).expect("Failed to serialize using ciborium");
        std::borrow::Cow::Owned(buf)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        ciborium::de::from_reader(&bytes[..]).expect("Failed to deserialize using ciborium")
    }

    const BOUND: Bound = Bound::Unbounded;
}

pub type GetTransactionRequest = GetBlocksRequest;

pub fn init() {}

pub fn append_block() {}

pub fn get_transaction() {}

pub fn get_transactions() {}

pub fn icrc3_get_blocks() {}

#[derive(CandidType, Deserialize, Debug)]
pub struct GetArchiveArgs {
    pub from: Option<Principal>,
}

#[query]
pub fn icrc3_get_archives(_arg: GetArchiveArgs) -> Vec<ArchiveInfo> {
    vec![]
}

#[query]
pub fn icrc3_get_tip_certificate() -> Option<DataCertificate> {
    // Only the Ledger certifies the tip of the chain.
    None
}

#[derive(CandidType, Debug)]
pub struct BlockType {
    pub block_type: String,
    pub url: String,
}

#[query]
pub fn icrc3_supported_block_types() -> Vec<BlockType> {
    vec![
        BlockType {
            block_type: "7mint".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
        },
        BlockType {
            block_type: "7burn".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
        },
        BlockType {
            block_type: "7x_fer".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
        },
        BlockType {
            block_type: "7update_token".into(),
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md".into(),
        },
    ]
}

thread_local! {
     /// Static configuration of the archive that init() sets once.
    static CONFIG: RefCell<ConfigCell> = RefCell::new(ConfigCell::init(
        config_memory(),
        ArchiveConfig::default(),
    ).expect("failed to initialize stable cell"));
    // static BLOCK: RefCell<BLockLog> = RefCell::new()
}

ic_cdk::export_candid!();
