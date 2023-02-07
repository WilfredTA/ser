use crate::instruction::Instruction;
use revm::{
    opcode::OpCode
};
pub struct Parser<'a> {
    pgm: &'a str
}

impl<'a> Parser<'a> {

    pub fn with_pgm(pgm: &'a str) -> Self {
        Self {
            pgm
        }
    }

    pub fn mnemonic(&self) -> Vec<Instruction> {
        let bytes = self.pgm.as_bytes();
        let mut opcodes: Vec<Instruction> = vec![];

        for b in bytes {
            let b = OpCode::try_from_u8(*b);
            if let Some(op) = b {

            }

        }

        todo!()
    }


}


impl From<&[u8]> for Instruction {
    fn from(value: &[u8]) -> Self {
        let first_byte = *value.first().unwrap();

        let opcode = OpCode::try_from_u8(first_byte).unwrap();
        if opcode.is

        todo!()
    }
}