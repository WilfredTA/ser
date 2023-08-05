use crate::{bvi, record::StackChange};
use std::fmt::{Debug, Formatter};

use super::smt::*;
use crate::traits::MachineComponent;
use smallvec::SmallVec;
use z3_ext::{
    ast::{Ast, BV},
    Config,
};
#[derive(Default, Debug, Clone)]
pub struct Stack<const SZ: usize> {
    stack: SmallVec<[BitVec<SZ>; 1024]>,
    size: usize,
}

impl<const SZ: usize> Stack<SZ> {
    pub fn push(&mut self, val: BitVec<SZ>) {
        self.size += 1;
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> BitVec<SZ> {
       // eprintln!("STACK SIZE: {} AND STACK TOP {:#?}", self.size, self.peek());
        self.size -= 1;
        self.stack.pop().unwrap()
    }

    pub fn peek(&self) -> Option<&BitVec<SZ>> {
        if (self.size) >= self.stack.len() {
            eprintln!("ERROR: STACK SIZE IS INCONSISTENT WITH INTERNAL STACK... Stack size: {}, internal stack len: {}, stack: {:#?}", 
            self.size,
            self.stack.len(),
            self.stack
        );
        }
        if self.size == 0 {
            self.stack.get(0)
        } else {

            self.stack.get(self.size - 1)
        }
    }

    pub fn size(&self) -> usize {
        assert!(self.stack.len() == self.size);
        self.stack.len()
    }

    // where n = 0 is top of the stack
    pub fn peek_nth(&self, n: usize) -> Option<&BitVec<SZ>> {
        if n >= self.size() {
            return None;
        }
        self.stack.get(self.size - n - 1)
    }

    // where n is stack modulo top element;
    pub(crate) fn swap_nth(&mut self, swap_depth: usize) {
        //eprintln!("SWAP EXECUTING AT DEPTH: {}", swap_depth);
        if swap_depth < self.size() {
            let mut new_stack = self.stack.clone();
            let top_idx = self.size - 1;
            let swap_idx = self.size - swap_depth - 1;
            let top = self.peek().cloned().unwrap();
            let swapped = self.peek_nth(swap_depth).cloned().expect(&format!(
                "stack too deep to swap with depth {}. Stack size: {}",
                swap_depth,
                self.size()
            ));
            //eprintln!("STACK BEFORE SWAP: {:#?}", new_stack);
            new_stack = new_stack
                .into_iter()
                .enumerate()
                .map(move |(idx, val)| {
                    if idx == swap_idx {
                        return top.clone();
                    } else if idx == top_idx {
                        return swapped.clone();
                    } else {
                        val
                    }
                })
                .collect::<SmallVec<_>>();
            //eprintln!("STACK AFTER SWAP: {:#?}", new_stack);
            // new_stack.remove(swap_idx);
            // new_stack.insert(swap_idx, top);
            // new_stack.pop();
            // new_stack.push(swapped);
            self.stack = new_stack;
        } else {
            eprintln!("WILL NOT SWAP");
        }
    }

    pub fn peek_top<const N: usize>(&self) -> Option<[&BitVec<SZ>; N]> {
        if self.size() < N {
            return None;
        }

        Some(std::array::from_fn(|i| self.peek_nth(i).unwrap()))
    }
}

impl<const SZ: usize> MachineComponent for Stack<SZ> {
    type Record = StackChange<SZ>;

    fn apply_change(&mut self, rec: Self::Record) {
        let StackChange {
            pop_qty,
            push_qty,
            ops,
            swap_depth,
        } = rec;

        let mut new_stack = self.clone();

        ops.iter().for_each(|op| match op {
            crate::record::StackOp::Push(v) => new_stack.push(v.clone()),
            crate::record::StackOp::Pop => {
                new_stack.pop();
            }
            crate::record::StackOp::Swap(depth) => {}
        });

        if swap_depth > 0 {
            // eprintln!(
            //     "SWAP OCCURRING of DEPTH {}. STACK BEFORE: {:#?}",
            //     swap_depth, new_stack
            // );
            new_stack.swap_nth(swap_depth as usize);
           // eprintln!("STACK AFTER {:#?}", new_stack);
        }

        self.stack = new_stack.stack;
        self.size = new_stack.size;
    }
}

#[test]
fn test_swap() {
    let mut stack: Stack<1> = Stack::default();
    stack.push(bvi(1));
    stack.push(bvi(2));
    stack.push(bvi(3));
    stack.push(bvi(4));
    stack.swap_nth(2);
    assert_eq!(stack.peek().cloned().unwrap(), bvi(2));
    assert_eq!(stack.peek_nth(2).cloned().unwrap(), bvi(4));
}
