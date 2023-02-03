use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::record::{Index, MemChange};
use crate::smt::BitVec;
use crate::traits::MachineComponent;

#[derive(Clone, Debug, Default)]
pub struct Memory {
    pub(crate) inner: Vec<BitVec<1>>,
}

impl MachineComponent for Memory {
    type Record = MemChange;

    fn apply_change(&mut self, rec: Self::Record) {}
}

impl Memory {
    pub fn read(&self, idx: Index) -> BitVec<1> {
        todo!()
    }
    pub fn read_word(&self, idx: Index) -> BitVec<32> {
        todo!()
    }

    pub fn write_word(&mut self, idx: Index, word: BitVec<32>) {
        todo!()
    }
}

// impl Memory {
//     pub fn init() -> Self {
//
//     }
//     pub fn store_byte(&mut self, idx: Index, val: BitVec<8>) {
//         self.inner.insert(idx, val);
//     }
// }
