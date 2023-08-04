#![allow(unused_imports, unused)]
use ser::{
    bvc, bvi, conversion::*, machine::*, memory::*, parser::*, stack::*, storage::*, traits::*,
};
use z3::{ast::*, SatResult};
/*
SHOULD REVERT:

PUSH1 0x42
PUSH1 0x00
PUSH2 0x5000
CALLDATASIZE
SWAP2
PUSH1 0x0e // 14
JUMPI
REVERT
JUMPDEST
PUSH1 0
RETURN


SHOULD NOT REVERT:

PUSH1 0x42
PUSH1 0x00
PUSH2 0x5000
PUSH1 0x40
SWAP3
PUSH1 0x0e // 14
JUMPI
REVERT
JUMPDEST
PUSH1 0x10
RETURN


MAYBE REVERT: 
PUSH1 0x42
PUSH1 0x00
PUSH2 0x5000
CALLDATASIZE
PUSH1 0x0d
JUMPI
REVERT
JUMPDEST
PUSH1 0x00
RETURN

*/
const SWAP2_JUMPI_REVERT: &str = r#"604260006150003691600d57fd5b6000f3"#;
const SWAP2_JUMPI_MAYBE_REVERT: &str = r#"6042600061500036600d57fd5b6000f3"#;
const SWAP3_JUMPI_RETURN_16: &str = r#"60426000615000604091600e57fd5b6000f3"#;
fn main() {
    let pgm = Parser::with_pgm(SWAP2_JUMPI_MAYBE_REVERT).parse();
    let mut evm = Evm::new(pgm);
    let execution = evm.exec();
    //eprintln!("Execution tree: {:#?}", execution.states.clone());
    // Should have two paths: both reachable. The first reachable path is the one in which calldata is zero and there is a revert.
    // The second reachable path is the one in which calldata is not zero and there is not a revert.
    let reachability_report = Evm::exec_check(execution);
    assert_eq!(2, reachability_report.len());
    eprintln!("REPORT: {:#?}", reachability_report);
    assert_eq!(
        SatResult::Sat,
        reachability_report.first().unwrap().1.unwrap()
    );
    assert_eq!(
        SatResult::Sat,
        reachability_report.get(1).unwrap().1.unwrap()
    );
}
