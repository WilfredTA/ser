#![allow(unused_imports)]
use ser::{
    bvc, bvi, conversion::*, machine::*, memory::*, parser::*, stack::*, storage::*, traits::*,
};
use z3::ast::*;

pub const STORAGE_SIMPLE: &str = r#"6080604052348015600f57600080fd5b506004361060325760003560e01c80631ab06ee5146037578063fac333ac146056575b600080fd5b605460423660046085565b60009182526020829052604090912055565b005b6073606136600460a6565b60006020819052908152604090205481565b60405190815260200160405180910390f35b60008060408385031215609757600080fd5b50508035926020909101359150565b60006020828403121560b757600080fd5b503591905056fea26469706673582212204a6bf5c04a6e273d775914b20b0bab1bca28228be5562d496002981e13ff015264736f6c63430008130033"#;

fn main() {
    let pgm = Parser::with_pgm(STORAGE_SIMPLE).parse();

    let mut evm = Evm::new(pgm);

    let execution = evm.exec();
    {
        let leaf = execution.states.leaves();
        assert_eq!(2, leaf.len());

        let final_tree = leaf.get(1).unwrap().clone();

        let mut mem_val = final_tree.val.mem_read(bvi(64)); // 0x40
        mem_val.simplify();
        assert_eq!(bvi(128), mem_val); // 0x80
    }

    let mut report = std::string::String::default();
    // execution.states.into_iter().for_each(|(state, constraint)| {
    //     report = format!("{}\n{} -- Constraints: {:#?}", report, state, constraint);
    // });
    //eprintln!("Execution report: {}", report);
    eprintln!("Tree: {:#?}", execution.states);
}
