use std::collections::HashMap;
use crate::traits::{MachineComponent};
use z3_ext::{
    ast::{
        BV, Ast, Array
    }
};
use crate::{bvc, bvi};
use crate::smt::{BitVec, ctx};

fn make_storage_arr() {

}
#[derive(Debug, Clone, Default)]
pub struct AccountStorage {
    inner: HashMap<BitVec<32>, StorageValue>
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StorageValue {
    Array(Array<'static>),
    BV(BitVec<32>)
}

impl Default for StorageValue {
    fn default() -> Self {
        Self::BV(Default::default())
    }
}

impl AccountStorage {
    pub fn sstore(&mut self, index: BitVec<32>, val: StorageValue) {
        self.inner.insert(index, val);
    }

    pub fn sload(&self, index: &BitVec<32>) -> StorageValue {
        if let Some(val) = self.inner.get(index) {
            val.clone()
        } else {
            Default::default()
        }
    }
}


pub type Address = BitVec<20>;

#[derive(Debug, Clone, Default)]
pub struct GlobalStorage {
    inner: HashMap<Address, AccountStorage>
}

impl GlobalStorage {
    pub fn init_with_addrs(addrs: Vec<Address>) -> Self {
        let mut store = Self::default();
        addrs.into_iter().for_each(|addr| {
            store.inner.insert(addr, Default::default());
        });
        store
    }

    pub fn get(&self, addr: &Address) -> AccountStorage {
        self.inner.get(addr).cloned().unwrap_or_default()
    }
}

#[test]
fn test_basic_lookup_works_in_acc_storage() {
    let addr = Address::new_const("Address1");
    let mut acc_store = AccountStorage::default();
    acc_store.sstore(bvi(5), StorageValue::BV(bvc("val_at_idx_5")));


    assert_eq!(acc_store.sload(&bvi(5)), StorageValue::BV(bvc("val_at_idx_5")));
}

#[test]
fn test_basic_lookup_global_storage() {
    let addr = Address::new_const("Address1");
    let addr2 = Address::new_const("Address2");
    let mut global = GlobalStorage::init_with_addrs(vec![addr, addr2.clone()]);

    let mut addr_2_storage = global.get(&addr2);
    addr_2_storage.sstore(bvi(3), StorageValue::BV(bvc("storage_val_at_idx_3")));
    global.inner.insert(addr2.clone(), addr_2_storage);

    assert_eq!(global.get(&addr2).sload(&bvi(3)), StorageValue::BV(bvc("storage_val_at_idx_3")));

}



 // Global Storage:
 // HashMap(Address -> AccountStorage)
 // AccountStorage(BitVec<32> -> StorageValue)
 // StorageValue(ConcreteBytes<32> OR SymbolicBytes<32>)
 //
 