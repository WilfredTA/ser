use crate::bvi;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use z3_ext::ast::{Ast, BV};

use crate::record::{Index, MemChange, MemOp};
use crate::smt::{BVType, BitVec, SolverType};
use crate::traits::MachineComponent;

#[derive(Clone, Debug, Default)]
pub struct Memory {
    pub(crate) inner: Vec<BitVec<1>>,
    highest_idx: usize,
}

impl MachineComponent for Memory {
    type Record = MemChange;

    fn apply_change(&mut self, rec: Self::Record) {
        let MemChange { ops_log } = rec;
        let mut highest_idx = self.highest_idx;
        ops_log.into_iter().for_each(|op| match op {
            MemOp::Write { val, idx } => {
                let mut val = val;
                val.simplify();
                let mut idx = idx;
                idx.simplify();
                eprintln!(
                    "MEM WRITE FOR MEM APPLY: idx: {:#?}, value: {:#?}",
                    idx, val
                );
                let idx_cmp: usize = idx.clone().into();
                if idx_cmp > highest_idx {
                    highest_idx = idx_cmp;
                }
                self.write_word(idx, val);
            }
            MemOp::Read { idx } => {
                let idx_cmp: usize = idx.into();
                if idx_cmp > highest_idx {
                    highest_idx = idx_cmp;
                }
            }
            MemOp::WriteByte { idx, val } => {
                let idx_cmp: usize = idx.clone().into();
                if idx_cmp > highest_idx {
                    highest_idx = idx_cmp;
                }
                self.write(idx, val);
            }
        })
    }
}

impl Memory {
    pub fn size(&self) -> usize {
        self.inner.len()
    }

    pub fn m_size(&self) -> usize {
        self.highest_idx
    }

    pub fn write(&mut self, idx: Index, val: BitVec<1>) {
        let idx = idx.into();
        self.inner.insert(idx, val);
        // Pad rest with zero
        for i in 0..30 {
            self.inner.push(bvi(0));
        }
    }
    pub fn read(&self, idx: Index) -> BitVec<1> {
        let idx: usize = idx.into();
        let val = self.inner.get(idx).unwrap().clone();
        val
    }
    pub fn read_word(&self, idx: Index) -> BitVec<32> {
        let mut i = 0;
        let idx: usize = idx.into();
        let mut bytes = vec![];
        let mut mem = self.inner.clone();
        eprintln!(" MEM IN READ WORD: {:#?}", mem);
        while i < 32 {
            let idx = idx + 31;
            let val = mem.get(idx - i).unwrap().as_ref().clone();
            eprintln!(
                "MEM VAL IN READ WORD FOR IDX - i:\nmem loc {:#?}\nval: {:#?}",
                (idx - i),
                val
            );
            bytes.push(val);
            i += 1;
        }
        let mut new_bv: BitVec<1> = BitVec::default();
        let mut new_bv_inner = new_bv.as_ref().clone();
        bytes.into_iter().enumerate().for_each(|(i, b)| {
            if i == 0 {
                new_bv = BitVec::with_bv(b);
                new_bv_inner = new_bv.as_ref().clone();
            } else {
                new_bv_inner = new_bv_inner.concat(&b);
            }
        });
        BitVec {
            inner: BVType::Z3(new_bv_inner),
            typ: SolverType::Z3,
        }
    }

    pub fn write_word(&mut self, idx: Index, word: BitVec<32>) {
        let idx = idx.into();
        if idx > self.size() {
            for i in 0..idx - self.size() {
                self.inner.push(BitVec::default());
            }
        }
        //eprintln!("WORD: {word:#?}");
        for i in 0..32 {
            let ii = 32 - i - 1;
            let bv = word.as_ref().extract(ii * 8 + 7, ii * 8).simplify();
            // eprintln!("Extracted: {:#?}", bv);

            let bv: BitVec<1> = bv.into();
            //eprintln!("Extracted size: {:#?}", bv.as_ref().get_size());
            self.inner.insert(idx, bv);
        }
    }
}
