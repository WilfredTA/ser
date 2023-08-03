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


#[derive(Clone, Default)]
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
        if self.pc >= self.pgm.get_size() {
            eprintln!("SET PC--- PC: {} -- PGM SIZE: {}", self.pc, self.pgm.get_size());
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
        self.pgm.get(self.pc).expect(&format!("Expected instruction at pc: {}", self.pc))
    }
    pub fn curr_inst_debug(&self) -> Instruction {
        if !self.can_continue() {

            eprintln!("Curr instruction debug requested but cannot continue");
        }
        self.pgm.get(self.pc).unwrap()
    }
}

impl std::fmt::Display for EvmState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "Pc: {}\nHalted: {}\nStack: {:?}\n{}", self.pc(), self.halt, self.stack(), self.mem())
        write!(f, "Pc: {}, Stack: {:#?} halt: {:#?}", self.pc(), self.stack(), self.halt)
    }
}

impl std::fmt::Debug for EvmState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EvmState").field("stack", &self.stack)
        .field("pc", &self.pc)
        .field("address", &self.address)
        .field("halt", &self.halt)
        .field("instruction", &self.curr_inst_debug())
        .field("pgm size", &self.pgm.size)
        .finish()
    }
}


