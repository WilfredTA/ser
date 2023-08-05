#![allow(unused_imports, unused)]
use ser::{
    bvc, bvi, conversion::*, instruction::Instruction, machine::*, memory::*, parser::*, stack::*,
    storage::*, traits::*,
};
use z3::ast::*;

// Deployment code for ./simple_storage.rs
pub const COUNTER_WITH_STORAGE_MAPPING_DEPLOY: &str = r#"608060405234801561001057600080fd5b5060056000806001815260200190815260200160002081905550610197806100396000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063846719e01461003b578063d78233d61461006b575b600080fd5b6100556004803603810190610050919061010a565b61009b565b6040516100629190610146565b60405180910390f35b6100856004803603810190610080919061010a565b6100b7565b6040516100929190610146565b60405180910390f35b6000806000838152602001908152602001600020549050919050565b60006020528060005260406000206000915090505481565b600080fd5b6000819050919050565b6100e7816100d4565b81146100f257600080fd5b50565b600081359050610104816100de565b92915050565b6000602082840312156101205761011f6100cf565b5b600061012e848285016100f5565b91505092915050565b610140816100d4565b82525050565b600060208201905061015b6000830184610137565b9291505056fea2646970667358fe122066b287fef10118cba238fe38953bfefe938afefefefefe94fefe3682fefefefe64736f6c63430008110033"#;
fn main() {
    let pgm = Parser::with_pgm(COUNTER_WITH_STORAGE_MAPPING_DEPLOY).parse();
    let mut evm = Evm::new(pgm);

    let execution = evm.exec();
    {
        let leaf = execution.states.leaves();
        assert_eq!(2, leaf.len());
        println!("LEAVES: {:#?}", leaf);
        let mem_val_leaf_2 = leaf.get(1).unwrap().val.mem();
        println!("MEM SIZE: {}", mem_val_leaf_2.size());
        println!("MEM M_SIZE: {}", mem_val_leaf_2.m_size());
    
        println!("LEAF 2 MEMORY CONCATENATED: {}", mem_val_leaf_2.memory_string());
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