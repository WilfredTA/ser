use crate::smt::BitVec;

use ruint::aliases::*;
use ruint::Uint;
use z3_ext::ast::Bool;

#[derive(Clone, Debug)]
pub struct MachineRecord<const StackItemSZ: u32> {
    pub mem: Option<MemChange>,
    pub stack: Option<StackChange<StackItemSZ>>,
    pub pc: (usize, usize),
    pub constraints: Option<Bool<'static>>,
    pub halt: bool,
}

pub type Index = U256;

#[derive(Default, Clone, Debug)]
pub struct MemChange {
    pub touched_slots: Vec<(Index, BitVec<32>)>,
    pub ops_log: Vec<MemOp>,
}
#[derive(Clone, Debug)]
pub enum MemOp {
    Write { idx: Index, val: BitVec<32> },
    Read { idx: Index },
}
#[derive(Clone, Debug)]
pub enum StackOp<const SZ: u32> {
    Push(BitVec<SZ>),
    Pop,
}

#[derive(Default, Clone, Debug)]
pub struct StackChange<const SZ: u32> {
    pub pop_qty: u64,
    pub push_qty: u64,
    pub ops: Vec<StackOp<SZ>>,
}
