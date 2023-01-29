use crate::{MachineComponent, record::StackChange};

use super::smt::*;
use z3_ext::{
    ast::{Ast, BV},
    Config,
};
#[derive(Default, Debug, Clone)]
pub struct Stack<const SZ: u32> {
    stack: Vec<BitVec<SZ>>,
}

impl<const SZ: u32> Stack<SZ> {
    pub fn push(&mut self, val: BitVec<SZ>) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> BitVec<SZ> {
        self.stack.pop().unwrap()
    }

    pub fn peek(&self) -> Option<&BitVec<SZ>> {
        self.stack.last()
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn peek_nth(&self, n: usize) -> Option<&BitVec<SZ>> {
        self.stack.get(n)
    }
}



impl<const SZ: u32>  MachineComponent for Stack<SZ> {
    type Record = StackChange;

    fn apply_change(&mut self, rec: Self::Record) {
        todo!()
    }
}