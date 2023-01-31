use std::collections::HashMap;

use crate::smt::BitVec;
use crate::record::{Index, MemChange};
use crate::traits::MachineComponent;


#[derive(Clone, Debug, Default)]
pub struct Memory {
    pub(crate) inner: HashMap<Index, BitVec<32>>
}

impl MachineComponent for Memory {
    type Record = MemChange;

    fn apply_change(&mut self, rec: Self::Record) {
        
    }
}