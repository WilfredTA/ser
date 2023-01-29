use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use z3_ext::{
    ast::{Ast, Bool, Int, BV},
    AstKind, Config, Context, Model, SatResult, Solver,
};

use crate::{record::*, stack::Stack, smt::{BitVec, ctx}, MachineComponent};
use crate::instruction::{Instruction, MachineInstruction};
use crate::memory::*;


pub trait MachineState {
    type PC;
    fn pc(&self) -> Self::PC;
    fn stack(&self) ->  &Stack<32>;
    fn stack_push(&mut self, val: BitVec<32>);
    fn stack_pop(&mut self) -> BitVec<32>;
    fn mem(&self) -> &Memory;
    fn mem_write(&mut self, idx: Index, val: BitVec<32>);
    fn mem_read(&self, idx: Index) -> BitVec<32>;
    fn stack_apply(&mut self, stack_rec: StackChange);
    fn mem_apply(&mut self, mem_rec: MemChange);

}


pub struct ExecutionSummary {
    reachable: Vec<EvmState>,
}

impl ExecutionSummary {
    pub fn new() -> Self {
        Self {
            reachable: vec![]
        }
    }

    pub fn with_state(state: EvmState) -> Self {
        Self {
            reachable: vec![state]
        }
    }

    pub fn with_states(states: Vec<EvmState>) -> Self {
        Self {
            reachable: states
        }
    }

    pub fn falsify<'ctx>(&self, assertion: Bool<'ctx>) -> bool {
        todo!()
    }

    pub fn rewind(&self, steps: usize) -> Self {
        todo!()
    }
}


pub trait Machine: MachineComponent {
    type State: MachineState;

    // All possible final states
    fn exec(&self) -> ExecutionSummary;
    fn pgm(&self) -> Vec<Instruction>;
    fn instruction(&self) -> Instruction;
    fn state(&self) -> Self::State;
    fn state_ref(&self) -> &Self::State;
    fn state_ref_mut(&mut self) -> &mut Self::State;
    fn path_conditions<'ctx>(&self) -> Vec<Bool<'ctx>>;
}



#[derive(Clone, Debug)]
pub struct StateTree<'ctx> {
    pub(crate) val: EvmState,
    pub(crate) path_condition: Bool<'ctx>,
    pub(crate) left: Option<Box<StateTree<'ctx>>>,
    pub(crate) right: Option<Box<StateTree<'ctx>>>,
}


impl<'ctx> StateTree<'ctx> {
    pub fn update(&self, val: EvmState) -> StateTree<'ctx> {
        let mut new_self = self.clone();
        new_self.val = val;
        new_self
    }

    pub fn update_mut(&mut self, val: EvmState) {
        self.val = val;
    }

    pub fn push(&mut self, val: EvmState, constraint: Bool<'ctx>) {
        if self.left.is_none() {
            self.left = Some(Box::new(StateTree {
                val,
                path_condition: constraint,
                left: None,
                right: None
            }));
        } else if self.right.is_none() {
            self.right = Some(Box::new(StateTree {
                val,
                path_condition: constraint,
                left: None,
                right: None
            }));
        } else if let Some(left) = &mut self.left {
            // This ensures that the constraints of each node is a conjunction of all of its ancestors constraints + the new branch condition.
            let new_constraint = Bool::and(ctx(), &[&self.path_condition, &constraint]);
            left.push(val, new_constraint);
        } else {
            panic!("Failed to insert new state into state tree. This should never happen");
        }
    }
}

impl<'ctx> Iterator for StateTree<'ctx> {
    type Item = (EvmState, Bool<'ctx>);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}



#[derive(Clone, Debug)]
pub struct EvmState{
    memory: Memory,
    stack: Stack<32>,
    pc: usize,
}

pub struct Evm<'ctx> {
    pgm: Vec<Instruction>,
    states: StateTree<'ctx>
}

impl MachineComponent for Evm<'_> {
    type Record = MachineRecord;

    fn apply_change(&mut self, rec: Self::Record) {
        let MachineRecord {pc, stack, mem, constraints} = rec;
        let mut state = self.states.val.clone();
        if let Some(stack_rec) = stack {
            state.stack_apply(stack_rec);
        }
    
        if let Some(mem_rec) = mem {
            state.mem_apply(mem_rec);
        }
     
        
        
        if constraints.is_none() {
            // Assert this because pgm counter always increments during execution except
            // for when a jump occurs. And jumps should always result in a constraint
            assert!(pc.1 == (pc.0 + 1));
            state.pc = pc.1;
            self.states.update_mut(state);
        } else {
            let constraint = constraints.unwrap();
            let mut does_jump_state = state.clone();
            does_jump_state.pc = pc.1;
            state.pc += 1;
            // Insert possible machine states such that:
            // - The leftmost path of the tree represents the straightline execution of the program with no branches.
            // - At each branch, we insert the condition of the branching and its negation
            self.states.push(state.clone(), constraint.not());
            self.states.push(does_jump_state, constraint);
  
        }   
    }
}


impl MachineState for EvmState {
    type PC = usize;

    fn pc(&self) -> Self::PC {
        self.pc
    }

    fn stack(&self) ->  &Stack<32> {
        &self.stack
    }

    fn stack_push(&mut self, val: BitVec<32>) {
       self.stack.push(val);
    }

    fn stack_pop(&mut self) -> BitVec<32> {
        self.stack.pop()
    }

    fn mem(&self) -> &Memory {
        &self.memory
    }

    fn mem_write(&mut self, idx: Index, val: BitVec<32>) {
        self.memory.inner.insert(idx, val);
    }

    fn mem_read(&self, idx: Index) -> BitVec<32> {
        self.memory.inner.get(&idx).cloned().unwrap_or_default().clone()
    }

    fn stack_apply(&mut self, stack_rec: StackChange) {
        self.stack.apply_change(stack_rec);
    }

    fn mem_apply(&mut self, mem_rec: MemChange) {
        self.memory.apply_change(mem_rec);
    }
}