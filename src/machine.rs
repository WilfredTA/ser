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

impl<'ctx> From<(EvmState, Bool<'ctx>)> for StateTree<'ctx> {
    fn from(t: (EvmState, Bool<'ctx>)) -> Self {
        Self {
            val: t.0,
            path_condition: Some(t.1),
            left: None,
            right: None
        }
    }
}

impl<'ctx> From<(EvmState, Option<Bool<'ctx>>)> for StateTree<'ctx> {
    fn from(t: (EvmState, Option<Bool<'ctx>>)) -> Self {
        Self {
            val: t.0,
            path_condition: t.1,
            left: None,
            right: None
        }
    }
}

impl<'ctx> StateTree<'ctx> {
    pub fn update(&self, val: EvmState) -> StateTree<'ctx> {
        let mut new_self = self.clone();
        new_self.val = val;
        new_self
    }

    pub fn inorder(&self) -> Vec<(EvmState, Option<Bool<'ctx>>)> {
        let mut items = vec![(self.val.clone(), self.path_condition.clone())];

        if let Some(left) = &self.left {
            let left_tree_inorder = left.inorder();
            items.extend(left_tree_inorder);
        }
        if let Some(right) = &self.right {
            let right_tree_inorder = right.inorder();
            items.extend(right_tree_inorder);
        }
        items
    }

   
    pub fn insert(&mut self, tree: impl Into<StateTree<'ctx>>) {

        if let Some(left) = &mut self.left {
            left.insert(tree);
        } else if let Some(right) = &mut self.right {
            right.insert(tree);
        } else if self.left.is_none() {
            self.left = Some(Box::new(tree.into()));
        } else {
            self.right = Some(Box::new(tree.into()));
        }
    } 

    pub fn insert_left(&mut self, tree: impl Into<StateTree<'ctx>>) {
        if let Some(left) = &mut self.left {
            left.insert_left(tree);
        } else {
            self.left = Some(Box::new(tree.into()));
        }
    }

    pub fn insert_right(&mut self, tree: impl Into<StateTree<'ctx>>) {
        if let Some(left) = &mut self.left {
            if let Some(child_left) = &mut left.left {
                child_left.insert_right(tree);
            } else {
                left.right = Some(Box::new(tree.into()));
            }   
        }
    }

    pub fn leaves(&self) -> Vec<StateTree> {
        let mut leaves = vec![];
        
        if let Some(left) = &self.left {
            if left.left.is_none() && left.right.is_none() {
                leaves.push((left.val.clone(), left.path_condition.clone()).into());
            } else {
                leaves.extend(left.leaves());
            }
        }

        
        if let Some(right) = &self.right {
            if right.right.is_none() && right.left.is_none() {
                leaves.push((right.val.clone(), right.path_condition.clone()).into());
            } else {
                leaves.extend(right.leaves());
            }
        }
        leaves
    }

    pub fn update_mut(&mut self, val: EvmState) {
        self.val = val;
    }

    pub fn push_branch(&mut self, val: EvmState, constraint: Bool<'ctx>) {
         if self.right.is_none() {
            self.right = Some(Box::new(StateTree {
                val,
                path_condition: Some(constraint),
                left: None,
                right: None
            }));
        } else if let Some(left) = &mut self.left {
            left.push_branch(val, constraint)
        }
    }

    pub fn push(&mut self, val: EvmState, constraint: Bool<'ctx>) {
        if self.left.is_none() {
            self.left = Some(Box::new(StateTree {
                val,
                path_condition: Some(constraint),
                left: None,
                right: None
            }));
        } else {
            if let Some(left) = &mut self.left {
                let final_constraint = if let Some(cond) = &self.path_condition {
                    Bool::and(ctx(), &[&cond, &constraint])
                } else {
                    constraint
                };
                // This ensures that the constraints of each node is a conjunction of all of its ancestors constraints + the new branch condition.
                let new_constraint = final_constraint;
                left.push(val, new_constraint);
            }
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
    pgm: Vec<Instruction>
}

impl EvmState {

    pub fn curr_instruction(&self) -> Instruction {
       
        self.pgm.get(self.pc).cloned().unwrap()
    }

    pub fn exec_once(mut self) -> (Self, Option<Self>) {
        let inst = self.curr_instruction();
        let change = inst.exec(&self);
        
        self.state_transition(change)
    }
    // Generates a set of next possible EvmStates given the state change record
    pub fn state_transition(&self, rec: MachineRecord<32>) -> (Self, Option<Self>) {
        let MachineRecord {pc, stack, mem, constraints} = rec;
        let mut new_state = self.clone();
        if let Some(stack_rec) = stack {
            new_state.stack_apply(stack_rec);
        }
    
        if let Some(mem_rec) = mem {
            new_state.mem_apply(mem_rec);
        }

        if constraints.is_none() {
            assert!(pc.1 == (pc.0 + 1));
            new_state.pc = pc.1;
            (new_state, None)
        } else {
            let constraint = constraints.unwrap();
            let mut does_jump_state = new_state.clone();
            does_jump_state.pc = pc.1;
            new_state.pc += 1;
            (new_state, Some(does_jump_state))
        }

    }
}

#[derive(Clone)]
pub struct Evm<'ctx> {
    pgm: Vec<Instruction>,
    pub states: StateTree<'ctx>,
    change_log: Vec<MachineRecord<32>>
}

