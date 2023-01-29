use crate::smt::{SolverType, SymbolicValue};

use super::super::ctx;
use super::{BVType, BitVec};
use z3::ast::{Ast, BV};

impl<'ctx, const SZ: u32> BitVec<SZ> {
    pub fn new_literal(val: u64) -> Self {
        let ctx = ctx();
        let bv = BV::from_u64(ctx, val, 256);
        Self {
            inner: BVType::Z3(bv),
            typ: Default::default(),
        }
    }

    pub fn new_const(name: impl AsRef<str>) -> Self {
        let bv = BV::new_const(&ctx(), name.as_ref(), SZ * 8);
        Self {
            inner: BVType::Z3(bv),
            typ: Default::default(),
        }
    }
}
impl<const SZ: u32> From<BV<'static>> for BitVec<SZ> {
    fn from(bv: BV<'static>) -> Self {
        let bit_sz = SZ * 8;
        let bvsz = bv.get_size();
        let bv = if bvsz < bit_sz {
            bv.zero_ext(bit_sz - bvsz)
        } else if bvsz > bit_sz {
            bv.extract(bit_sz, 0)
        } else {
            bv
        };
        Self {
            inner: BVType::Z3(bv),
            typ: Default::default(),
        }
    }
}

impl<const SZ: u32> From<BitVec<SZ>> for BV<'static> {
    fn from(bv: BitVec<SZ>) -> Self {
        match bv.inner {
            BVType::Z3(bv) => bv,
            _ => panic!("Should never happen"),
        }
    }
}

impl<const SZ: u32> AsRef<BV<'static>> for BitVec<SZ> {
    fn as_ref(&self) -> &BV<'static> {
        match &self.inner {
            BVType::Z3(bv) => bv,
            _ => panic!("Should never happen"),
        }
    }
}

impl<const SZ: u32> SymbolicValue<BV<'static>, u64> for BitVec<SZ> {
    fn new_literal(val: u64) -> Self {
        let ctx = ctx();
        let bv = BV::from_u64(ctx, val, 256);
        Self {
            inner: BVType::Z3(bv),
            typ: Default::default(),
        }
    }

    fn new_const(name: impl AsRef<str>) -> Self {
        let bv = BV::new_const(&ctx(), name.as_ref(), SZ * 8);
        Self {
            inner: BVType::Z3(bv),
            typ: Default::default(),
        }
    }
}
