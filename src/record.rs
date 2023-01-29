use crate::smt::BitVec;


use ruint::Uint;
use ruint::aliases::*;
use z3_ext::ast::Bool;

pub struct MachineRecord {
    pub mem: MemChange,
    pub stack: StackChange,
    pub pc: (usize, usize),
    pub constraints: Option<Bool<'static>>
}

pub type Index = U256;

#[derive(Default)]
pub struct MemChange {
    pub touched_slots: Vec<(Index, BitVec<32>)>,
    pub ops_log: Vec<MemOp>
}

pub enum MemOp {
    Write {
        idx: Index,
        val: BitVec<32>
    },
    Read {
        idx: Index
    }
}

pub enum StackOp {
    Push(BitVec<32>),
    Pop
}

#[derive(Default)]
pub struct StackChange {
    pub pop_qty: u64,
    pub push_qty: u64,
    pub ops: Vec<StackOp>

}

