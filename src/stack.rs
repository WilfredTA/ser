use crate::record::StackChange;

use super::smt::*;
use crate::traits::MachineComponent;
use z3_ext::{
    ast::{Ast, BV},
    Config,
};

#[derive(Debug, Clone)]
pub struct Stack<const SZ: u32> {
    stack: [BitVec<SZ>; 1025],
    top: usize,
}

const INIT_STACK_VAL: Option<BitVec<32>> = None;
impl<const SZ: u32> Default for Stack<SZ> {
    fn default() -> Self {
        let stack = [(); 1025].map(|_| BitVec::default());
        Self { stack, top: 1 }
    }
}
impl<const SZ: u32> Stack<SZ> {
    pub fn push(&mut self, val: BitVec<SZ>) {
        self.stack[self.top] = val;
        self.top += 1;
    }

    pub fn pop(&mut self) -> BitVec<SZ> {
        if self.top == 0 {
            panic!(
                "Stack top must be positive integer. Instead it is {} with state {:?}",
                self.top,
                self.stack.get(self.top)
            );
        } else {
            self.top -= 1;
            self.stack.get(self.top).cloned().unwrap()
        }
    }

    pub fn peek(&self) -> Option<&BitVec<SZ>> {
        self.stack.get(self.top - 1)
    }

    pub fn size(&self) -> usize {
        self.top + 1
    }

    pub fn peek_nth(&self, n: usize) -> Option<&BitVec<SZ>> {
        self.stack.get(n - 1)
    }
}

impl<const SZ: u32> MachineComponent for Stack<SZ> {
    type Record = StackChange<SZ>;

    fn apply_change(&mut self, rec: Self::Record) {
        let StackChange {
            pop_qty,
            push_qty,
            ops,
        } = rec;

        let mut new_stack = self.clone();

        ops.iter().for_each(|op| match op {
            crate::record::StackOp::Push(v) => new_stack.push(v.clone()),
            crate::record::StackOp::Pop => {
                new_stack.pop();
            }
        });
        self.stack = new_stack.stack;
        self.top = new_stack.top;
    }
}
