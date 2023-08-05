use crate::smt::BitVec;
use crate::storage::Address;

use ruint::aliases::*;
use ruint::Uint;
use z3_ext::ast::Ast;
use z3_ext::ast::Bool;

#[derive(Clone, Debug)]
pub struct MachineRecord<const STACK_ITEM_SZ: usize> {
    pub mem: Option<MemChange>,
    pub stack: Option<StackChange<STACK_ITEM_SZ>>,
    pub storage: Option<StorageChange>,
    pub pc: (usize, usize),
    pub constraints: Option<Bool<'static>>,
    pub halt: bool,
}

pub type Index = BitVec<32>;
pub type Value = BitVec<32>;

impl From<Index> for usize {
    fn from(idx: Index) -> Self {
        idx.as_ref().as_u64().unwrap() as usize
    }
}
#[derive(Default, Clone, Debug)]
pub struct MemChange {
    pub ops_log: Vec<MemOp>,
}
#[derive(Clone, Debug)]
pub enum MemOp {
    Write { idx: Index, val: BitVec<32> },
    WriteByte { idx: Index, val: BitVec<1> },
    Read { idx: Index },
}
#[derive(Clone, Debug)]
pub enum StackOp<const SZ: usize> {
    Push(BitVec<SZ>),
    Pop,
    Swap(usize),
}

#[derive(Default, Clone, Debug)]
pub struct StackChange<const SZ: usize> {
    pub pop_qty: u64,
    pub push_qty: u64,
    pub swap_depth: usize,
    pub ops: Vec<StackOp<SZ>>,
}

impl<const SZ: usize> StackChange<SZ> {
    pub fn push(val: BitVec<SZ>) -> Self {
        Self {
            pop_qty: 0,
            push_qty: 1,
            swap_depth: 0,
            ops: vec![StackOp::Push(val)],
        }
    }

    pub fn with_ops(ops: Vec<StackOp<SZ>>) -> Self {
        let mut pop_qty = 0;
        let mut push_qty = 0;

        ops.iter().for_each(|op| match op {
            StackOp::Push(_) => push_qty += 1,
            StackOp::Pop => pop_qty += 1,
            _ => (),
        });

        Self {
            push_qty,
            pop_qty,
            swap_depth: 0,
            ops,
        }
    }
}

#[derive(Clone, Debug)]
pub enum StorageOp {
    Read {
        addr: Address,
        idx: Index,
    },
    Write {
        addr: Address,
        idx: Index,
        val: Value,
    },
}

#[derive(Clone, Debug)]
pub struct StorageChange {
    pub log: Vec<StorageOp>,
}

pub fn push<const SZ: usize>(val: BitVec<SZ>) -> StackOp<SZ> {
    StackOp::Push(val)
}
