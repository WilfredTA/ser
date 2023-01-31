#![allow(unused)]
// #![feature(adt_const_params)]
extern crate z3 as z3_ext;
pub mod instruction;
pub mod smt;
pub mod record;
pub mod machine;
pub mod stack;
pub mod memory;
pub mod state;
pub mod traits;
use instruction::*;
use smt::*;
use stack::*;
use z3_ext::{
    ast::{Ast, Bool, Int, BV},
    AstKind, Config, Context, Model, SatResult, Solver,
};

use std::cell::{Ref, RefCell};



#[derive(Debug, Clone)]
pub struct MachineState {
    pc: usize,
    stack: Stack<32>,
    pgm: Vec<Instruction>,
}
#[derive(Debug, Clone)]
pub struct MachineStateDiff<'ctx> {
    pc: usize,
    constraints: Vec<Bool<'ctx>>,
}

impl<'ctx> MachineStateDiff<'ctx> {
    pub fn with_constraint(c: Bool<'ctx>) -> Self {
        Self {
            pc: 0,
            constraints: vec![c],
        }
    }
    pub fn with_constraints(c: Vec<Bool<'ctx>>) -> Self {
        Self {
            pc: 0,
            constraints: c,
        }
    }
    pub fn new(pc: usize, c: Vec<Bool<'ctx>>) -> Self {
        Self { pc, constraints: c }
    }
}

impl<'ctx> MachineState {
    fn curr_inst(&self) -> &Instruction {
        self.pgm.get(self.pc).unwrap()
    }

    fn tick(&mut self) {
        self.pc += 1;
    }

    fn step(&mut self) -> Instruction {
        let inst = self.pgm.get(self.pc).unwrap().clone();
        self.pc += 1;
        inst
    }

    fn add(&mut self) -> Option<MachineStateDiff<'ctx>> {
        let top = self.stack.pop();
        let nxt = self.stack.pop();
        self.stack.push(top.as_ref().bvadd(nxt.as_ref()).into());
        None
    }
    // fn revert(&mut self) -> Option<MachineStateDiff<'ctx>> {

    // }

    // fn assert(&mut self,  val: BitVec<32>) -> Option<MachineStateDiff<'ctx>> {
    //     let top = self.stack.peek();
    //     if let Some(top) = top.cloned() {
    //         if top == val {
    //             Some()
    //         }
    //     } 
    // }

    fn sub(&mut self) -> Option<MachineStateDiff<'ctx>> {
        let top = self.stack.pop();
        let nxt = self.stack.pop();
        self.stack.push(top.as_ref().bvsub(nxt.as_ref()).into());
        None
    }

    fn push(&mut self, val: BitVec<32>) -> Option<MachineStateDiff<'ctx>> {
        self.stack.push(val.clone());
        None
    }

    fn iszero(&mut self) -> Option<MachineStateDiff<'ctx>> {
        let tst = self.stack.pop();
        let cond = tst.as_ref().bvule(&BV::from_u64(ctx(), 0, 256));
        let bv1 = BV::from_u64(ctx(), 1, 256);
        let bv2 = BV::from_u64(ctx(), 0, 256);
        let condf = cond.ite(&bv1, &bv2);
        self.stack.push(condf.into());
        None
    }

    fn jumpi(&mut self) -> MachineStateDiff<'ctx> {
        let jump_dest = self.stack.pop();
        eprintln!("JUMP DEST: {:?}", jump_dest);
        let jump_dest_concrete = jump_dest.as_ref().simplify().as_u64().unwrap() as usize;
        eprintln!("JUMP DEST CONC: {:?}", jump_dest_concrete);
        let cond = self.stack.pop();

        let bv_zero = BV::from_u64(ctx(), 0, 256);
        let cond = cond.as_ref()._eq(&bv_zero);
        let cond = Bool::not(&cond);

        MachineStateDiff::new(jump_dest_concrete, vec![cond])
    }
}

impl<'ctx> From<Vec<Instruction>> for MachineState {
    fn from(pgm: Vec<Instruction>) -> Self {
        Self {
            pc: 0,
            stack: Default::default(),
            pgm,
        }
    }
}

pub struct Executor<'ctx> {
    left: Option<Machine<'ctx>>,
    right: Option<Machine<'ctx>>
}

impl<'ctx> Executor<'ctx> {

