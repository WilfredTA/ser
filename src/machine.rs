use std::collections::HashMap;
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};
use uuid::Uuid;

use z3_ext::{
    ast::{Ast, Bool, Int, BV},
    AstKind, Config, Context, Model, SatResult, Solver,
};

use crate::instruction::*;
use crate::memory::*;
use crate::state::evm::EvmState;
use crate::state::tree::{NodeId, StateTree};
use crate::traits::{Machine, MachineComponent, MachineInstruction, MachineState};
use crate::{
    bvc, bvi,
    record::*,
    smt::{ctx, BitVec},
    stack::Stack,
};

pub struct ExecutionSummary {
    reachable: Vec<EvmState>,
}

impl Default for ExecutionSummary {
    fn default() -> Self {
        Self::new()
    }
}
impl ExecutionSummary {
    pub fn new() -> Self {
        Self { reachable: vec![] }
    }

    pub fn with_state(state: EvmState) -> Self {
        Self {
            reachable: vec![state],
        }
    }

    pub fn with_states(states: Vec<EvmState>) -> Self {
        Self { reachable: states }
    }

    pub fn falsify(&self, assertion: Bool) -> bool {
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
    pub inverse_state: HashMap<Uuid, Uuid>,
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
            states: StateTree {
                id: NodeId::new(),
                val: evm_state,
                path_condition: None,
                left: None,
                right: None,
            },
            change_log: vec![],
            inverse_state: Default::default(),
        }
    }

    fn exec_check(&mut self) -> Vec<(ExecBranch, Option<Model<'ctx>>)> {
        let evm_trace = self.exec();
        let mut solver = z3_ext::Solver::new(ctx());
        evm_trace
            .into_iter()
            .filter_map(|(state, constraints)| {
                let constraint = constraints
                    .clone()
                    .into_iter()
                    .reduce(|c, e| Bool::and(ctx(), &[&c, &e]))
                    .unwrap();
                solver.assert(&constraint);
                match solver.check() {
                    SatResult::Sat => {
                        let model = solver.get_model();
                        eprintln!(
                            "State {:#?} is reachable.\nMODEL:{:#?}",
                            state,
                            solver.get_model()
                        );
                        Some(((state, constraints), model))
                    }
                    SatResult::Unsat => {
                        eprintln!("Unsat");
                        None
                    }
                    SatResult::Unknown => {
                        eprintln!("Unknown");
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
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

    fn exec(&mut self) -> Vec<ExecBranch<'ctx>> {
        let mut curr_state = self.states.val.clone();
        let curr_id = self.states.id.clone();

        let mut jump_ctx = vec![curr_id.id()];
        let mut state_tree = self.states.clone();
        let mut trace: Vec<ExecBranch> = vec![(curr_state, vec![])];
        let mut leaves: Vec<ExecBranch> = vec![];

        loop {
            let curr_state = trace.pop();
            let mut temp_id_ptr = jump_ctx.pop();
            //eprintln!("Executing node with id {:?}", temp_id_ptr);
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

                    let branch_right_id = state_tree.insert_right_of(branch.clone(), temp_id_ptr);

                    jump_ctx.push(branch_right_id);
                    if branch_state.can_continue() {
                        trace.push(branch);
                    } else {
                        leaves.push(branch);
                    }
                }
                let (nxt_state, nxt_constraints) = next_state;
                curr_cond.extend(nxt_constraints);
                let branch = (nxt_state.clone(), curr_cond.clone());
                // eprintln!("Inserting to the left of {:?}", temp_id_ptr);

                let branch_left_id = state_tree.insert_left_of(branch.clone(), temp_id_ptr);
                // eprintln!("Inserted {} to the left of {}", branch_left_id, temp_id_ptr);
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
        self.pgm.clone()
    }

    fn state(&self) -> Self::State {
        self.states
            .clone()
            .into_iter()
            .last()
            .unwrap_or((self.states.val.clone(), None))
            .0
    }

    fn state_ref(&self) -> &Self::State {
        &self.states.val
    }

    fn state_ref_mut(&mut self) -> &mut Self::State {
        &mut self.states.val
    }
}

impl MachineState<32> for EvmState {
    type PC = usize;

    fn pc(&self) -> Self::PC {
        self.pc
    }

    fn stack(&self) -> &Stack<32> {
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
        self.memory.write_word(idx, val);
    }

    fn mem_read(&self, idx: Index) -> BitVec<32> {
        self.memory.read_word(idx)
    }

    fn stack_apply(&mut self, stack_rec: StackChange<32>) {
        self.stack.apply_change(stack_rec);
    }

    fn mem_apply(&mut self, mem_rec: MemChange) {
        self.memory.apply_change(mem_rec);
    }
}

pub struct EvmExecutor<'ctx> {
    pub inner: Evm<'ctx>,
}

#[test]
fn machine_returns_one_exec_for_non_branching_pgm() {
    let one = bvi(1);
    let two = bvi(2);
    let a = bvc("a");

    // Two states (one reachable & one unreachable):
    // 1. Stack is [2 100 50]  <-- Reachable
    // 2.Stack is [2 50]       <-- Unreachable
    let pgm = vec![
        push32(two),
        push32(one),
        push32(a),
        add(),
        push32(bvi(7)),
        Instruction::JumpI,
        Instruction::Push(bvi(100)),
        push32(bvi(50)),
    ];

    let mut evm = Evm::new(pgm);

    let sat_branches = evm.exec_check();
    // assert!(
    //     sat_branches.first().is_some()
    //         && sat_branches
    //             .first()
    //             .unwrap()
    //             .0
    //              .0
    //             .stack()
    //             .peek_nth(1)
    //             .cloned()
    //             .unwrap()
    //             == bvi(100)
    // );
    //
    // assert_eq!(sat_branches.len(), 1);
}
