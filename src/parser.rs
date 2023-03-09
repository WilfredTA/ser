use crate::instruction::Instruction;
use revm::{
    opcode::OpCode, OPCODE_JUMPMAP
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

        let mut pgm = vec![];
        for b in bytes {
            
            let inst = Instruction::from(*b);
            
            pgm.push(inst);            

        }
        pgm

       
    }

    


}

fn is_push(b: u8) -> bool {
    0x60 <= b && 0x7f >= b
}

fn push_size(b: u8) -> u8 {
    if b > 0x60 {
        b - 0x60 + 1
    } else {
        0
    }

}


impl Instruction {
    pub fn from_byte(value: u8) -> Self {
        value.into()

    }

    // Has to handle when it's a push or dup, otherwise easy 1-1 conversion
    pub fn from_slice(bytes: &[u8]) -> Vec<Instruction> {
        todo!()
    }
}
impl From<u8> for Instruction {
    fn from(value: u8) -> Self {

        match value {
            0x00 => Instruction::Stop,
            0x01 => Instruction::Add,
            0x02 => Instruction::Mul,
            0x03 => Instruction::Sub,
            0x04 => Instruction::Div,
            0x05 => Instruction::SDiv,
            0x06 => Instruction::Mod,
            0x07 => Instruction::SMod,
            0x08 => Instruction::AddMod,
            0x09 => Instruction::MulMod,
            0x0a => Instruction::Exp,
            0x0b => Instruction::SignExtend,
            0x10 => Instruction::Lt,
            0x11 => Instruction::Gt,
            0x12 => Instruction::Slt,
            0x13 => Instruction::Sgt,
            0x14 => Instruction::Eq,
            0x15 => Instruction::And,
            0x16 => Instruction::Or,
            0x17 => Instruction::Xor,
            0x18 => Instruction::Not,
            0x19 => Instruction::Byte,
            0x1a => Instruction::Shl,
            0x1b => Instruction::Shr,
            0x20 => Instruction::Sha3,
            0x30 => Instruction::Address,
            0x31 => Instruction::Balance,
            0x32 => Instruction::Origin,
            0x33 => Instruction::Caller,
            0x34 => Instruction::CallValue,
            0x35 => Instruction::CallDataLoad,
            0x36 => Instruction::CallDataSize,
            0x37 => Instruction::CallDataCopy,
            0x38 => Instruction::CodeSize,
            0x39 => Instruction::CodeCopy,
            0x3a => Instruction::GasPrice,
            0x3b => Instruction::ExtCodeSize,
            0x3c => Instruction::ExtCodeCopy,
            0x3d => Instruction::ReturnDataSize,
            0x3e => Instruction::ReturnDataCopy,
            0x3f => Instruction::ExtCodeHash,
            0x40 => Instruction::BlockHash,
            0x41 => Instruction::Coinbase,
            0x42 => Instruction::Timestamp,
            0x43 => Instruction::Number,
            0x44 => Instruction::Difficulty,
            0x45 => Instruction::GasLimit,
            0x46 => Instruction::ChainId,
            0x47 => Instruction::SelfBalance,
            0x50 => Instruction::Pop,
            0x51 => Instruction::MLoad,
            0x52 => Instruction::MStore,
            0x53 => Instruction::MStore8,
            0x54 => Instruction::SLoad,
            0x55 => Instruction::SStore,
            0x56 => Instruction::Jump,
            0x57 => Instruction::JumpI,
            0x58 => Instruction::Pc,
            0x59 => Instruction::MSize,
            0x5a => Instruction::Gas,
            0x5b => Instruction::JumpDest,
            0x60 => Instruction::Push1(Default::default()),
            0x61 => Instruction::Push2(Default::default()),
            0x62 => Instruction::Push3(Default::default()),
            0x63 => Instruction::Push4(Default::default()),
            0x64 => Instruction::Push5(Default::default()), 
            0x65 => Instruction::Push6(Default::default()), 
            0x66 => Instruction::Push7(Default::default()), 
            0x67 => Instruction::Push8(Default::default()), 
            0x68 => Instruction::Push9(Default::default()), 
            0x69 => Instruction::Push10(Default::default()), 
            0x6a => Instruction::Push11(Default::default()),
            0x6b => Instruction::Push12(Default::default()), 
            0x6c => Instruction::Push13(Default::default()), 
            0x6d => Instruction::Push14(Default::default()),
            0x6e => Instruction::Push15(Default::default()), 
            0x6f => Instruction::Push16(Default::default()), 
            0x70 => Instruction::Push17(Default::default()),
            0x71 => Instruction::Push18(Default::default()), 
            0x72 => Instruction::Push19(Default::default()),
            0x73 => Instruction::Push20(Default::default()), 
            0x74 => Instruction::Push21(Default::default()), 
            0x75 => Instruction::Push22(Default::default()), 
            0x76 => Instruction::Push23(Default::default()),  
            0x77 => Instruction::Push24(Default::default()),  
            0x78 => Instruction::Push25(Default::default()), 
            0x79 => Instruction::Push26(Default::default()), 
            0x7a => Instruction::Push27(Default::default()), 
            0x7b => Instruction::Push28(Default::default()), 
            0x7c => Instruction::Push29(Default::default()), 
            0x7d => Instruction::Push30(Default::default()), 
            0x7e => Instruction::Push31(Default::default()), 
            0x7f => Instruction::Push32(Default::default()), 
            0x80 => Instruction::Dup1,  
            0x81 => Instruction::Dup2,  
            0x82 => Instruction::Dup3,  
            0x83 => Instruction::Dup4,  
            0x84 => Instruction::Dup5,  
            0x85 => Instruction::Dup6,  
            0x86 => Instruction::Dup7,  
            0x87 => Instruction::Dup8,  
            0x88 => Instruction::Dup9,  
            0x89 => Instruction::Dup10,  
            0x8a => Instruction::Dup11,  
            0x8b => Instruction::Dup12,  
            0x8c => Instruction::Dup13,  
            0x8d => Instruction::Dup14,  
            0x8e => Instruction::Dup15,  
            0x8f => Instruction::Dup16,  
            0x90 => Instruction::Swap1,  
            0x91 => Instruction::Swap2,  
            0x92 => Instruction::Swap3,  
            0x93 => Instruction::Swap4,  
            0x94 => Instruction::Swap5,  
            0x95 => Instruction::Swap6,  
            0x96 => Instruction::Swap7,  
            0x97 => Instruction::Swap8,  
            0x98 => Instruction::Swap9,  
            0x99 => Instruction::Swap10,  
            0x9a => Instruction::Swap11,  
            0x9b => Instruction::Swap12,  
            0x9c => Instruction::Swap13, 
            0x9d => Instruction::Swap14,  
            0x9e => Instruction::Swap15,  
            0x9f => Instruction::Swap16, 
            0xa0 => Instruction::Log0, 
            0xa1 => Instruction::Log1,  
            0xa2 => Instruction::Log2,  
            0xa3 => Instruction::Log3,  
            0xa4 => Instruction::Log4, 
            0xf0 => Instruction::Create,  
            0xf1 => Instruction::Call, 
            0xf2 => Instruction::CallCode,  
            0xf3 => Instruction::Return, 
            0xf4 => Instruction::DelegateCall,  
            0xf5 => Instruction::Create2,  
            0xfa => Instruction::StaticCall,  
            0xfd => Instruction::Revert,  
            0xfe => Instruction::Invalid,  
            0xff => Instruction::SelfDestruct,  
            _ => Instruction::Invalid,
        }
        
    }
}

#[test]
fn is_push_works() {
    for b in (0x60_u8..0x7f) {
        assert!(is_push(b));
    }

    for b in (0x00_u8 .. 0x5f) {
        assert!(!is_push(b));
    }

    for b in (0x80..0xff_u8) {
        assert!(!is_push(b));
    }
}