use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static EXAMPLE_MAP: RefCell<StableBTreeMap<u64, u64, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
    ));
}

#[ic_cdk::query]
fn get_participation(key: u64) -> Option<u64> {
    EXAMPLE_MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk::query]
fn get_item_num() -> u64 {
    EXAMPLE_MAP.with(|p| p.borrow().len())
}

#[ic_cdk::update]
fn insert_participation(key: u64, value: u64) -> Option<u64> {
    EXAMPLE_MAP.with(|p| p.borrow_mut().insert(key, value))
}
