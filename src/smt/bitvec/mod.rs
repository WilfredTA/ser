pub mod z3;

use std::hash::{Hash, Hasher};
pub use self::z3::*;
use super::{ctx, SolverType};
use z3_ext::ast::{Ast, BV};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BVType {
    Z3(BV<'static>),
}

pub type SymByte = BitVec<8>;

#[derive(Debug, Clone, Eq)]
pub struct BitVec<const SZ: u32> {
    pub inner: BVType,
    pub(crate) typ: super::SolverType,
}


// impl<const SZ: u32> PartialEq for BitVec<SZ> {
//     fn eq(&self, other: &Self) -> bool {
//         let BVType::Z3(a) = &self.inner;
//         let BVType::Z3(b) = &other.inner;
//
//         a == b
//     }
// }

impl<const SZ: u32> Hash for BitVec<SZ> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
        state.finish();
    }
}
impl<const SZ: u32> BitVec<SZ> {
    pub fn with_bv(bv: BV<'static>) -> Self {
        Self {
            inner: BVType::Z3(bv),
            typ: SolverType::Z3,
        }
    }
    pub fn simplify(&mut self) {
        let BVType::Z3(bv) = &self.inner;
        self.inner = BVType::Z3(bv.simplify());
    }
}
impl<const SZ: u32> Default for BitVec<SZ> {
    fn default() -> Self {
        let ctx = ctx();
        Self {
            inner: BVType::Z3(BV::from_u64(ctx, 0, SZ * 8)),
            typ: SolverType::Z3,
        }
    }
}

impl<const SZ: u32> PartialEq for BitVec<SZ> {
    fn eq(&self, other: &Self) -> bool {

        let a = self.as_ref();
        let b = other.as_ref();

        a.eq(b)
        // match &self.inner {
        //     BVType::Z3(bv) => {
        //         //eprintln!("LHS BV EQ: {:#?}", bv);
        //         match &other.inner {
        //             BVType::Z3(bvo) => {
        //                 //eprintln!("RHS BV EQ: {:#?}", bvo);
        //                 if let Some(bv1) = bv.as_u64() {
        //                     if let Some(bv2) = bvo.as_u64() {
        //                         bv1 == bv2
        //                     } else {
        //                         false
        //                     }
        //                 } else {
        //                     false
        //                 }
        //             }
        //             _ => false,
        //         }
        //     }
        //     _ => false,
        // }
    }
}