impl<'ctx> Evm<'ctx> {
    // Given a machine (which has its internal state tree) & a machine record
    // this method returns a new state tree containing all possible new machine states
    // that would result from taking the step represented by the machine record
    fn state_transition(tree: StateTree<'ctx>, rec: MachineRecord<32>) -> StateTree {
        let MachineRecord {pc, stack, mem, constraints} = rec.clone();
        let mut curr_node = tree.val.clone();
        let mut new_state = tree.clone();
        eprintln!("STACK BEFORE STATE TRANSITION: {:#?}", curr_node.stack());

        let (straight_exec, jump_exec) = curr_node.state_transition(rec);
        eprintln!("STACK AFTER STATE TRANSITION: {:#?}", straight_exec.stack());
        new_state.insert_left((straight_exec, tree.path_condition.clone().unwrap_or(Bool::from_bool(ctx(), true))));

        if let Some(jump_state) = jump_exec {
            new_state.insert_right((jump_state, constraints));
        }
        new_state

    }
}
impl MachineComponent for Evm<'_> {
    type Record = MachineRecord<32>;

    fn apply_change(&mut self, rec: Self::Record) {
       self.states = Evm::state_transition(self.states.clone(), rec);
    }

   
}


impl<'ctx> Evm<'ctx> {

    pub fn new(pgm: Vec<Instruction>) -> Self {
        let evm_state = EvmState {
            memory: Default::default(),
            stack: Default::default(),
            pc: 0,
            pgm: pgm.clone(),
        };
        Self {
            pgm,
            states: StateTree { val: evm_state, path_condition:None, left: None, right: None },
            change_log: vec![]
        }
    }

    pub fn exec_once(mut self) -> Self {
        let inst = self.state().curr_instruction();
        let change = inst.exec(&self.state());
        self.change_log.push(change.clone());
        self.states = Evm::state_transition(self.states.clone(), change);
        self
    }

    pub fn exec_mut(&mut self) {
        let mut execution_trace = vec![];
        for inst in self.pgm.clone() {
            let record = inst.exec(&self.state());
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
        self.states.clone().into_iter().last().unwrap_or((self.states.val.clone(), None)).0
        
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
    
        // Instruction::JumpI,
        // Instruction::Push(bvi(24)),
    ];


    let mut evm = Evm::new(pgm);
    evm = evm.exec_once();
    let states = evm.states.inorder();
}