    pub fn run_right(&mut self) -> Option<Machine<'ctx>> {
        if let Some(right) = &self.right {
            Some(right.clone().run())
        } else {
            None
        }
    }

    pub fn run_left(&mut self) -> Option<Machine<'ctx>> {
        if let Some(left) = &self.left {
            Some(left.clone().run())
        } else {
            None
        }
    }
    pub fn run_once(mut self) -> Self {
       let left = self.run_left();
       let right = self.run_right();
        Self {
            left,
            right
        }
    }


}
#[derive(Debug, Clone)]
pub struct Machine<'ctx> {
    pub state: RefCell<MachineState>,
    diffs: RefCell<Vec<MachineStateDiff<'ctx>>>,
}

impl<'ctx> Machine<'ctx> {
    pub fn new(pgm: Vec<Instruction>) -> Self {
        Self {
            state: RefCell::new(pgm.into()),
            diffs: Default::default(),
        }
    }

    pub fn has_diffs(&self) -> bool {
        self.diffs.borrow().len() > 0
    }

    pub fn diffs(&self) -> Vec<MachineStateDiff<'_>> {
        self.diffs.borrow().clone()
    }

    pub fn step(&self) {
        let inst = self.state.borrow_mut().step();
        eprintln!("CURR INST: {:?}", inst);
        match inst {
            Instruction::Add => {
                let res = self.state.borrow_mut().add();
                if let Some(diff) = res {
                    self.diffs.borrow_mut().push(diff);
                }
            }
            Instruction::Sub => {
                let res = self.state.borrow_mut().sub();
                if let Some(diff) = res {
                    self.diffs.borrow_mut().push(diff);
                }
            }
            Instruction::Push(v) => {
                let res = self.state.borrow_mut().push(v.into());
                if let Some(diff) = res {
                    self.diffs.borrow_mut().push(diff);
                }
            }
            Instruction::IsZero => {
                let res = self.state.borrow_mut().iszero();
                if let Some(diff) = res {
                    self.diffs.borrow_mut().push(diff);
                }
            }
            Instruction::JumpI => {
                let res = self.state.borrow_mut().jumpi();

                self.diffs.borrow_mut().push(res);
            }
            _ => panic!("Unsupported instruction"),
        }
    }

    pub fn run(self) -> Machine<'ctx> {
        let (mut pc, len) = {
            let pc = self.state.borrow().pc.clone();
            let len = self.state.borrow().pgm.len();
            (pc, len)
        };

        eprintln!("PC: {:?}, LEN: {:?}", pc, len);
        // let mut diffs = vec![];
        while pc < len {
            self.step();
            eprintln!("DIFFS NOW {:?}", self.diffs());
            pc += 1;
        }

        self
    }

    pub fn solve(self) -> (Machine<'ctx>, Vec<(SatResult, Option<Model<'ctx>>)>) {
        let ctx = ctx();
        let solver = Solver::new(ctx);
        let path_results = self
            .diffs
            .borrow()
            .iter()
            .map(|d| {
                let constraints = d.constraints.clone();
                for c in &constraints {
                    solver.assert(c);
                }
                let is_sat = solver.check();
                let model = {
                    if is_sat == SatResult::Sat {
                        solver.get_model()
                    } else {
                        None
                    }
                };
                eprintln!("IS SAT: {:?}\nModel: {:?}", is_sat, model);
                (is_sat, model)
            })
            .collect::<Vec<_>>();
        (self, path_results)
    }

    pub fn state(&self) -> Ref<MachineState> {
        self.state.borrow()
    }
}


pub fn bvi(val: impl Into<i32>) -> BitVec<32> {
    BitVec::new_literal(val.into() as u64)
}
pub fn bvc(val: impl AsRef<str>) -> BitVec<32> {
    BitVec::new_const(val)
}

#[test]
#[ignore]
fn basic_step() {
    let one = bvi(1);

    let two = bvi(2);
    //let three = BitVec::new_literal(3);
    let four = bvi(4);
    let a = bvc("a");

    /**
     * 
     * 2 pc 0
     * 1 2 pc 1
     * a 1 2 pc 2
     * (a + 1) 2 pc 3
     * 7 (a + 1) 2 pc 4
     * 
     */
    let pgm = vec![
        Instruction::Push(two.clone()),
        Instruction::Push(one),
        Instruction::Push(a),
        Instruction::Add,
        Instruction::Push(bvi(7)),
        // Instruction::Sub,
        // Instruction::Push(four),
    
        Instruction::JumpI,
        Instruction::Push(bvi(24)),
    ];

    let mut mach = Machine::new(pgm);
    let mach = mach.run();

    assert!(mach.diffs().len() == 1);
    let (mach, paths) = mach.solve();
    assert!(paths.iter().all(|p| p.0 == SatResult::Sat));
    let stack_top = mach.state.borrow().stack.peek().unwrap().clone();

    assert_eq!(stack_top, bvi(25));
    //assert!(false);
}
