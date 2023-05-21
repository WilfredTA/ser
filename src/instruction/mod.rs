use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, BitXor};

use ruint::aliases::U256;
use z3_ext::ast::{Ast, Bool, BV};

use crate::record::{push, MemChange, MemOp};
use crate::state::env::*;
use crate::state::evm::EvmState;
use crate::traits::*;
use crate::{
    random_bv_arg,
    bvi,
    machine::Evm,
    memory::Memory,
    record::{Index, MachineRecord, StackChange, StackOp},
    stack::Stack,
};

use super::smt::*;



#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Stop,
    Add,
    Mul,
    Sub,
    Div,
    SDiv,
    Mod,
    SMod,
    AddMod,
    MulMod,
    Exp,
    SignExtend,
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

fn exec_dup_nth(mach: &EvmState, n: usize) -> MachineRecord<32> {
    let item = mach.stack().peek_nth(n - 1).unwrap();
    let ops = vec![push(item.clone())];

    MachineRecord {
        stack: Some(StackChange::with_ops(ops)),
        pc: (mach.pc(), mach.pc() + 1),
        mem: Default::default(),
        halt: false,
                    storage: None,
        constraints: None,
    }
}

impl<'ctx> MachineInstruction<'ctx, 32> for Instruction {
    fn exec(&self, mach: &EvmState) -> MachineRecord<32> {
        match self {
            Instruction::Stop => todo!(),
            Instruction::Add => {
                let stack = mach.stack();
                let [stack_top, stack_top2] = stack.peek_top().unwrap();

                let stack_op_1 = StackOp::Pop;
                let stack_op_2 = StackOp::Pop;

                let stack_op_3 =
                    StackOp::Push(stack_top.as_ref().bvadd(stack_top2.as_ref()).into());
                let pc = mach.pc();
                let stack_change = StackChange {
                    pop_qty: 2,
                    push_qty: 1,
                    ops: vec![stack_op_1, stack_op_2, stack_op_3],
                };
                MachineRecord {
                    stack: Some(stack_change),
                    mem: Default::default(),
                    pc: (pc, pc + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Mul => {
                let stack = mach.stack();
                let [mul1, mul2] = stack.peek_top().unwrap();
                let product: BitVec<32> = mul1.as_ref().bvmul(mul2.as_ref()).into();
                let ops = vec![pop(), pop(), push(product)];
                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Sub => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let difference: BitVec<32> = a.as_ref().bvsub(b.as_ref()).into();
                let ops = vec![pop(), pop(), push(difference)];
                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Div => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let quot: BitVec<32> = a.as_ref().bvudiv(b.as_ref()).into();
                let ops = vec![pop(), pop(), push(quot)];
                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::SDiv => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let quot: BitVec<32> = a.as_ref().bvsdiv(b.as_ref()).into();
                let ops = vec![pop(), pop(), push(quot)];
                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::SMod => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let rem: BitVec<32> = a.as_ref().bvsmod(b.as_ref()).into();
                let ops = vec![pop(), pop(), push(rem)];
                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Mod => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let rem: BitVec<32> = a.as_ref().bvurem(b.as_ref()).into();
                let ops = vec![pop(), pop(), push(rem)];
                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::AddMod => {
                let stack = mach.stack();
                let [a, b, n] = stack.peek_top().unwrap();
                let res: BitVec<32> = a.as_ref().bvadd(b.as_ref()).bvurem(n.as_ref()).into();
                let ops = vec![pop(), pop(), pop(), push(res)];
                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::MulMod => {
                let stack = mach.stack();
                let [a, b, n] = stack.peek_top().unwrap();
                let res: BitVec<32> = a.as_ref().bvmul(b.as_ref()).bvurem(n.as_ref()).into();
                let ops = vec![pop(), pop(), pop(), push(res)];
                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Exp => {
                let stack = mach.stack();
                let [a, power] = stack.peek_top().unwrap();
                let mut power_conc = power.as_ref().as_u64().unwrap();
                let mut exp: BitVec<32> = if power_conc == 0 {
                    bvi(1)
                } else if power_conc == 1 {
                    a.clone()
                } else {
                    let mut temp_exp = a.clone();
                    while (power_conc) > 1 {
                        temp_exp = temp_exp.as_ref().bvmul(a.as_ref()).into();
                        power_conc -= 1;
                    }
                    temp_exp
                };

                let ops = vec![pop(), pop(), push(exp)];
                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Lt => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let lt: BitVec<32> = a
                    .as_ref()
                    .bvult(b.as_ref())
                    .ite(bvi::<32>(1).as_ref(), bvi::<32>(0).as_ref())
                    .into();

                let ops = vec![pop(), pop(), push(lt)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Gt => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let lt: BitVec<32> = a
                    .as_ref()
                    .bvugt(b.as_ref())
                    .ite(bvi::<32>(1).as_ref(), bvi::<32>(0).as_ref())
                    .into();

                let ops = vec![pop(), pop(), push(lt)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Slt => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let lt: BitVec<32> = a
                    .as_ref()
                    .bvslt(b.as_ref())
                    .ite(bvi::<32>(1).as_ref(), bvi::<32>(0).as_ref())
                    .into();

                let ops = vec![pop(), pop(), push(lt)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Sgt => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let gt: BitVec<32> = a
                    .as_ref()
                    .bvsgt(b.as_ref())
                    .ite(bvi::<32>(1).as_ref(), bvi::<32>(0).as_ref())
                    .into();

                let ops = vec![pop(), pop(), push(gt)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Eq => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let eq: BitVec<32> = a
                    .as_ref()
                    ._eq(b.as_ref())
                    .ite(bvi::<32>(1).as_ref(), bvi::<32>(0).as_ref())
                    .into();

                let ops = vec![pop(), pop(), push(eq)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::And => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let and = a.as_ref().bitand(b.as_ref()).into();

                let ops = vec![pop(), pop(), push(and)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Or => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let or = a.as_ref().bitor(b.as_ref()).into();

                let ops = vec![pop(), pop(), push(or)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Xor => {
                let stack = mach.stack();
                let [a, b] = stack.peek_top().unwrap();
                let xor = a.as_ref().bitxor(b.as_ref()).into();

                let ops = vec![pop(), pop(), push(xor)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Not => {
                let stack = mach.stack();
                let a = stack.peek().unwrap();

                let neg = a.as_ref().bvneg().into();

                let ops = vec![pop(), pop(), push(neg)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Byte => todo!(),
            Instruction::Shl => {
                let stack = mach.stack();
                let [shift, value] = stack.peek_top().unwrap();

                let shl = value.as_ref().bvshl(shift.as_ref()).into();

                let ops = vec![pop(), pop(), push(shl)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Shr => {
                let stack = mach.stack();
                let [shift, value] = stack.peek_top().unwrap();

                let shr = value.as_ref().bvlshr(shift.as_ref()).into();

                let ops = vec![pop(), pop(), push(shr)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Sha3 => todo!(),
            Instruction::Address => todo!(),
            Instruction::Balance => {
                let stack = mach.stack();
                let addr = stack.peek().unwrap();
                let bal = balance()
                    .apply(&[addr.as_ref(), random_bv_arg::<32>().as_ref()])
                    .as_bv()
                    .unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(bal.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Origin => {
                let stack = mach.stack();
                let orig = origin().apply(&[]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(orig.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Caller => {
                let stack = mach.stack();
                let caller = caller().apply(&[]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(caller.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::CallValue => {
                let stack = mach.stack();
                let call_val = call_value().apply(&[]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(call_val.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::CallDataLoad => {
                let stack = mach.stack();
                let offset = stack.peek().unwrap();
                let call_data = call_data_load().apply(&[offset.as_ref()]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(call_data.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::CallDataSize => {
                let stack = mach.stack();
                let call_data_sz = call_data_size().apply(&[]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(call_data_sz.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::CallDataCopy => todo!(),
            Instruction::CodeSize => todo!(),
            Instruction::CodeCopy => todo!(),
            Instruction::GasPrice => {
                let stack = mach.stack();
                let price = gas_price().apply(&[]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(price.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::ExtCodeSize => {
                let stack = mach.stack();
                let addr = stack.peek().unwrap();
                let ext_code_sz = call_value().apply(&[addr.as_ref()]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(ext_code_sz.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::ExtCodeCopy => todo!(),
            Instruction::ReturnDataSize => todo!(),
            Instruction::ReturnDataCopy => todo!(),
            Instruction::ExtCodeHash => todo!(),
            Instruction::BlockHash => {
                let stack = mach.stack();
                let blk_hash = block_hash().apply(&[]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(blk_hash.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Coinbase => {
                let stack = mach.stack();
                let coin_base = coinbase().apply(&[]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(coin_base.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Timestamp => {
                let stack = mach.stack();
                let timestmp = timestamp().apply(&[]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(timestmp.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Number => todo!(),
            Instruction::Difficulty => {
                let stack = mach.stack();
                let difficulty = difficulty().apply(&[]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(difficulty.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::GasLimit => {
                let stack = mach.stack();
                let gas_limit = gas_lim().apply(&[]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(gas_limit.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::ChainId => {
                let stack = mach.stack();
                let cid = chain_id().apply(&[]).as_bv().unwrap();
                let stack_diff = StackChange::with_ops(vec![pop(), push(cid.into())]);

                MachineRecord {
                    stack: Some(stack_diff),
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
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
                    stack: Some(stack_rec),
                    pc: (pc, pc + 1),
                    mem: Default::default(),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::MLoad => {
                let stack = mach.stack();
                let dest = stack.peek().unwrap();
                let mut val_mem = mach.memory.read_word(dest.clone());
                val_mem.simplify();

                let mem_change = MemChange {
                    ops_log: vec![MemOp::Read { idx: dest.clone() }],
                };

                MachineRecord {
                    stack: Some(StackChange::with_ops(vec![pop(), push(val_mem)])),
                    mem: Some(mem_change),
                    pc: (mach.pc(), mach.pc() + 1),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::MStore => {
                let stack = mach.stack();

                let dest = stack.peek().unwrap();

                let val = stack.peek_nth(1).unwrap();

                let mem_change = MemChange {
                    ops_log: vec![MemOp::Write {
                        idx: dest.clone(),
                        val: val.clone(),
                    }],
                };

                let stack_change = StackChange::with_ops(vec![pop(), pop()]);

                MachineRecord {
                    mem: Some(mem_change),
                    stack: Some(stack_change),
                    constraints: None,
                    halt: false,
                    storage: None,
                    pc: (mach.pc(), mach.pc() + 1),
                }
            }
            Instruction::MStore8 => {
                let stack = mach.stack();

                let dest = stack.peek().unwrap();

                let val = stack.peek_nth(1).unwrap();

                //let val_inner = val.as_ref().extract(31 * 8 + 7, 31 * 8);
                let val_inner = val.as_ref().extract(7, 0);

                let val: BitVec<1> = BitVec::with_bv(val_inner.simplify());

                let mem_change = MemChange {
                    ops_log: vec![MemOp::WriteByte {
                        idx: dest.clone(),
                        val,
                    }],
                };

                let stack_change = StackChange::with_ops(vec![pop(), pop()]);

                MachineRecord {
                    mem: Some(mem_change),
                    stack: Some(stack_change),
                    constraints: None,
                    halt: false,
                    storage: None,
                    pc: (mach.pc(), mach.pc() + 1),
                }
            }
            Instruction::SLoad => todo!(),
            Instruction::SStore => todo!(),
            Instruction::Jump => todo!(),
            Instruction::JumpI => {
                let jump_dest = mach.stack().peek().unwrap();
                let cond = mach.stack().peek_nth(1).unwrap();
                let jump_dest_concrete = jump_dest.as_ref().simplify().as_u64().unwrap() as usize;

                let bv_zero = BV::from_u64(ctx(), 0, 256);
                let cond = cond.as_ref()._eq(&bv_zero);
                let cond = Bool::not(&cond);

                let stack_rec = StackChange {
                    pop_qty: 2,
                    push_qty: 0,
                    ops: vec![StackOp::Pop, StackOp::Pop],
                };

                MachineRecord {
                    stack: Some(stack_rec),
                    pc: (mach.pc(), jump_dest_concrete),
                    constraints: Some(cond),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                }
            }
            Instruction::Pc => {
                let pc = BitVec::new_literal(mach.pc() as u64);
                let stack_rec = StackChange {
                    pop_qty: 0,
                    push_qty: 1,
                    ops: vec![StackOp::Push(pc)],
                };
                MachineRecord {
                    stack: Some(stack_rec),
                    pc: (mach.pc(), mach.pc() + 1),
                    constraints: None,
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                }
            }
            Instruction::MSize => {
                let mem = mach.mem();
                let size = mem.m_size();
                let ops = vec![push(bvi(size as i32))];

                let stack = Some(StackChange::with_ops(ops));

                MachineRecord {
                    stack,
                    mem: Default::default(),
                    pc: (mach.pc(), mach.pc() + 1),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Gas => todo!(),
            Instruction::JumpDest => todo!(),
            Instruction::Push1(bv) => {
                let new_bv = bv.as_ref().zero_ext(31).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push2(bv) => {
                let new_bv = bv.as_ref().zero_ext(30).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push3(bv) => {
                let new_bv = bv.as_ref().zero_ext(29).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push4(bv) => {
                let new_bv = bv.as_ref().zero_ext(28).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push5(bv) => {
                let new_bv = bv.as_ref().zero_ext(27).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push6(bv) => {
                let new_bv = bv.as_ref().zero_ext(26).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push7(bv) => {
                let new_bv = bv.as_ref().zero_ext(25).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push8(bv) => {
                let new_bv = bv.as_ref().zero_ext(24).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push9(bv) => {
                let new_bv = bv.as_ref().zero_ext(23).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push10(bv) => {
                let new_bv = bv.as_ref().zero_ext(22).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push11(bv) => {
                let new_bv = bv.as_ref().zero_ext(21).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push12(bv) => {
                let new_bv = bv.as_ref().zero_ext(20).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push13(bv) => {
                let new_bv = bv.as_ref().zero_ext(19).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push14(bv) => {
                let new_bv = bv.as_ref().zero_ext(18).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push15(bv) => {
                let new_bv = bv.as_ref().zero_ext(17).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push16(bv) => {
                let new_bv = bv.as_ref().zero_ext(16).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push17(bv) => {
                let new_bv = bv.as_ref().zero_ext(15).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push18(bv) => {
                let new_bv = bv.as_ref().zero_ext(14).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push19(bv) => {
                let new_bv = bv.as_ref().zero_ext(13).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push20(bv) => {
                let new_bv = bv.as_ref().zero_ext(12).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push21(bv) => {
                let new_bv = bv.as_ref().zero_ext(11).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push22(bv) => {
                let new_bv = bv.as_ref().zero_ext(10).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push23(bv) => {
                let new_bv = bv.as_ref().zero_ext(9).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push24(bv) => {
                let new_bv = bv.as_ref().zero_ext(8).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push25(bv) => {
                let new_bv = bv.as_ref().zero_ext(7).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push26(bv) => {
                let new_bv = bv.as_ref().zero_ext(6).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push27(bv) => {
                let new_bv = bv.as_ref().zero_ext(5).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push28(bv) => {
                let new_bv = bv.as_ref().zero_ext(4).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push29(bv) => {
                let new_bv = bv.as_ref().zero_ext(3).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push30(bv) => {
                let new_bv = bv.as_ref().zero_ext(2).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push31(bv) => {
                let new_bv = bv.as_ref().zero_ext(1).into();

                let ops = vec![push(new_bv)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Push32(bv) => {
                let ops = vec![push(bv.clone())];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
            Instruction::Dup1 => exec_dup_nth(mach, 1),
            Instruction::Dup2 => exec_dup_nth(mach, 2),
            Instruction::Dup3 => exec_dup_nth(mach, 3),
            Instruction::Dup4 => exec_dup_nth(mach, 4),
            Instruction::Dup5 => exec_dup_nth(mach, 5),
            Instruction::Dup6 => exec_dup_nth(mach, 6),
            Instruction::Dup7 => exec_dup_nth(mach, 7),
            Instruction::Dup8 => exec_dup_nth(mach, 8),
            Instruction::Dup9 => exec_dup_nth(mach, 9),
            Instruction::Dup10 => exec_dup_nth(mach, 10),
            Instruction::Dup11 => exec_dup_nth(mach, 11),
            Instruction::Dup12 => exec_dup_nth(mach, 12),
            Instruction::Dup13 => exec_dup_nth(mach, 13),
            Instruction::Dup14 => exec_dup_nth(mach, 14),
            Instruction::Dup15 => exec_dup_nth(mach, 15),
            Instruction::Dup16 => exec_dup_nth(mach, 16),
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
            Instruction::SignExtend => todo!(),
            Instruction::Push(bv) => {
                let stack_change = StackChange {
                    pop_qty: 0,
                    push_qty: 1,
                    ops: vec![StackOp::Push(bv.clone())],
                };
                let pc = mach.pc();
                MachineRecord {
                    stack: Some(stack_change),
                    mem: Default::default(),
                    pc: (pc, pc + 1),
                    constraints: None,
                    halt: false,
                    storage: None,
                }
            }
            Instruction::IsZero => {
                let top = mach.stack().peek().unwrap();
                let is_zero: BitVec<32> = top
                    .as_ref()
                    ._eq(bvi::<32>(0).as_ref())
                    .ite(bvi::<32>(1).as_ref(), bvi::<32>(0).as_ref())
                    .into();

                let ops = vec![pop(), push(is_zero)];

                MachineRecord {
                    stack: Some(StackChange::with_ops(ops)),
                    pc: (mach.pc(), mach.pc() + 1),
                    mem: Default::default(),
                    halt: false,
                    storage: None,
                    constraints: None,
                }
            }
        }
    }
}

pub fn pop<const SZ: u32>() -> StackOp<SZ> {
    StackOp::Pop
}
pub fn add() -> Instruction {
    Instruction::Add
}

pub fn jumpi() -> Instruction {
    Instruction::JumpI
}

pub fn is_zero() -> Instruction {
    Instruction::IsZero
}
pub fn dup1() -> Instruction {
    Instruction::Dup1
}
pub fn dup2() -> Instruction {
    Instruction::Dup2
}
pub fn dup3() -> Instruction {
    Instruction::Dup3
}
pub fn dup4() -> Instruction {
    Instruction::Dup4
}
pub fn dup5() -> Instruction {
    Instruction::Dup5
}
pub fn dup6() -> Instruction {
    Instruction::Dup6
}
pub fn dup7() -> Instruction {
    Instruction::Dup7
}
pub fn dup8() -> Instruction {
    Instruction::Dup8
}
pub fn dup9() -> Instruction {
    Instruction::Dup9
}
pub fn dup10() -> Instruction {
    Instruction::Dup10
}
pub fn dup11() -> Instruction {
    Instruction::Dup11
}
pub fn dup12() -> Instruction {
    Instruction::Dup12
}
pub fn dup13() -> Instruction {
    Instruction::Dup13
}
pub fn dup14() -> Instruction {
    Instruction::Dup14
}
pub fn dup15() -> Instruction {
    Instruction::Dup15
}
pub fn dup16() -> Instruction {
    Instruction::Dup16
}
// pub fn push<const SZ: u32>(size: usize, val: BitVec<>) -> Instruction {
//     Instruction::Push5(BitVec::default())
// }

pub fn push1(v: BitVec<1>) -> Instruction {
    Instruction::Push1(v)
}
pub fn push2(v: BitVec<2>) -> Instruction {
    Instruction::Push2(v)
}
pub fn push3(v: BitVec<3>) -> Instruction {
    Instruction::Push3(v)
}
pub fn push4(v: BitVec<4>) -> Instruction {
    Instruction::Push4(v)
}
pub fn push5(v: BitVec<5>) -> Instruction {
    Instruction::Push5(v)
}
pub fn push6(v: BitVec<6>) -> Instruction {
    Instruction::Push6(v)
}
pub fn push7(v: BitVec<7>) -> Instruction {
    Instruction::Push7(v)
}
pub fn push8(v: BitVec<8>) -> Instruction {
    Instruction::Push8(v)
}
pub fn push9(v: BitVec<9>) -> Instruction {
    Instruction::Push9(v)
}
pub fn push10(v: BitVec<10>) -> Instruction {
    Instruction::Push10(v)
}
pub fn push11(v: BitVec<11>) -> Instruction {
    Instruction::Push11(v)
}
pub fn push12(v: BitVec<12>) -> Instruction {
    Instruction::Push12(v)
}
pub fn push13(v: BitVec<13>) -> Instruction {
    Instruction::Push13(v)
}
pub fn push14(v: BitVec<14>) -> Instruction {
    Instruction::Push14(v)
}
pub fn push15(v: BitVec<15>) -> Instruction {
    Instruction::Push15(v)
}
pub fn push16(v: BitVec<16>) -> Instruction {
    Instruction::Push16(v)
}
pub fn push17(v: BitVec<17>) -> Instruction {
    Instruction::Push17(v)
}
pub fn push18(v: BitVec<18>) -> Instruction {
    Instruction::Push18(v)
}
pub fn push19(v: BitVec<19>) -> Instruction {
    Instruction::Push19(v)
}
pub fn push20(v: BitVec<20>) -> Instruction {
    Instruction::Push20(v)
}
pub fn push21(v: BitVec<21>) -> Instruction {
    Instruction::Push21(v)
}
pub fn push22(v: BitVec<22>) -> Instruction {
    Instruction::Push22(v)
}
pub fn push23(v: BitVec<23>) -> Instruction {
    Instruction::Push23(v)
}
pub fn push24(v: BitVec<24>) -> Instruction {
    Instruction::Push24(v)
}
pub fn push25(v: BitVec<25>) -> Instruction {
    Instruction::Push25(v)
}
pub fn push26(v: BitVec<26>) -> Instruction {
    Instruction::Push26(v)
}
pub fn push27(v: BitVec<27>) -> Instruction {
    Instruction::Push27(v)
}
pub fn push28(v: BitVec<28>) -> Instruction {
    Instruction::Push28(v)
}
pub fn push29(v: BitVec<29>) -> Instruction {
    Instruction::Push29(v)
}
pub fn push30(v: BitVec<30>) -> Instruction {
    Instruction::Push30(v)
}
pub fn push31(v: BitVec<31>) -> Instruction {
    Instruction::Push31(v)
}
pub fn push32(v: BitVec<32>) -> Instruction {
    Instruction::Push32(v)
}
