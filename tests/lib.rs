extern crate ser;
use ser::{
    traits::*,
    machine::*,
    stack::*,
    storage::*,
    memory::*,
    conversion::*,
    bvc,
    bvi,
    parser::*,
};
use z3::ast::*;

pub const COUNTER_WITH_STORAGE_MAPPING: &str = r#"608060405234801561001057600080fd5b5060056000806001815260200190815260200160002081905550610197806100396000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063846719e01461003b578063d78233d61461006b575b600080fd5b6100556004803603810190610050919061010a565b61009b565b6040516100629190610146565b60405180910390f35b6100856004803603810190610080919061010a565b6100b7565b6040516100929190610146565b60405180910390f35b6000806000838152602001908152602001600020549050919050565b60006020528060005260406000206000915090505481565b600080fd5b6000819050919050565b6100e7816100d4565b81146100f257600080fd5b50565b600081359050610104816100de565b92915050565b6000602082840312156101205761011f6100cf565b5b600061012e848285016100f5565b91505092915050565b610140816100d4565b82525050565b600060208201905061015b6000830184610137565b9291505056fea2646970667358fe122066b287fef10118cba238fe38953bfefe938afefefefefe94fefe3682fefefefe64736f6c63430008110033"#;
pub const SUPERSIMPLE: &str = r#"604260005260206000F3"#;
pub const STORAGE_SIMPLE: &str = r#"6080604052348015600f57600080fd5b506004361060325760003560e01c80631ab06ee5146037578063fac333ac146056575b600080fd5b605460423660046085565b60009182526020829052604090912055565b005b6073606136600460a6565b60006020819052908152604090205481565b60405190815260200160405180910390f35b60008060408385031215609757600080fd5b50508035926020909101359150565b60006020828403121560b757600080fd5b503591905056fea26469706673582212204a6bf5c04a6e273d775914b20b0bab1bca28228be5562d496002981e13ff015264736f6c63430008130033"#; 
#[test]

fn can_run_simple_parsed_pgm() {
    let pgm = Parser::with_pgm(SUPERSIMPLE).parse();
    let mut evm = Evm::new(pgm);

    {
        let sat_branches = evm.exec();
       //eprintln!("EXECUTION: {:#?}", sat_branches);
  
        // eprintln!("SAT BRANCHES: {:#?}", sat_branches);
        // assert_eq!(sat_branches.len(), 1);
        // let top = sat_branches
        //     .first()
        //     .unwrap()
        //     .0
        //      .0
        //     .stack()
        //     .peek()
        //     .cloned()
        //     .unwrap();
        // eprintln!("Stack top size: {:#?}", top.as_ref().get_size());
        // assert_eq!(top, bvi(5));
        let leaf = sat_branches.states.leaves();
        assert_eq!(1, leaf.len());
        let final_tree = leaf.first().unwrap().clone();
       // eprintln!("FINAL TREE: {:#?}", final_tree);
        let mut mem_val = final_tree.val.mem_read(bvi(0));
        mem_val.simplify();
        assert_eq!(bvi(66), mem_val);
    }
}

#[test]
fn can_run_simple_storage_pgm() {
    let pgm = Parser::with_pgm(STORAGE_SIMPLE).parse();
    let mut evm = Evm::new(pgm);

    {
        let sat_branches = evm.exec();
       //eprintln!("EXECUTION: {:#?}", sat_branches);
  
        // eprintln!("SAT BRANCHES: {:#?}", sat_branches);
        // assert_eq!(sat_branches.len(), 1);
        // let top = sat_branches
        //     .first()
        //     .unwrap()
        //     .0
        //      .0
        //     .stack()
        //     .peek()
        //     .cloned()
        //     .unwrap();
        // eprintln!("Stack top size: {:#?}", top.as_ref().get_size());
        // assert_eq!(top, bvi(5));
        let leaf = sat_branches.states.leaves();
        assert_eq!(2, leaf.len());
        eprintln!("LEAVES: {:#?}", leaf);
        let final_tree = leaf.get(1).unwrap().clone();
       // eprintln!("FINAL TREE: {:#?}", final_tree);
        let mut mem_val = final_tree.val.mem_read(bvi(64));
        mem_val.simplify();
        assert_eq!(bvi(128), mem_val);
    }
}