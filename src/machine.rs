use std::collections::HashMap;
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};
use uuid::Uuid;

use z3_ext::{
    ast::{Ast, Bool, Int, BV},
    AstKind, Config, Context, Model, SatResult, Solver,
};

use crate::exec::Execution;
use crate::instruction::*;
use crate::memory::*;
use crate::parser::Program;
use crate::state::evm::EvmState;
use crate::state::tree::{NodeId, StateTree};
use crate::storage::{AccountStorage, Address};
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

pub struct TransactionContext {
    calldata: Option<Vec<u8>>,
    output: Option<Vec<u8>>,
    storage: HashMap<usize, [u8; 256]>,
}

// The Evm is an *implementation* of the Machine trait
// Its job is merely to orchestrate the incremental construction of an Execution
// by initializing an Execution with a starting StateTree and calling Execution.step()
// It also provides a handler to an Execution, which in turn provides a handler to EvmState
// so that when an Instruction requires *reading* environmental / network state OR when an Instruction's behavior
// *depends* on env / network state, it can access it through the Evm, which can be initialized with things like a Transaction context
#[derive(Clone)]
pub struct Evm<'ctx> {
    pgm: Program,
    pub states: StateTree<'ctx>,
    change_log: Vec<MachineRecord<32>>,
    pub inverse_state: HashMap<Uuid, Uuid>,
}

