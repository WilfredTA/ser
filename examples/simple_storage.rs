#![allow(unused_imports, unused)]
use ser::{
    bvc, bvi, conversion::*, instruction::Instruction, machine::*, memory::*, parser::*, stack::*,
    storage::*, traits::*,
};
use z3::ast::*;

pub const STORAGE_SIMPLE: &str = r#"6080604052348015600f57600080fd5b506004361060325760003560e01c80631ab06ee5146037578063fac333ac146056575b600080fd5b605460423660046085565b60009182526020829052604090912055565b005b6073606136600460a6565b60006020819052908152604090205481565b60405190815260200160405180910390f35b60008060408385031215609757600080fd5b50508035926020909101359150565b60006020828403121560b757600080fd5b503591905056fea26469706673582212204a6bf5c04a6e273d775914b20b0bab1bca28228be5562d496002981e13ff015264736f6c63430008130033"#;

fn main() {
    let pgm = Parser::with_pgm(STORAGE_SIMPLE).parse();

    let mut evm = Evm::with_pgm(pgm);

    let execution = evm.exec();
    {
        let leaf = execution.states.leaves();
        // as seen here https://bytegraph.xyz/bytecode/e5987a6f24f8af926faddae88de7980f/graph
        assert_eq!(7, leaf.len());
    }

    let reachability_report = Evm::exec_check(execution);
    println!("Report: {:#?}", reachability_report);
    let traces = reachability_report
        .iter()
        .map(|trace| trace.0.iter().map(|t| &t.1).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("traces: {:#?}", traces);
    let reverted_traces = traces
        .into_iter()
        .filter(|t| *t.last().unwrap().clone() == Instruction::Revert)
        .collect::<Vec<_>>();
    println!("TRACES WITH REVERTS {:#?}", reverted_traces);
}
