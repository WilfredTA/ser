use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use z3_ext::{
    ast::{Ast, Bool, Int, BV},
    AstKind, Config, Context, Model, SatResult, Solver,
};

use crate::{record::*, stack::Stack, smt::{BitVec, ctx}, MachineComponent, bvi, bvc};
use crate::instruction::{Instruction, MachineInstruction};
use crate::memory::*;


pub trait MachineState<const StackItemSZ: u32> {
    type PC;
    
    fn pc(&self) -> Self::PC;
    fn stack(&self) ->  &Stack<32>;
    fn stack_push(&mut self, val: BitVec<StackItemSZ>);
    fn stack_pop(&mut self) -> BitVec<StackItemSZ>;
    fn mem(&self) -> &Memory;
    fn mem_write(&mut self, idx: Index, val: BitVec<32>);
    fn mem_read(&self, idx: Index) -> BitVec<32>;
    fn stack_apply(&mut self, stack_rec: StackChange<StackItemSZ>);
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


pub trait Machine<const StackItemSZ: u32>: MachineComponent {
    type State: MachineState<StackItemSZ>;

    // All possible final states
    fn exec(&self) -> ExecutionSummary;
    fn pgm(&self) -> Vec<Instruction>;
    fn instruction(&self) -> Instruction;
    fn state(&self) -> Self::State;
    fn state_ref(&self) -> &Self::State;
    fn state_ref_mut(&mut self) -> &mut Self::State;
    fn path_conditions<'ctx>(&self) -> Vec<Bool>;
}



#[derive(Clone, Debug, Default)]
pub struct StateTree<'ctx> {
    pub(crate) val: EvmState,
    pub(crate) path_condition: Option<Bool<'ctx>>,
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
                path_condition: Some(constraint),
                left: None,
                right: None
            }));
        } else if self.right.is_none() {
            self.right = Some(Box::new(StateTree {
                val,
                path_condition: Some(constraint),
                left: None,
                right: None
            }));
        } else if let Some(left) = &mut self.left {
            let final_constraint = if let Some(cond) = &self.path_condition {
                Bool::and(ctx(), &[&cond, &constraint])
            } else {
                constraint
            };
            // This ensures that the constraints of each node is a conjunction of all of its ancestors constraints + the new branch condition.
            let new_constraint = final_constraint;
            left.push(val, new_constraint);
        } else {
            panic!("Failed to insert new state into state tree. This should never happen");
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct StateTreeIterator<'ctx> {
    curr_state: Option<(EvmState, Option<Bool<'ctx>>)>,
    nexts: Vec<StateTree<'ctx>>
}

impl<'ctx> Iterator for StateTreeIterator<'ctx> {
    type Item = (EvmState,Option<Bool<'ctx>>);

    fn next(&mut self) -> Option<Self::Item> {
        let curr = &self.curr_state;
        let nxt = self.nexts.pop();

        if let Some(nxtt) = nxt {
            if let Some(left) = nxtt.left {
                self.nexts.push(*left);
            }

            if let Some(right) = nxtt.right {
                self.nexts.push(*right);
            }

            Some((nxtt.val, nxtt.path_condition))
        } else {
            None
        }


        // .map(|s| {
            
        //     (s.val, s.path_condition)
        // })
    }
} 
impl<'ctx> IntoIterator for StateTree<'ctx> {
    type Item = (EvmState, Option<Bool<'ctx>>);

    type IntoIter = StateTreeIterator<'ctx>;

    fn into_iter(self) -> Self::IntoIter {
        let (left, right) = (self.left, self.right);
        let mut iterator = StateTreeIterator {
            curr_state: Some((self.val, self.path_condition)),
            nexts: vec![]
        };

        if let Some(left) = left {
            iterator.nexts.push(*left);
        }

        if let Some(right) = right {
            iterator.nexts.push(*right);
        }
        iterator

        
    }
}



#[derive(Clone, Debug, Default)]
pub struct EvmState{
    memory: Memory,
    stack: Stack<32>,
    pc: usize,
}

pub struct Evm<'ctx> {
    pgm: Vec<Instruction>,
    pub states: StateTree<'ctx>
}

impl MachineComponent for Evm<'_> {
    type Record = MachineRecord<32>;

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


impl<'ctx> Evm<'ctx> {

    pub fn new(pgm: Vec<Instruction>) -> Self {
        let evm_state = EvmState::default();
        Self {
            pgm,
            states: Default::default()
        }
    }
    pub fn exec_mut(&mut self) {
        let mut execution_trace = vec![];
        for inst in self.pgm.clone() {
            let record = inst.exec(self);
            self.apply_change(record.clone());
            execution_trace.push(record);
        }

   

        

    }
}

impl<'ctx> Machine<32> for Evm<'ctx> {
    type State = EvmState;

    fn exec(&self) -> ExecutionSummary {
        todo!()
    }

    fn pgm(&self) -> Vec<Instruction> {
        todo!()
    }

    fn instruction(&self) -> Instruction {
        todo!()
    }

    fn state(&self) -> Self::State {
        self.states.val.clone()
        
    }

    fn state_ref(&self) -> &Self::State {
        todo!()
    }

    fn state_ref_mut(&mut self) -> &mut Self::State {
        todo!()
    }

    fn path_conditions(&self) -> Vec<Bool<'ctx>> {
        todo!()
    }
}

impl MachineState<32> for EvmState {
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

    fn stack_apply(&mut self, stack_rec: StackChange<32>) {
        self.stack.apply_change(stack_rec);
    }

    fn mem_apply(&mut self, mem_rec: MemChange) {
        self.memory.apply_change(mem_rec);
    }
}


#[test]
fn machine_returns_one_exec_for_non_branching_pgm() {
    let one = bvi(1);

    let two = bvi(2);
    //let three = BitVec::new_literal(3);
    let four = bvi(4);
    let a = bvc("a");

    /**
     * 
     * 2 pc 0
     * 1 2 pc 1
     * a 1 2 pc 2
     * (a + 1) 2 pc 3
     * 7 (a + 1) 2 pc 4
     * 
     */
    let pgm = vec![
        Instruction::Push(two.clone()),
        Instruction::Push(one),
        Instruction::Push(a),
        Instruction::Add,
        Instruction::Push(bvi(7)),
        // Instruction::Sub,
        // Instruction::Push(four),
    
        Instruction::JumpI,
        Instruction::Push(bvi(24)),
    ];


    let mut evm = Evm::new(pgm);
    evm.exec_mut();
    eprintln!("{:#?}", evm.states);
    assert!(false);
}