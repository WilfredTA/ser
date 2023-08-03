use super::evm::*;
use crate::{ctx, z3_ext::ast::Ast, z3_ext::ast::Bool, instruction::Instruction};
use std::borrow::BorrowMut;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NodeId {
    id: Uuid,
    parent: Option<Uuid>,
}

impl Default for NodeId {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
        }
    }
}

impl NodeId {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn pid(&self) -> Option<Uuid> {
        self.parent
    }
    pub fn new_with_parent(pid: &Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: Some(*pid),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct StateTree<'ctx> {
    pub id: NodeId,
    pub val: EvmState,
    pub(crate) path_condition: Option<Bool<'ctx>>,
    pub(crate) left: Option<Box<StateTree<'ctx>>>,
    pub(crate) right: Option<Box<StateTree<'ctx>>>,
}



impl<'ctx> From<(EvmState, Bool<'ctx>)> for StateTree<'ctx> {
    fn from(t: (EvmState, Bool<'ctx>)) -> Self {
        Self {
            id: NodeId::new(),
            val: t.0,
            path_condition: Some(t.1),
            left: None,
            right: None,
        }
    }
}

impl<'ctx> From<(EvmState, Option<Bool<'ctx>>)> for StateTree<'ctx> {
    fn from(t: (EvmState, Option<Bool<'ctx>>)) -> Self {
        Self {
            id: NodeId::new(),
            val: t.0,
            path_condition: t.1,
            left: None,
            right: None,
        }
    }
}

impl<'ctx> From<(EvmState, Vec<Bool<'ctx>>)> for StateTree<'ctx> {
    fn from(t: (EvmState, Vec<Bool<'ctx>>)) -> Self {
        let conds = t.1.iter().collect::<Vec<_>>();
        let cond = if conds.is_empty() {
            None
        } else {
            Some(Bool::and(ctx(), conds.as_slice()))
        };
        Self {
            id: NodeId::new(),
            val: t.0,
            path_condition: cond,
            left: None,
            right: None,
        }
    }
}

