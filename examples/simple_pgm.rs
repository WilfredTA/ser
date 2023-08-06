#![allow(unused_imports)]
use ser::{
    bvc, bvi, conversion::*, machine::*, memory::*, parser::*, stack::*, storage::*, traits::*,
};
use z3::ast::*;

pub const SUPERSIMPLE: &str = r#"604260005260206000F3"#;
fn main() {
    let pgm = Parser::with_pgm(SUPERSIMPLE).parse();
    let mut evm = Evm::with_pgm(pgm);

    let execution_trace = evm.exec();
    {
        let leaf = execution_trace.states.leaves();
        assert_eq!(1, leaf.len());
        let final_tree = leaf.first().unwrap().clone();

        let mut mem_val = final_tree.val.mem_read(bvi(0));
        mem_val.simplify();
        assert_eq!(bvi(66), mem_val);
    }

    let tree_flattened = execution_trace.states.into_iter().collect::<Vec<_>>();
    eprintln!(
        "Nodes in tree: {}\nTree Nodes: {:#?}",
        tree_flattened.len(),
        tree_flattened
    );
}