impl<'ctx> Evm<'ctx> {
    pub fn with_pgm(pgm: Program) -> Self {
        let evm_state = EvmState::with_pgm(pgm.clone());
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

    pub fn set_init_state(&mut self, state: EvmState) {
        self.states.val = state;
    }
}

impl<'ctx> Evm<'ctx> {
    pub fn new(pgm: Program) -> Self {
        let evm_state = EvmState::with_pgm(pgm.clone());

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

    // A path from a node is the current node and the union of the paths of its children
    pub fn paths(trace: Execution<'ctx>) -> Vec<Vec<(NodeId, Instruction, Option<Bool<'ctx>>)>> {
        let mut paths_collected = vec![];

        let mut curr_path = vec![];
        let tree = Some(Box::new(trace.states));
        StateTree::find_paths(&tree, &mut curr_path, &mut paths_collected);
        paths_collected
    }

    pub fn exec_check(
        trace: Execution<'ctx>,
    ) -> Vec<(
        Vec<(NodeId, Instruction, Option<Bool<'ctx>>)>,
        Option<SatResult>,
        Option<String>,
    )> {
        let mut solver = z3_ext::Solver::new(ctx());
        let paths = Self::paths(trace);
        let reachable = paths
            .into_iter()
            .map(|path| {
                let mut result = (path.clone(), None, None);
                solver.push();
                path.iter().for_each(|step| {
                    if let Some(constraint) = step.2.clone() {
                        solver.assert(&constraint);
                    }
                });
                match solver.check() {
                    SatResult::Sat => {
                        let model = solver.get_model();
                        result.1 = Some(SatResult::Sat);

                        result.2 = model.map(|m| m.to_string());
                    }
                    SatResult::Unsat => {
                        result.1 = Some(SatResult::Unsat);
                    }
                    SatResult::Unknown => {
                        result.1 = Some(SatResult::Unknown);
                    }
                }
                solver.pop(1);
                result
            })
            .collect::<Vec<_>>();
        reachable
    }

    // pub fn exec_check(&mut self) -> Vec<(ExecBranch, Option<Model<'ctx>>)> {
    //     let evm_trace = self.exec();
    //     let mut solver = z3_ext::Solver::new(ctx());
    //     evm_trace
    //         .into_iter()
    //         .filter_map(|(state, constraints)| {
    //             let constraint = constraints
    //                 .clone()
    //                 .into_iter()
    //                 .reduce(|c, e| Bool::and(ctx(), &[&c, &e]));
    //             if let Some(constraint) = constraint {
    //                 solver.assert(&constraint);
    //             }
    //             match solver.check() {
    //                 SatResult::Sat => {
    //                     let model = solver.get_model();
    //                     eprintln!(
    //                         "State {:#?} is reachable.\nMODEL:{:#?}",
    //                         state,
    //                         solver.get_model()
    //                     );
    //                     Some(((state, constraints), model))
    //                 }
    //                 SatResult::Unsat => {
    //                     eprintln!("Unsat");
    //                     None
    //                 }
    //                 SatResult::Unknown => {
    //                     eprintln!("Unknown");
    //                     None
    //                 }
    //             }
    //         })
    //         .collect::<Vec<_>>()
    // }
}

impl<'ctx> Machine<32> for Evm<'ctx> {
    type State = EvmState;

    fn exec(&mut self) -> Execution {
        let mut halt = false;
        let mut step_recs = vec![];
        let mut exec = Execution::new(self.states.val.clone(), self.pgm.clone());
        let first_step = exec.step_mut();
        step_recs.push(first_step);
        let mut ids = vec![];
        loop {
            if let Some(step) = step_recs.pop() {
                // eprintln!(
                //     "HALTED LEFT: {}, HALTED RIGHT: {}",
                //     step.halted_left(),
                //     step.halted_right()
                // );
                // eprintln!(
                //     "LEFT ID: {:#?} RIGHT ID: {:#?}",
                //     step.left_id(),
                //     step.right_id()
                // );
                // if !step.halted_right() {
                //     let continue_from_right = step.right_id();
                if let Some(right_id) = step.right_id() {
                    ids.push(right_id.id());
                    let nxt_right_step = exec.step_from_mut(right_id);
                    step_recs.push(nxt_right_step);
                }
                //}
                // if !step.halted_left() {
                //     let continue_from_left = step.left_id();
                if let Some(left_id) = step.left_id() {
                    ids.push(left_id.id());
                    let nxt_step = exec.step_from_mut(left_id);
                    step_recs.push(nxt_step);
                }
                // }

                if step.halted_left() && step.halted_right() {
                    eprintln!(
                        "Both have halted... Here are the step recs left: {:#?}",
                        step_recs
                    );
                }
            } else {
                break;
            }
        }
        eprintln!("All ids that were executed during a step: {:#?}", ids);

        exec
    }

    // fn exec(&mut self) -> Vec<ExecBranch<'ctx>> {
    //     let mut curr_state = self.states.val.clone();
    //     let curr_id = self.states.id.clone();

    //     let mut jump_ctx = vec![curr_id.id()];
    //     let mut state_tree = self.states.clone();
    //     let mut trace: Vec<ExecBranch> = vec![(curr_state, vec![])];
    //     let mut leaves: Vec<ExecBranch> = vec![];

    //     loop {
    //         let curr_state = trace.pop();
    //         let mut temp_id_ptr = jump_ctx.pop();
    //         //eprintln!("Executing node with id {:?}", temp_id_ptr);
    //         if let Some(curr_state) = curr_state {
    //             let temp_id_ptr = temp_id_ptr.unwrap();

    //             let (curr_state, curr_cond) = curr_state;
    //             let mut curr_cond = curr_cond.clone();
    //             let mut branch_cond_pre = curr_cond.clone();
    //             let (next_state, next_state_branch, halt) = curr_state.exec_once();
    //             if let Some(branch) = next_state_branch {
    //                 let (branch_state, branch_cond) = branch;
    //                 branch_cond_pre.extend(branch_cond);
    //                 let branch = (branch_state.clone(), branch_cond_pre.clone());

    //                 let branch_right_id = state_tree.insert_right_of(branch.clone(), temp_id_ptr);

    //                 jump_ctx.push(branch_right_id);
    //                 if branch_state.can_continue() {
    //                     trace.push(branch);
    //                 } else {
    //                     leaves.push(branch);
    //                 }
    //             }

    //             let (nxt_state, nxt_constraints) = next_state;
    //             eprintln!("NEXT STATE {:#?}", nxt_state);
    //             curr_cond.extend(nxt_constraints);
    //             let branch = (nxt_state.clone(), curr_cond.clone());
    //             // eprintln!("Inserting to the left of {:?}", temp_id_ptr);

    //             let branch_left_id = state_tree.insert_left_of(branch.clone(), temp_id_ptr);
    //             // eprintln!("Inserted {} to the left of {}", branch_left_id, temp_id_ptr);
    //             if nxt_state.can_continue() && !halt {
    //                 trace.push(branch);
    //                 jump_ctx.push(branch_left_id);
    //             } else {
    //                 leaves.push(branch);
    //             }
    //             if halt {
    //                 break;
    //             }
    //         } else {
    //             break;
    //         }
    //     }
    //     self.states = state_tree;
    //     leaves
    // }

    fn pgm(&self) -> Program {
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
        self.pgm_counter()
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

    fn storage(&self) -> &AccountStorage {
        &self.storage
    }

    fn storage_write(&mut self, idx: Index, val: crate::storage::StorageValue) {
        self.storage.sstore(idx, val);
    }

    fn storage_read(&self, idx: &Index) -> crate::storage::StorageValue {
        self.storage.sload(idx)
    }

    fn storage_apply(&mut self, storage_rec: StorageChange) {
        self.storage.apply_change(storage_rec);
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
        Instruction::Push32(bvi(100)),
        push32(bvi(50)),
    ];

    // let mut evm = Evm::new(pgm);
    // let execution = evm.exec();
    // let exec_tree = &execution.states;
    // let final_states = exec_tree.leaves();
    // assert_eq!(2, final_states.len());
    // assert_eq!(
    //     final_states
    //         .first()
    //         .unwrap()
    //         .val
    //         .stack()
    //         .peek_nth(1)
    //         .cloned()
    //         .unwrap(),
    //     bvi(100)
    // );
    // eprintln!("Final states: {:#?}", final_states);
}

#[test]
fn test_mem_store_mem_load() {
    let pgm = vec![
        push32(bvi(3)),
        push32(bvi(2)),
        Instruction::MStore,
        push32(bvi(2)),
        Instruction::MLoad,
        // push32(bvi(5)),
        // Instruction::MStore8,
        // push32(bvi(5)),
        // Instruction::MLoad,
    ];

    // let mut evm = Evm::new(pgm);

    // {
    //     let sat_branches = evm.exec_check();
    //     assert_eq!(sat_branches.len(), 1);
    //     let top = sat_branches
    //         .first()
    //         .unwrap()
    //         .0
    //          .0
    //         .stack()
    //         .peek()
    //         .cloned()
    //         .unwrap();
    //     eprintln!("Stack top size: {:#?}", top.as_ref().get_size());
    //     assert_eq!(top, bvi(3));
    // }
    //eprintln!("STATES > {:#?}", evm.states);
}
