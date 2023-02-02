#![allow(unused)]
// #![feature(adt_const_params)]
extern crate z3 as z3_ext;
pub mod instruction;
pub mod machine;
pub mod memory;
pub mod record;
pub mod smt;
pub mod stack;
pub mod state;
pub mod traits;
use instruction::*;
use smt::*;
use stack::*;
use z3_ext::{
    ast::{Ast, Bool, Int, BV},
    AstKind, Config, Context, Model, SatResult, Solver,
};

pub fn bvi<const SZ: u32>(val: impl Into<i32>) -> BitVec<SZ> {
    BitVec::new_literal(val.into() as u64)
}
pub fn bvc(val: impl AsRef<str>) -> BitVec<32> {
    BitVec::new_const(val)
}
