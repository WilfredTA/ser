#![allow(unused_imports)]
use ser::{
    bvc, bvi, conversion::*, machine::*, memory::*, parser::*, stack::*, storage::*, traits::*,
};
use z3::ast::*;
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

*/
const SWAP2_JUMPI_REVERT: &str = r#"604260006150003691600d57fd5b6000f3"#;
const SWAP3_JUMPI_RETURN_16: &str = r#"60426000615000604091600e57fd5b6000f3"#;
fn main() {
    let pgm = Parser::with_pgm(SWAP2_JUMPI_REVERT).parse();
    let mut evm = Evm::new(pgm);
    let execution = evm.exec();
    eprintln!("Execution tree: {:#?}", execution.states);
}