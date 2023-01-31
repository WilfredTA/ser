use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};
use std::collections::HashMap;
use uuid::Uuid;

use z3_ext::{
    ast::{Ast, Bool, Int, BV},
    AstKind, Config, Context, Model, SatResult, Solver,
};

use crate::{record::*, stack::Stack, smt::{BitVec, ctx}, bvi, bvc};
use crate::instruction::{iadd, Instruction, ipush};
use crate::memory::*;
use crate::state::evm::EvmState;
use crate::state::tree::{NodeId, StateTree};
use crate::traits::{Machine, MachineComponent, MachineState, MachineInstruction};


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








pub type ExecBranch<'ctx> = (EvmState, Vec<Bool<'ctx>>);
#[derive(Clone)]
pub struct Evm<'ctx> {
    pgm: Vec<Instruction>,
    pub states: StateTree<'ctx>,
    change_log: Vec<MachineRecord<32>>,
    pub inverse_state: HashMap<Uuid, Uuid>
}

impl<'ctx> Evm<'ctx> {
    // Given a machine (which has its internal state tree) & a machine record
    // this method returns a new state tree containing all possible new machine states
    // that would result from taking the step represented by the machine record
    // fn state_transition(tree: StateTree<'ctx>, rec: MachineRecord<32>) -> StateTree {
    //     let MachineRecord {pc, stack, mem, constraints, halt} = rec.clone();
    //     let mut curr_node = tree.val.clone();
    //     let mut new_state = tree.clone();
    //     eprintln!("STACK BEFORE STATE TRANSITION: {:#?}", curr_node.stack());
    //
    //     let (straight_exec, jump_exec) = curr_node.state_transition(rec);
    //     let (left_mach, mut left_mach_new_conds) = straight_exec;
    //     eprintln!("STACK AFTER STATE TRANSITION: {:#?}", left_mach.stack());
    //     if let Some(conds) = tree.path_condition {
    //         left_mach_new_conds.push(conds.clone());
    //         let cond_slice = left_mach_new_conds.iter().map(|c| c).collect::<Vec<_>>();
    //         new_state.insert_left((left_mach,Bool::and(ctx(), &cond_slice.as_slice())));
    //     }
    //
    //
    //     if let Some(jump_state) = jump_exec {
    //         new_state.insert_right((jump_state.0, constraints));
    //     }
    //     new_state
    //
    //     todo!()
    // }
}


impl MachineComponent for Evm<'_> {
    type Record = MachineRecord<32>;

    fn apply_change(&mut self, rec: Self::Record) {
        todo!()
       // self.states = Evm::state_transition(self.states.clone(), rec);
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
            states: StateTree { id: NodeId::new(), val: evm_state, path_condition:None, left: None, right: None },
            change_log: vec![],
            inverse_state: Default::default()
        }
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


     fn exec(&mut self) ->  Vec<ExecBranch<'ctx>> {
        let mut curr_state = self.states.val.clone();
        let curr_id = self.states.id.clone();

         let mut jump_ctx = vec![curr_id.id()];
        let mut state_tree = self.states.clone();
        let mut trace:Vec<ExecBranch> = vec![(curr_state.clone(), vec![])];
        let mut leaves: Vec<ExecBranch> = vec![];
        // let mut right_branch_ptr = curr_id.id();
         //let mut left_branch_ptr = curr_id.id();

        loop {
            let curr_state = trace.pop();
            let mut temp_id_ptr = jump_ctx.pop();
            eprintln!("Executing node with id {:?}", temp_id_ptr);
            if let Some(curr_state) = curr_state {
                let temp_id_ptr = temp_id_ptr.unwrap();

                let (curr_state, curr_cond) = curr_state;
                let mut curr_cond = curr_cond.clone();
                let mut branch_cond_pre = curr_cond.clone();
                let (next_state, next_state_branch) = curr_state.exec_once();
                if let Some(branch) = next_state_branch {
                    let (branch_state, branch_cond) = branch;
                    branch_cond_pre.extend(branch_cond);
                    let branch = (branch_state.clone(), branch_cond_pre.clone());

                    eprintln!("Inserting to the right of {:?}", temp_id_ptr);
                    let branch_right_id = state_tree.insert_right_of(branch.clone(), temp_id_ptr);
                    eprintln!("Inserted {} to the right of {}", branch_right_id, temp_id_ptr);


                    jump_ctx.push(branch_right_id);
                    if branch_state.can_continue() {

                        trace.push(branch);

                    } else {
                        eprintln!("NODE {} CANNOT CONTINUE", branch_right_id);
                        leaves.push(branch);
                    }
                }
                let (nxt_state, nxt_constraints) = next_state;
                curr_cond.extend(nxt_constraints);
                let branch = (nxt_state.clone(), curr_cond.clone());
                eprintln!("Inserting to the left of {:?}", temp_id_ptr);


                let branch_left_id = state_tree.insert_left_of(branch.clone(), temp_id_ptr);
                eprintln!("Inserted {} to the left of {}", branch_left_id, temp_id_ptr);
                if nxt_state.can_continue() {
                    trace.push(branch);
                    jump_ctx.push(branch_left_id);
                } else {
                    leaves.push(branch);
                }
            } else {
                break;
            }


        }
        self.states = state_tree;
         leaves
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


pub struct EvmExecutor<'ctx> {
    pub inner: Evm<'ctx>
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

    // Two reachable states; one where 50 (0x32) is on top of stack, and one where 100 (0x64)
    // is on top of stack
    let pgm = vec![
        ipush(two.clone()),
        ipush(one),
        ipush(a),
        iadd(),
        ipush(bvi(7)),
         Instruction::JumpI,
         Instruction::Push(bvi(100)),
         ipush(bvi(50))
    ];


    let mut evm = Evm::new(pgm);
    {
        let mut evm_trace = evm.exec();
        eprintln!("FINAL STATES: {:#?}", evm_trace);
    }

    eprintln!("FINAL STATE TREE: {:#?}", evm.states);
    eprintln!("Leaves: {:#?}", evm.states.leaves());
    assert!(false);
}