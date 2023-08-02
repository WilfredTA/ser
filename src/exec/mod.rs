use uuid::Uuid;

use crate::{record::{
    MachineRecord
}, instruction::Instruction, traits::{MachineInstruction, MachineComponent, MachineState}};
use crate::state::evm::EvmState;
use crate::state::tree::*;


#[derive(Default, Debug)]
pub struct Execution<'ctx> {
    changes: Vec<MachineRecord<32>>,
    program: Vec<Instruction>,
    pub states: StateTree<'ctx>
}

#[derive(Default, Debug)]
pub struct StepRecord 

{
    left_insert: Option<NodeId>,
    right_insert: Option<NodeId>,
    halted_left: bool,
    halted_right: bool
}

impl StepRecord {

    pub fn new(halted_left: bool, halted_right: bool) -> Self {
        Self {
            halted_left, halted_right, ..Default::default()
        }
    }
    pub fn halted_left(&self) -> bool {
        self.halted_left
    }

    pub fn halted_right(&self) -> bool {
        self.halted_right
    }
    pub fn branched(&self) -> bool {
        self.left_insert.is_some() && self.right_insert.is_some()
    }

    pub fn left_id(&self) -> Option<&NodeId> {
        self.left_insert.as_ref()
    }

    pub fn right_id(&self) -> Option<&NodeId> {
        self.right_insert.as_ref()
    }

    pub fn set_left(mut self, left: NodeId) -> Self {
        self.left_insert = Some(left);
        self
    }
    pub fn set_right(mut self, right: NodeId) -> Self {
        self.right_insert = Some(right);
        self
    }
}

impl<'ctx> Execution<'ctx> {

    pub fn new(start_state: EvmState, pgm: Vec<Instruction>) -> Self {
        Self {
            program: pgm,
            states: StateTree::from((start_state, None)),
            ..Default::default()
        }
    }

    // Returns the StepRecord AND updates the Exec state tree
    pub fn step_mut(&mut self)  -> StepRecord { // bool returns if there is a branch
        let curr_state_id = self.states.id.clone();
        let mut curr_state = self.states.val.clone();
        let curr_inst = curr_state.curr_instruction();
        let curr_pc = curr_state.pc();
        let change_rec = curr_inst.exec(&curr_state);

        let is_branch = change_rec.constraints.is_some();
        curr_state.apply_change(change_rec.clone());
        let mut report = StepRecord::new(false, change_rec.halt);
        if is_branch {
            // then curr_state.apply generated the right branching state; thus, a state tree w/
            // an additional constraint
            // and left tree (by convention left path represents straight line execution) is the negation of such constraint
            let mut left_state = curr_state.clone();
            left_state.set_pc(curr_pc + 1);
            if !left_state.can_continue() {
                report.halted_left = true;
            }
            
            let left_tree = StateTree::from((left_state.clone(), change_rec.constraints.clone().unwrap().not()));
            let right_tree = StateTree::from((left_state, change_rec.constraints.unwrap()));
            
            let left_tree_ref = self.states.insert_left_of(left_tree, curr_state_id.id());
            
            
            let right_tree_ref = self.states.insert_right_of(right_tree, curr_state_id.id());
            
            report = report.set_left(left_tree_ref);
            report.set_right(right_tree_ref)

        } else {
            let left_tree = StateTree::from((curr_state, None));
            let left_id = self.states.insert_left_of(left_tree, curr_state_id.id());
            
            report.set_left(left_id)
        }

    }

    // Returns the step record but does not mutate the Exec state tree
    pub fn step(&self)  -> StepRecord { // bool returns if there is a branch
        let curr_state_id = self.states.id.clone();
        let mut curr_state = self.states.val.clone();
        let curr_inst = curr_state.curr_instruction();
        let curr_pc = curr_state.pc();
        let change_rec = curr_inst.exec(&curr_state);

        let is_branch = change_rec.constraints.is_some();
        curr_state.apply_change(change_rec.clone());
        let mut report = StepRecord::new(false, change_rec.halt);
        if is_branch {
            // then curr_state.apply generated the right branching state; thus, a state tree w/
            // an additional constraint
            // and left tree (by convention left path represents straight line execution) is the negation of such constraint
            let mut left_state = curr_state.clone();
            left_state.set_pc(curr_pc + 1);
            if !left_state.can_continue() {
                report.halted_left = true;
            }
            let left_tree = StateTree::from((left_state.clone(), change_rec.constraints.clone().unwrap().not()));
            let right_tree = StateTree::from((left_state, change_rec.constraints.unwrap()));
           
            report.set_left(left_tree.id.clone()).set_right(right_tree.id.clone())

        } else {
            let left_tree = StateTree::from((curr_state, None));
     
            report.set_left(left_tree.id.clone())
        }

    }


    pub fn step_from_mut(&mut self, node_id: &NodeId) -> StepRecord {
        let curr_state_id = node_id.clone();
        let mut curr_state_tree = self.states.find_by_id(node_id).unwrap().clone();
        let mut curr_state = &mut curr_state_tree.val;
        if !curr_state.can_continue() {

            return StepRecord::new(true, true);
        }
        
        let curr_inst = curr_state.curr_instruction();
        let curr_pc = curr_state.pc();
        let change_rec = curr_inst.exec(&curr_state);

        let is_branch = change_rec.constraints.is_some();
        curr_state.apply_change(change_rec.clone());
        let curr_state = curr_state;
        let mut report = StepRecord::new(false, change_rec.halt);
       // assert_eq!(change_rec.halt, curr_state.halt);
        if is_branch {
            // then curr_state.apply generated the right branching state; thus, a state tree w/
            // an additional constraint
            // and left tree (by convention left path represents straight line execution) is the negation of such constraint
            let mut left_state = curr_state.clone();
            left_state.set_pc(curr_pc + 1);
            report.halted_left = left_state.halt;
          
            let left_tree = StateTree::from((left_state.clone(), change_rec.constraints.clone().unwrap().not()));
            let right_tree = StateTree::from((left_state, change_rec.constraints.unwrap()));
            let left_tree_ref = self.states.insert_left_of(left_tree, node_id.id());
            let right_tree_ref = self.states.insert_right_of(right_tree, node_id.id());
            // curr_state_tree.left = Some(Box::new(left_tree));
            // curr_state_tree.right = Some(Box::new(right_tree));
        
            report.set_left(left_tree_ref)
                .set_right(right_tree_ref)

        } else {
            let left_tree = StateTree::from((curr_state.clone(), None));
            let left_tree_ref = self.states.insert_left_of(left_tree, curr_state_id.id());
            //curr_state_tree.left = Some(Box::new(left_tree));
            
            report.set_left(left_tree_ref)
        }

        
    }
}

