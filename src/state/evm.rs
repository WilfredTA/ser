use crate::machine::ExecBranch;
use crate::parser::Program;
use crate::state::tree::NodeId;
use crate::storage::{AccountStorage, Address};
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
    pub storage: AccountStorage,
    pub stack: Stack<32>,
    pc: usize,
    pub pgm: Program,
    pub address: Address,
    pub halt: bool,
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
            storage,
        } = rec;
        if let Some(mem) = mem {
            self.memory.apply_change(mem);
        }
        if let Some(stack) = stack {
            self.stack.apply_change(stack);
        }
        self.halt = halt;
        self.set_pc(pc.1);
    }
}

impl<'ctx> EvmState {
    pub fn with_pgm(pgm: Program) -> Self {
        Self {
            pgm,
            ..Default::default()
        }
    }

    pub fn pgm_counter(&self) -> usize {
        self.pc
    }
    pub fn set_pc(&mut self, new_pc: usize) {
        self.pc = new_pc;
        if self.pc >= self.pgm.pgm.len() {
            self.halt = true;
        }
    }

    pub fn inc_pc(&mut self) {
        let curr_inst = self.curr_instruction();
        

        self.set_pc(self.pc + curr_inst.byte_size());
    }
    pub fn can_continue(&self) -> bool {
        self.pc < self.pgm.get_size() && !self.halt
    }
    pub fn curr_instruction(&self) -> Instruction {
        if !self.can_continue() {
            eprintln!(
                "EVM STATE CANNOT CONTINUE; BUT CURR INST IS REQUESTED: {:#?}",
                self
            );
            eprintln!(
                "Getting curr inst.. curr pc: {} and curr pgm len: {}",
                self.pc,
                self.pgm.get_size()
            );
        }
        
        self.pgm.get(self.pc).expect(&format!("Expected instruction at pc: {}", self.pc))
    }
}

impl std::fmt::Display for EvmState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pc: {}\nHalted: {}\nStack: {:?}\n{}", self.pc(), self.halt, self.stack(), self.mem())
        
    }
}
