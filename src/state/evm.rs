use crate::machine::ExecBranch;
use crate::state::tree::NodeId;
use crate::storage::Address;
use crate::traits::MachineState;
use crate::{
    instruction::Instruction,
    memory::Memory,
    record::*,
    stack::Stack,
    traits::{MachineComponent, MachineInstruction},
};
use z3_ext::ast::Bool;

#[derive(Clone, Debug, Default)]
pub struct EvmState {
    pub memory: Memory,
    pub stack: Stack<32>,
    pub pc: usize,
    pub pgm: Vec<Instruction>,
    pub address: Address
}

impl MachineComponent for EvmState {
    type Record = MachineRecord<32>;

    fn apply_change(&mut self, rec: Self::Record) {
        let MachineRecord {
            halt,
            pc,
            stack,
            mem,
            constraints,
            storage
        } = rec;
        if let Some(mem) = mem {
            self.memory.apply_change(mem);
        }
        if let Some(stack) = stack {
            self.stack.apply_change(stack);
        }

        self.pc = pc.1;
    }
}

impl<'ctx> EvmState {
    pub fn can_continue(&self) -> bool {
        self.pc < self.pgm.len()
    }
    pub fn curr_instruction(&self) -> Instruction {
        self.pgm.get(self.pc).cloned().unwrap()
    }

    pub fn exec_once(mut self) -> (ExecBranch<'ctx>, Option<ExecBranch<'ctx>>) {
        let inst = self.curr_instruction();
        let change = inst.exec(&self);

        self.state_transition(change)
    }
    // Generates a set of next possible EvmStates given the state change record
    pub fn state_transition(
        &self,
        rec: MachineRecord<32>,
    ) -> (ExecBranch<'ctx>, Option<ExecBranch<'ctx>>) {
        let MachineRecord {
            halt,
            pc,
            stack,
            mem,
            constraints,
            storage
        } = rec;
        let mut new_state = self.clone();
        if let Some(stack_rec) = stack {
            new_state.stack_apply(stack_rec);
        }

        if let Some(mem_rec) = mem {
            new_state.mem_apply(mem_rec);
        }

        if let Some(constraint) = constraints {
            let mut does_jump_state = new_state.clone();
            does_jump_state.pc = pc.1;
            new_state.pc += 1;
            (
                (new_state, vec![constraint.not()]),
                Some((does_jump_state, vec![constraint])),
            )
        } else {
            assert_eq!(pc.1, (pc.0 + 1));
            new_state.pc = pc.1;

            ((new_state, vec![]), None)
        }
    }
}
