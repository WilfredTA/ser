use std::collections::HashMap;

use ruint::aliases::U256;
use z3_ext::ast::{BV, Ast, Bool};

use crate::{stack::Stack, record::{Index, MachineRecord, StackOp, StackChange}, memory::Memory, machine::{Machine, MachineState, EvmState}};

use super::smt::*;

#[derive(Clone, Debug)]
pub enum Instruction {
    Stop,
    Add,
    Mul,
    Sub,
    Div,
    SDiv,
    SMod,
    Mod,
    AddMod,
    MulMod,
    Exp,
    Lt,
    Gt,
    Slt,
    Sgt,
    Eq,
    And,
    Or,
    Xor,
    Not,
    Byte,
    Shl,
    Shr,
    Sha3,
    Address,
    Balance,
    Origin,
    Caller,
    CallValue,
    CallDataLoad,
    CallDataSize,
    CallDataCopy,
    CodeSize,
    CodeCopy,
    GasPrice,
    ExtCodeSize,
    ExtCodeCopy,
    ReturnDataSize,
    ReturnDataCopy,
    ExtCodeHash,
    BlockHash,
    Coinbase,
    Timestamp,
    Number,
    Difficulty,
    GasLimit,
    ChainId,
    SelfBalance,
    BaseFee,
    Pop,
    MLoad,
    MStore,
    MStore8,
    SLoad,
    SStore,
    Jump,
    JumpI,
    Pc,
    MSize,
    Gas,
    JumpDest,
    Push1(BitVec<1>),
    Push2(BitVec<2>),
    Push3(BitVec<3>),
    Push4(BitVec<4>),
    Push5(BitVec<5>),
    Push6(BitVec<6>),
    Push7(BitVec<7>),
    Push8(BitVec<8>),
    Push9(BitVec<9>),
    Push10(BitVec<10>),
    Push11(BitVec<11>),
    Push12(BitVec<12>),
    Push13(BitVec<13>),
    Push14(BitVec<14>),
    Push15(BitVec<15>),
    Push16(BitVec<16>),
    Push17(BitVec<17>),
    Push18(BitVec<18>),
    Push19(BitVec<19>),
    Push20(BitVec<20>),
    Push21(BitVec<21>),
    Push22(BitVec<22>),
    Push23(BitVec<23>),
    Push24(BitVec<24>),
    Push25(BitVec<25>),
    Push26(BitVec<26>),
    Push27(BitVec<27>),
    Push28(BitVec<28>),
    Push29(BitVec<29>),
    Push30(BitVec<30>),
    Push31(BitVec<31>),
    Push32(BitVec<32>),

    Dup1,
    Dup2,
    Dup3,
    Dup4,
    Dup5,
    Dup6,
    Dup7,
    Dup8,
    Dup9,
    Dup10,
    Dup11,
    Dup12,
    Dup13,
    Dup14,
    Dup15,
    Dup16,
    Swap1,
    Swap2,
    Swap3,
    Swap4,
    Swap5,
    Swap6,
    Swap7,
    Swap8,
    Swap9,
    Swap10,
    Swap11,
    Swap12,
    Swap13,
    Swap14,
    Swap15,
    Swap16,
    Log0,
    Log1,
    Log2,
    Log3,
    Log4,
    Create,
    Call,
    CallCode,
    Return,
    DelegateCall,
    Create2,
    StaticCall,
    Revert,
    Invalid,
    SelfDestruct,
    Push(BitVec<32>),
    IsZero,
    // Assert(BitVec<32>),
}




pub trait MachineInstruction {
    fn exec(&self, mach: impl AsRef<dyn Machine<Record = MachineRecord, State = EvmState>>) -> MachineRecord;
}


