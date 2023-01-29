pub mod z3;
pub use self::z3::*;
use super::{ctx, SolverType};
use z3_ext::ast::{Ast, BV};

#[derive(Debug, Clone)]
pub enum BVType {
    Z3(BV<'static>),
}

#[derive(Debug, Clone)]
pub struct BitVec<const SZ: u32> {
    pub inner: BVType,
    typ: super::SolverType,
}


impl<const SZ: u32> PartialEq for BitVec<SZ> {
    fn eq(&self, other: &Self) -> bool {
        
        match &self.inner {
            BVType::Z3(bv) => {
                //eprintln!("LHS BV EQ: {:#?}", bv);
                match &other.inner {
                    BVType::Z3(bvo) => {
                        //eprintln!("RHS BV EQ: {:#?}", bvo);
                        if let Some(bv1) = bv.as_u64() {
                            if let Some(bv2) = bvo.as_u64() {
                                bv1 == bv2
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    },
                    _ => false
                }
            },
            _ => false
        }
    }
}