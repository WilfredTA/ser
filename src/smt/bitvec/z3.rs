use crate::smt::{SolverType, SymbolicValue};
use std::cmp::Ordering;

use super::super::ctx;
use super::{BVType, BitVec};
use ruint::Uint;
use z3::ast::{Ast, BV};

impl<const SZ: usize> BitVec<SZ> {
    pub fn new_literal(val: u64) -> Self {
        let ctx = ctx();
        let bv = BV::from_u64(ctx, val, (SZ * 8) as u32);
        Self {
            inner: BVType::Z3(bv),
            typ: Default::default(),
        }
    }

    pub fn new_const(name: impl AsRef<str>) -> Self {
        let bv = BV::new_const(ctx(), name.as_ref(), (SZ * 8) as u32);
        Self {
            inner: BVType::Z3(bv),
            typ: Default::default(),
        }
    }
}

impl<const SZ: usize> SymbolicValue<BV<'static>, u64> for BitVec<SZ> {
    fn new_literal(val: u64) -> Self {
        let ctx = ctx();
        let bv = BV::from_u64(ctx, val, (SZ * 8) as u32);
        Self {
            inner: BVType::Z3(bv),
            typ: Default::default(),
        }
    }

    fn new_const(name: impl AsRef<str>) -> Self {
        let bv = BV::new_const(ctx(), name.as_ref(), (SZ * 8) as u32);
        Self {
            inner: BVType::Z3(bv),
            typ: Default::default(),
        }
    }
}