impl<'ctx> MachineInstruction for Instruction {
    fn exec(&self, mach: impl AsRef<dyn Machine<Record = MachineRecord, State = EvmState>>) -> MachineRecord {
        let mach = mach.as_ref();
        let mach = mach.state();
        match self {
            Instruction::Stop => todo!(),
            Instruction::Add => {
                let stack = mach.stack();
                let stack_top = stack.peek().unwrap();
                let stack_top2 = stack.peek_nth(1).unwrap();

                let stack_op_1 = StackOp::Pop;
                let stack_op_2 = StackOp::Pop;
                
                let stack_op_3 = StackOp::Push(stack_top.as_ref().bvadd(stack_top2.as_ref()).into());
                let pc = mach.pc();
                let stack_change = StackChange {
                    pop_qty: 2,
                    push_qty: 1,
                    ops: vec![stack_op_1, stack_op_2, stack_op_3],
                };
                MachineRecord {
                    stack: stack_change,
                    mem: Default::default(),
                    pc: (pc, pc + 1),
                    constraints: None
                }
            },
            Instruction::Mul => todo!(),
            Instruction::Sub => todo!(),
            Instruction::Div => todo!(),
            Instruction::SDiv => todo!(),
            Instruction::SMod => todo!(),
            Instruction::Mod => todo!(),
            Instruction::AddMod => todo!(),
            Instruction::MulMod => todo!(),
            Instruction::Exp => todo!(),
            Instruction::Lt => todo!(),
            Instruction::Gt => todo!(),
            Instruction::Slt => todo!(),
            Instruction::Sgt => todo!(),
            Instruction::Eq => todo!(),
            Instruction::And => todo!(),
            Instruction::Or => todo!(),
            Instruction::Xor => todo!(),
            Instruction::Not => todo!(),
            Instruction::Byte => todo!(),
            Instruction::Shl => todo!(),
            Instruction::Shr => todo!(),
            Instruction::Sha3 => todo!(),
            Instruction::Address => todo!(),
            Instruction::Balance => todo!(),
            Instruction::Origin => todo!(),
            Instruction::Caller => todo!(),
            Instruction::CallValue => todo!(),
            Instruction::CallDataLoad => todo!(),
            Instruction::CallDataSize => todo!(),
            Instruction::CallDataCopy => todo!(),
            Instruction::CodeSize => todo!(),
            Instruction::CodeCopy => todo!(),
            Instruction::GasPrice => todo!(),
            Instruction::ExtCodeSize => todo!(),
            Instruction::ExtCodeCopy => todo!(),
            Instruction::ReturnDataSize => todo!(),
            Instruction::ReturnDataCopy => todo!(),
            Instruction::ExtCodeHash => todo!(),
            Instruction::BlockHash => todo!(),
            Instruction::Coinbase => todo!(),
            Instruction::Timestamp => todo!(),
            Instruction::Number => todo!(),
            Instruction::Difficulty => todo!(),
            Instruction::GasLimit => todo!(),
            Instruction::ChainId => todo!(),
            Instruction::SelfBalance => todo!(),
            Instruction::BaseFee => todo!(),
            Instruction::Pop => {
                let pc = mach.pc();
                let stack_rec = StackChange {
                    pop_qty: 1,
                    push_qty: 0,
                    ops: vec![StackOp::Pop],
                };
                MachineRecord {
                    stack: stack_rec,
                    pc: (pc, pc + 1),
                    mem: Default::default(),
                    constraints: None
                }
            },
            Instruction::MLoad => todo!(),
            Instruction::MStore => todo!(),
            Instruction::MStore8 => todo!(),
            Instruction::SLoad => todo!(),
            Instruction::SStore => todo!(),
            Instruction::Jump => todo!(),
            Instruction::JumpI => {
                let jump_dest = mach.stack().peek().unwrap();
                let cond = mach.stack().peek_nth(1).unwrap();
                eprintln!("JUMP DEST: {:?}", jump_dest);
                let jump_dest_concrete = jump_dest.as_ref().simplify().as_u64().unwrap() as usize;
                eprintln!("JUMP DEST CONC: {:?}", jump_dest_concrete);
                
        
                let bv_zero = BV::from_u64(ctx(), 0, 256);
                let cond = cond.as_ref()._eq(&bv_zero);
                let cond = Bool::not(&cond);
        
                let stack_rec = StackChange {
                    pop_qty: 2,
                    push_qty: 0,
                    ops: vec![StackOp::Pop, StackOp::Pop]
                };

                MachineRecord {
                    stack: stack_rec,
                    pc: (mach.pc(), jump_dest_concrete),
                    constraints: Some(cond),
                    mem: Default::default()
                }
            },
            Instruction::Pc => todo!(),
            Instruction::MSize => todo!(),
            Instruction::Gas => todo!(),
            Instruction::JumpDest => todo!(),
            Instruction::Push1(bv) => todo!(),
            Instruction::Push2(bv) => todo!(),
            Instruction::Push3(bv) => todo!(),
            Instruction::Push4(bv) => todo!(),
            Instruction::Push5(bv) => todo!(),
            Instruction::Push6(bv) => todo!(),
            Instruction::Push7(bv) => todo!(),
            Instruction::Push8(bv) => todo!(),
            Instruction::Push9(bv) => todo!(),
            Instruction::Push10(bv) => todo!(),
            Instruction::Push11(bv) => todo!(),
            Instruction::Push12(bv) => todo!(),
            Instruction::Push13(bv) => todo!(),
            Instruction::Push14(bv) => todo!(),
            Instruction::Push15(bv) => todo!(),
            Instruction::Push16(bv) => todo!(),
            Instruction::Push17(bv) => todo!(),
            Instruction::Push18(bv) => todo!(),
            Instruction::Push19(bv) => todo!(),
            Instruction::Push20(bv) => todo!(),
            Instruction::Push21(bv) => todo!(),
            Instruction::Push22(bv) => todo!(),
            Instruction::Push23(bv) => todo!(),
            Instruction::Push24(bv) => todo!(),
            Instruction::Push25(bv) => todo!(),
            Instruction::Push26(bv) => todo!(),
            Instruction::Push27(bv) => todo!(),
            Instruction::Push28(bv) => todo!(),
            Instruction::Push29(bv) => todo!(),
            Instruction::Push30(bv) => todo!(),
            Instruction::Push31(bv) => todo!(),
            Instruction::Push32(bv) => todo!(),
            Instruction::Dup1 => todo!(),
            Instruction::Dup2 => todo!(),
            Instruction::Dup3 => todo!(),
            Instruction::Dup4 => todo!(),
            Instruction::Dup5 => todo!(),
            Instruction::Dup6 => todo!(),
            Instruction::Dup7 => todo!(),
            Instruction::Dup8 => todo!(),
            Instruction::Dup9 => todo!(),
            Instruction::Dup10 => todo!(),
            Instruction::Dup11 => todo!(),
            Instruction::Dup12 => todo!(),
            Instruction::Dup13 => todo!(),
            Instruction::Dup14 => todo!(),
            Instruction::Dup15 => todo!(),
            Instruction::Dup16 => todo!(),
            Instruction::Swap1 => todo!(),
            Instruction::Swap2 => todo!(),
            Instruction::Swap3 => todo!(),
            Instruction::Swap4 => todo!(),
            Instruction::Swap5 => todo!(),
            Instruction::Swap6 => todo!(),
            Instruction::Swap7 => todo!(),
            Instruction::Swap8 => todo!(),
            Instruction::Swap9 => todo!(),
            Instruction::Swap10 => todo!(),
            Instruction::Swap11 => todo!(),
            Instruction::Swap12 => todo!(),
            Instruction::Swap13 => todo!(),
            Instruction::Swap14 => todo!(),
            Instruction::Swap15 => todo!(),
            Instruction::Swap16 => todo!(),
            Instruction::Log0 => todo!(),
            Instruction::Log1 => todo!(),
            Instruction::Log2 => todo!(),
            Instruction::Log3 => todo!(),
            Instruction::Log4 => todo!(),
            Instruction::Create => todo!(),
            Instruction::Call => todo!(),
            Instruction::CallCode => todo!(),
            Instruction::Return => todo!(),
            Instruction::DelegateCall => todo!(),
            Instruction::Create2 => todo!(),
            Instruction::StaticCall => todo!(),
            Instruction::Revert => todo!(),
            Instruction::Invalid => todo!(),
            Instruction::SelfDestruct => todo!(),
            Instruction::Push(bv) => {
                let stack_change = StackChange {
                    pop_qty: 0,
                    push_qty: 1,
                    ops: vec![StackOp::Push(bv.clone())],
                };
                let pc = mach.pc();
                MachineRecord {
                    stack: stack_change,
                    mem: Default::default(),
                    pc: (pc, pc + 1),
                    constraints: None
                }
            },
            Instruction::IsZero => todo!(),
        }
    }
}