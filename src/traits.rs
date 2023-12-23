use std::error::Error;

use crate::exec::Execution;
use crate::instruction::Instruction;
use crate::machine::{ExecBranch, ExecutionSummary};
use crate::memory::Memory;
use crate::parser::Program;
use crate::record::{Index, MachineRecord, MemChange, StackChange, StorageChange};
use crate::smt::BitVec;
use crate::stack::Stack;
use crate::state::context::ExecutionEnv;
use crate::state::evm::EvmState;
use crate::storage::{AccountStorage, StorageValue};
use uuid::Uuid;
use z3_ext::ast::Bool;


pub trait NodeLike {
    fn parent(&self) -> &Self;
}
pub trait Tree {
    type Node;
    fn insert_left_of(&mut self, id: Uuid, node: Self::Node) -> Option<Uuid>;
    fn insert_right_of(&mut self, id: Uuid, node: Self::Node) -> Option<Uuid>;
    fn leaves(&self) -> Vec<Self::Node>;
    fn path_to(&self, id: Uuid) -> Vec<Self::Node>;
    fn all_paths(&self) -> Vec<Vec<Self::Node>>;
}
pub trait MachineState<const STACK_ITEM_SZ: usize> {
    type PC;

    fn pc(&self) -> Self::PC;
    fn stack(&self) -> &Stack<32>;
    fn stack_push(&mut self, val: BitVec<STACK_ITEM_SZ>);
    fn stack_pop(&mut self) -> BitVec<STACK_ITEM_SZ>;
    fn mem(&self) -> &Memory;
    fn mem_write(&mut self, idx: Index, val: BitVec<32>);
    fn mem_read(&self, idx: Index) -> BitVec<32>;
    fn stack_apply(&mut self, stack_rec: StackChange<STACK_ITEM_SZ>);
    fn mem_apply(&mut self, mem_rec: MemChange);
    fn storage(&self) -> &AccountStorage;
    fn storage_write(&mut self, idx: Index, val: StorageValue);
    fn storage_read(&self, idx: &Index) -> StorageValue;
    fn storage_apply(&mut self, storage_rec: StorageChange);
}

pub trait Machine<const STACK_ITEM_SZ: usize> {
    type State: MachineState<STACK_ITEM_SZ>;
    
    // All possible final states
    fn exec(&mut self) -> Execution;
    fn pgm(&self) -> Program;
    fn state(&self) -> Self::State;
    fn state_ref(&self) -> &Self::State;
    fn state_ref_mut(&mut self) -> &mut Self::State;
}

pub trait MachineInstruction<'ctx, const SZ: usize> {
    type Error;
    fn exec(&self, mach: &EvmState, env: &ExecutionEnv) -> MachineRecord<SZ>;
}

pub trait MachineComponent {
    type Record;
    fn apply_change(&mut self, rec: Self::Record);
}
