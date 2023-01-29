mod bitvec;
pub use bitvec::*;
use std::{borrow::Borrow, cell::RefCell, sync::Arc};

use z3_ext::{
    ast::{Ast, Bool, BV},
    Config, Context, Solver,
};

use once_cell::{sync::Lazy, sync::OnceCell};
use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;

thread_local! {
    static CFG: RefCell<Config> = RefCell::new(Config::new());

}

thread_local! {
    static CONTEXT: Context =   CFG.with(|c| {
        let mut cfg = c.borrow_mut();
         cfg.set_model_generation(true);
         Context::new(&cfg)
        });
}

pub struct Ctx {}

impl Ctx {
    pub fn current(&self) -> &'static Context {
        CONTEXT.with(|c| unsafe { std::mem::transmute::<&Context, &'static Context>(c) })
    }
}

pub fn ctx() -> &'static Context {
    Ctx {}.current()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverType {
    Z3,
}

impl SolverType {
    pub fn name(&self) -> String {
        match self {
            SolverType::Z3 => "z3".to_string(),
        }
    }
}
impl Default for SolverType {
    fn default() -> Self {
        SolverType::Z3
    }
}

pub trait SymbolicValue<ExtSolverValType, LiteralTyp>
where
    Self: From<ExtSolverValType>,
    ExtSolverValType: From<Self>,
    Self: AsRef<ExtSolverValType>,
{
    fn new_literal(data: LiteralTyp) -> Self;

    fn new_const(name: impl AsRef<str>) -> Self;
}