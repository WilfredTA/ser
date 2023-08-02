use crate::{
    instruction::Instruction,
    record::{StorageChange, StorageOp},
    traits::MachineComponent,
};
use std::collections::HashMap;
use z3_ext::ast::{Array, Ast, BV};

use crate::smt::{ctx, BitVec};
use crate::{bvc, bvi};

fn make_storage_arr() {}
#[derive(Debug, Clone, Default)]
pub struct AccountStorage {
    inner: HashMap<BitVec<32>, StorageValue>,
    touched: HashMap<BitVec<32>, bool>,
    code: Option<Vec<Instruction>>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StorageValue {
    Array(Array<'static>),
    BV(BitVec<32>),
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
    inner: HashMap<Address, AccountStorage>,
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
    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_address(mut self, addr: Address) -> Self {
        self.inner.insert(addr, Default::default());
        self
    }
    pub fn with_contract(mut self, addr: Address, pgm: Vec<Instruction>) -> Self {
        let mut account = AccountStorage::default();
        account.code = Some(pgm);
        self.inner.insert(addr, account);
        self
    }
}

impl MachineComponent for GlobalStorage {
    type Record = StorageChange;
    fn apply_change(&mut self, rec: Self::Record) {
        let StorageChange { log } = rec;
        let mut addr_record_map = HashMap::new();
        log.iter().for_each(|op| match op {
            crate::record::StorageOp::Read { idx, addr } => {
                addr_record_map
                    .entry(addr)
                    .and_modify(|logs: &mut Vec<StorageOp>| {
                        logs.push(StorageOp::Read {
                            idx: idx.clone(),
                            addr: addr.clone(),
                        })
                    })
                    .or_insert(vec![StorageOp::Read {
                        idx: idx.clone(),
                        addr: addr.clone(),
                    }]);
            }
            crate::record::StorageOp::Write { addr, idx, val } => {
                addr_record_map
                    .entry(addr)
                    .and_modify(|logs: &mut Vec<StorageOp>| {
                        logs.push(StorageOp::Write {
                            idx: idx.clone(),
                            addr: addr.clone(),
                            val: val.clone(),
                        })
                    })
                    .or_insert(vec![StorageOp::Write {
                        idx: idx.clone(),
                        addr: addr.clone(),
                        val: val.clone(),
                    }]);
            }
        });
        addr_record_map
            .into_iter()
            .for_each(|(address, storage_ops_log)| {
                let change = StorageChange {
                    log: storage_ops_log,
                };
                self.inner
                    .entry(address.clone())
                    .and_modify(|account| account.apply_change(change.clone()))
                    .or_insert_with(|| {
                        let mut new_acc = AccountStorage::default();
                        new_acc.apply_change(change);
                        new_acc
                    });
            })
    }
}

impl MachineComponent for AccountStorage {
    type Record = StorageChange;
    fn apply_change(&mut self, rec: Self::Record) {
        let StorageChange { log } = rec;
        log.into_iter().for_each(|op| match op {
            crate::record::StorageOp::Read { idx, addr } => {
                self.touched.insert(idx, true);
            }
            crate::record::StorageOp::Write { addr, idx, val } => {
                self.touched.insert(idx.clone(), true);
                self.inner.insert(idx, StorageValue::BV(val));
            }
        })
    }
}

#[test]
fn test_basic_lookup_works_in_acc_storage() {
    let addr = Address::new_const("Address1");
    let mut acc_store = AccountStorage::default();
    acc_store.sstore(bvi(5), StorageValue::BV(bvc("val_at_idx_5")));

    assert_eq!(
        acc_store.sload(&bvi(5)),
        StorageValue::BV(bvc("val_at_idx_5"))
    );
}

#[test]
fn test_basic_lookup_global_storage() {
    let addr = Address::new_const("Address1");
    let addr2 = Address::new_const("Address2");
    let mut global = GlobalStorage::init_with_addrs(vec![addr, addr2.clone()]);

    let mut addr_2_storage = global.get(&addr2);
    addr_2_storage.sstore(bvi(3), StorageValue::BV(bvc("storage_val_at_idx_3")));
    global.inner.insert(addr2.clone(), addr_2_storage);

    assert_eq!(
        global.get(&addr2).sload(&bvi(3)),
        StorageValue::BV(bvc("storage_val_at_idx_3"))
    );
}

#[test]
fn test_storage_with_solidity_mapping() {}

// Storage keys must be concrete
// However, values can be symbolic
// In the case of compound structures like mappings, the following
// Global Storage:
// HashMap(Address -> AccountStorage)
// AccountStorage(BitVec<32> -> StorageValue)
// StorageValue(ConcreteBytes<32> OR SymbolicBytes<32>)
//