impl<'ctx> StateTree<'ctx> {
    pub fn update(&self, val: EvmState) -> StateTree<'ctx> {
        let mut new_self = self.clone();
        new_self.val = val;
        new_self
    }

    pub fn inorder(&self) -> Vec<(NodeId, EvmState, Option<Bool<'ctx>>)> {
        let mut items = vec![(
            self.id.clone(),
            self.val.clone(),
            self.path_condition.clone(),
        )];

        if let Some(left) = &self.left {
            let left_tree_inorder = left.inorder();
            items.extend(left_tree_inorder);
        }
        if let Some(right) = &self.right {
            let right_tree_inorder = right.inorder();
            items.extend(right_tree_inorder);
        }
        items
    }

    pub fn find_paths(node: &Option<Box<StateTree<'ctx>>>, current_path: &mut Vec<(NodeId, Instruction, Option<Bool<'ctx>>)>, all_paths: &mut Vec<Vec<(NodeId, Instruction, Option<Bool<'ctx>>)>> ) {
        if let Some(ref node) = *node {
            current_path.push((node.id.clone(), node.val.curr_instruction(), node.path_condition.clone()));
    
            if node.left.is_none() && node.right.is_none() {
                all_paths.push(current_path.clone());
            } else {
                Self::find_paths(&node.left, current_path, all_paths);
                Self::find_paths(&node.right, current_path, all_paths);
            }
    
            current_path.pop();
        }
    }
    pub fn inorder_stateless(&self) -> Vec<(NodeId, Option<Bool<'ctx>>)> {
        let mut items = vec![(
            self.id.clone(),
            self.path_condition.clone(),
        )];

        if let Some(left) = &self.left {
            let left_tree_inorder = left.inorder_stateless();
            items.extend(left_tree_inorder);
        }
        if let Some(right) = &self.right {
            let right_tree_inorder = right.inorder_stateless();
            items.extend(right_tree_inorder);
        }
        items
    }

    pub fn insert(&mut self, tree: impl Into<StateTree<'ctx>>) {
        if let Some(left) = &mut self.left {
            left.insert(tree);
        } else if let Some(right) = &mut self.right {
            right.insert(tree);
        } else if self.left.is_none() {
            self.left = Some(Box::new(tree.into()));
        } else {
            self.right = Some(Box::new(tree.into()));
        }
    }

    pub fn find_by_id(&self, id: &NodeId) -> Option<&StateTree<'ctx>> {
        if self.id == *id {
            Some(self)
        } else {
            if let Some(left) = self.left.as_ref() {
                if let Some(found_l) = left.find_by_id(id) {
                    return Some(found_l);
                }
            }
            if let Some(right) = self.right.as_ref() {
                if let Some(found_r) = right.find_by_id(id) {
                    return Some(found_r);
                }
            }
            None
        }
    }

    pub fn insert_left_helper(
        &mut self,
        tree: impl Into<StateTree<'ctx>>,
        id: Uuid,
    ) -> Option<&StateTree> {
        let tree = tree.into();
        let inserted_id = tree.id.id;

        if self.id.id == id {
            let tree = Box::new(tree);
            self.left = Some(tree);
            self.left.as_ref().map(|t| t.as_ref())
        } else {
            let left_result = if let Some(left) = &mut self.left {
                left.insert_left_helper(tree.clone(), id)
            } else {
                None
            };

            if let Some(res) = left_result {
                return Some(res);
            }

            if let Some(right) = &mut self.right {
                right.insert_left_helper(tree, id)
            } else {
                None
            }
        }
    }

    pub fn insert_right_helper(
        &mut self,
        tree: impl Into<StateTree<'ctx>>,
        id: Uuid,
    ) -> Option<&StateTree> {
        let tree = tree.into();
        let inserted_id = tree.id.id;

        if self.id.id == id {
            self.right = Some(Box::new(tree));
            self.right.as_ref().map(|t| t.as_ref())
        } else {
            let left_result = if let Some(left) = &mut self.left {
                left.insert_right_helper(tree.clone(), id)
            } else {
                None
            };

            if let Some(res) = left_result {
                return Some(res);
            }

            if let Some(right) = &mut self.right {
                right.insert_right_helper(tree, id)
            } else {
                None
            }
        }
    }
    pub fn insert_left_of(&mut self, tree: impl Into<StateTree<'ctx>>, id: Uuid) -> NodeId {
        match self.insert_left_helper(tree, id) {
            Some(i) => i.id.clone(),
            None => panic!("Could not find id {id} in the state tree"),
        }
        // if self.id.id == id {
        //     let mut insert = tree;
        //     insert.id.parent = Some(self.id.id);
        //     self.left = Some(Box::new(insert));
        //     return inserted_id;
        // } else if let Some(left) = &mut self.left {
        //    return left.insert_left_of(tree, id);
        //
        // } else if let Some(right) = &mut self.right {
        //   return right.insert_left_of(tree, id);
        // }
        // eprintln!("ALL NODES: {:?}", self.inorder());
    }

    pub fn insert_right_of(&mut self, tree: impl Into<StateTree<'ctx>>, id: Uuid) -> NodeId {
        match self.insert_right_helper(tree, id) {
            Some(i) => i.id.clone(),
            None => panic!("Could not find id {id} in the state tree"),
        }
    }

    pub fn leaves(&self) -> Vec<StateTree> {
        let mut leaves = vec![];

        if self.left.is_none() && self.right.is_none() {
            leaves.push((self.val.clone(), self.path_condition.clone()).into());
            return leaves;
        }

        if let Some(left) = self.left.as_ref() {
            leaves.extend(left.leaves());
        }

        if let Some(right) = self.right.as_ref() {
            leaves.extend(right.leaves())
        }

        // if let Some(left) = &self.left {

        //     if left.left.is_none() && left.right.is_none() {
        //         leaves.push((left.val.clone(), left.path_condition.clone()).into());
        //     } else {
        //         leaves.extend(left.leaves());
        //     }
        // }

        // if let Some(right) = &self.right {
        //     if right.right.is_none() && right.left.is_none() {
        //         leaves.push((right.val.clone(), right.path_condition.clone()).into());
        //     } else {
        //         leaves.extend(right.leaves());
        //     }
        // }
        leaves
    }

    pub fn update_mut(&mut self, val: EvmState) {
        self.val = val;
    }

    pub fn push_branch(&mut self, val: EvmState, constraint: Bool<'ctx>) {
        if self.right.is_none() {
            self.right = Some(Box::new(StateTree {
                id: NodeId::new(),
                val,
                path_condition: Some(constraint),
                left: None,
                right: None,
            }));
        } else if let Some(left) = &mut self.left {
            left.push_branch(val, constraint)
        }
    }

    pub fn push(&mut self, val: EvmState, constraint: Bool<'ctx>) {
        if self.left.is_none() {
            self.left = Some(Box::new(StateTree {
                id: NodeId::new(),
                val,
                path_condition: Some(constraint),
                left: None,
                right: None,
            }));
        } else if let Some(left) = &mut self.left {
            let final_constraint = if let Some(cond) = &self.path_condition {
                Bool::and(ctx(), &[cond, &constraint])
            } else {
                constraint
            };
            // This ensures that the constraints of each node is a conjunction of all of its ancestors constraints + the new branch condition.
            let new_constraint = final_constraint;
            left.push(val, new_constraint);
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct StateTreeIterator<'ctx> {
    curr_state: Option<(EvmState, Option<Bool<'ctx>>)>,
    nexts: Vec<StateTree<'ctx>>,
}

impl<'ctx> Iterator for StateTreeIterator<'ctx> {
    type Item = (EvmState, Option<Bool<'ctx>>);

    fn next(&mut self) -> Option<Self::Item> {
        let curr = &self.curr_state;
        let nxt = self.nexts.pop();

        if let Some(nxtt) = nxt {
            if let Some(left) = nxtt.left {
                self.nexts.push(*left);
            }

            if let Some(right) = nxtt.right {
                self.nexts.push(*right);
            }

            Some((nxtt.val, nxtt.path_condition))
        } else {
            None
        }
    }
}
impl<'ctx> IntoIterator for StateTree<'ctx> {
    type Item = (EvmState, Option<Bool<'ctx>>);

    type IntoIter = StateTreeIterator<'ctx>;

    fn into_iter(self) -> Self::IntoIter {
        let (left, right) = (self.left, self.right);
        let mut iterator = StateTreeIterator {
            curr_state: Some((self.val, self.path_condition)),
            nexts: vec![],
        };

        if let Some(left) = left {
            iterator.nexts.push(*left);
        }

        if let Some(right) = right {
            iterator.nexts.push(*right);
        }
        iterator
    }
}
