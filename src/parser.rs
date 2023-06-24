use crate::{instruction::*, smt::BitVec, bvi};
use revm::{
    opcode::OpCode, OPCODE_JUMPMAP
};
use hex::{decode};
pub struct Parser<'a> {
    pgm: &'a str
}

impl<'a> Parser<'a> {

    pub fn with_pgm(pgm: &'a str) -> Self {
        Self {
            pgm
        }
    }

    pub fn parse(&self) -> Vec<Instruction> {
        let bytes = decode(self.pgm).unwrap();
        let mut opcodes: Vec<Instruction> = vec![];
        
        let mut pgm = vec![];
        let mut skip_size = 0;
        let mut idx = 0_usize;
        for b in &bytes {
            
            let b = *b;
            if skip_size > 0 {
                skip_size -= 1;
                continue;
                idx += 1;
            }
            if is_push(b) {
                // push1 0xab push4 0xabcd
                let sz = push_size(b);
                let (inst, push_size, new_bytes) = parse_push(&bytes[idx as usize..]);
                skip_size = push_size;
                idx += skip_size as usize;
                pgm.push(inst);
            } else if is_dup(b) {
                pgm.push(parse_dup(b))
            } else if is_swap(b) {
                pgm.push(parse_swap(b));
            } else {
                let inst = Instruction::from(b);
                pgm.push(inst);  
            }
            idx += 1;
          
        }
        pgm

       
    }

}

// Returns instruction + any left over bytes in the slice after extracting the opcode & opcode args
fn parse_push(bytes: &[u8]) -> (Instruction, u8, &[u8]) {

    let instruction_byte = *bytes.first().unwrap();
    let push_size = push_size(instruction_byte);
    eprintln!("PUSH SIZE IS {}", push_size);
    let push_val = &bytes[1..(push_size + 1) as usize];
    eprintln!("PUSH VAL IS {:#?}", push_val);
    (push_op(push_size, push_val),push_size, &bytes[(push_size) as usize..bytes.len()])
}

fn parse_dup(byte: u8) -> Instruction {
    dup_op(dup_size(byte))
}

fn parse_swap(byte: u8) -> Instruction {
    swap_op(swap_size(byte))
}

fn is_dup(b: u8) -> bool {
    0x80 <= b && 0x8f >= b
}

fn is_swap(b: u8) -> bool {
    0x90 <= b && 0x9f >= b
}

fn is_push(b: u8) -> bool {
    0x60 <= b && 0x7f >= b
}

fn dup_size(b: u8) -> u8 {
    if b >= 0x80 {
        b - 0x80 + 1
    } else {
        0
    }
}

fn swap_size(b: u8) -> u8 {
    if b >= 0x90 {
        b - 0x90 + 1
    } else {
        0
    }
}
fn push_size(b: u8) -> u8 {
    if b >= 0x60 {
        b - 0x60 + 1
    } else {
        0
    }
}


fn push_op(sz: u8, val: &[u8]) -> Instruction {
    let mut zero_len = 8 - val.len();
    let mut buf = vec![];
    for _ in (0..zero_len) {
        buf.push(0);
    }
    buf.extend_from_slice(val);
   
    let mut sliced = [0u8; 8];
    sliced.copy_from_slice(&buf);
    let val = u64::from_be_bytes(sliced);

    match sz {
        1 => push1(BitVec::<1>::new_literal(val)),
        2 => push2(BitVec::<2>::new_literal(val)),
        3 => push3(BitVec::<3>::new_literal(val)),
        4 => push4(BitVec::<4>::new_literal(val)),
        5 => push5(BitVec::<5>::new_literal(val)),
        6 => push6(BitVec::<6>::new_literal(val)),
        7 => push7(BitVec::<7>::new_literal(val)),
        8 => push8(BitVec::<8>::new_literal(val)),
        9 => push9(BitVec::<9>::new_literal(val)),
        10 => push10(BitVec::<10>::new_literal(val)),
        11 => push11(BitVec::<11>::new_literal(val)),
        12 => push12(BitVec::<12>::new_literal(val)),
        13 => push13(BitVec::<13>::new_literal(val)),
        14 => push14(BitVec::<14>::new_literal(val)),
        15 => push15(BitVec::<15>::new_literal(val)),
        16 => push16(BitVec::<16>::new_literal(val)),
        17 => push17(BitVec::<17>::new_literal(val)),
        18 => push18(BitVec::<18>::new_literal(val)),
        19 => push19(BitVec::<19>::new_literal(val)),
        20 => push20(BitVec::<20>::new_literal(val)),
        21 => push21(BitVec::<21>::new_literal(val)),
        21 => push22(BitVec::<22>::new_literal(val)),
        23 => push23(BitVec::<23>::new_literal(val)),
        24 => push24(BitVec::<24>::new_literal(val)),
        25 => push25(BitVec::<25>::new_literal(val)),
        26 => push26(BitVec::<26>::new_literal(val)),
        27 => push27(BitVec::<27>::new_literal(val)),
        28 => push28(BitVec::<28>::new_literal(val)),
        29 => push29(BitVec::<29>::new_literal(val)),
        30 => push30(BitVec::<30>::new_literal(val)),
        31 => push31(BitVec::<31>::new_literal(val)),
        32 => push32(BitVec::<32>::new_literal(val)),
        _ => {
            todo!()
        }
    }
}

fn swap_op(sz: u8) -> Instruction {
    match sz {
        1 => Instruction::Swap1,
        2 => Instruction::Swap2,
        3 => Instruction::Swap3,
        4 => Instruction::Swap4,
        5 => Instruction::Swap5,
        6 => Instruction::Swap6,
        7 => Instruction::Swap7,
        8 => Instruction::Swap8,
        9 => Instruction::Swap9,
        10 => Instruction::Swap10,
        11 => Instruction::Swap11,
        12 => Instruction::Swap12,
        13 => Instruction::Swap13,
        14 => Instruction::Swap14,
        15 => Instruction::Swap15,
        16 => Instruction::Swap16,
        _ => todo!()
    }
}

fn dup_op(sz: u8) -> Instruction {
    match sz {
        1 => dup1(),
        2 => dup2(),
        3 => dup3(),
        4 => dup4(),
        5 => dup5(),
        6 => dup6(),
        7 => dup7(),
        8 => dup8(),
        9 => dup9(),
        10 => dup10(),
        11 => dup11(),
        12 => dup12(),
        13 => dup13(),
        14 => dup14(),
        15 => dup15(),
        16 => dup16(),
        _ => todo!()
    }    
}


impl Instruction {
    pub fn from_byte(value: u8) -> Self {
        value.into()
    }

    // Has to handle when it's a push or dup, otherwise easy 1-1 conversion
    pub fn from_slice(bytes: &[u8]) -> Instruction {
       let instruction_byte = *bytes.first().unwrap();
       
       if is_push(instruction_byte) {
        let push_size = push_size(instruction_byte);
        let push_val = &bytes[1..push_size as usize];
        push_op(push_size, push_val)
       } else if is_dup(instruction_byte) {
            let dup_size = dup_size(instruction_byte);
            dup_op(dup_size)
       } else if is_swap(instruction_byte) {
            let swap_size = swap_size(instruction_byte);
            swap_op(swap_size)
       } else {
            Instruction::from(instruction_byte)
       }
       
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
            0x15 => Instruction::IsZero,
            0x16 => Instruction::And,
            0x17 => Instruction::Or,
            0x18 => Instruction::Xor,
            0x19 => Instruction::Not,
            0x1a => Instruction::Byte,
            0x1b => Instruction::Shl,
            0x1c => Instruction::Shr,
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

/**
 * pragma solidity ^0.8.3;

contract Counter {
    uint public count;

    // Function to get the current count
    function get() public view returns (uint) {
        return count;
    }

    // Function to increment count by 1
    function inc() public {
        count += 1;
    }

    // Function to decrement count by 1
    function dec() public {
        count -= 1;
    }
}
 */
#[test]
fn can_parse_simple_pgm() {
    const COUNTER_SOL_CODE: &'static str = "604260005260206000610040F3";
    
    let pgm = Parser::with_pgm(COUNTER_SOL_CODE).parse();
    let sixty_four: BitVec<32> = bvi(0x0040);
    eprintln!("SIXTY FOUR: {:#?}", sixty_four);
    let expected = vec![
        Instruction::Push1(bvi(0x42)),
        Instruction::Push1(bvi(0)),
        Instruction::MStore,
        Instruction::Push1(bvi(0x20)),
        Instruction::Push1(bvi(0)),
        push2(bvi(0x0040)),
        Instruction::Return
    ];

    assert_eq!(expected, pgm);


}

#[test]
fn can_parse_larger_pgm_with_storage() {
    let pgm_raw = crate::test::COUNTER_WITH_STORAGE_MAPPING;
    let pgm = Parser::with_pgm(pgm_raw).parse();

    let expected_first_30 = vec![
        push1(bvi(0x80)),
        push1(bvi(0x40)),
        Instruction::MStore,
        Instruction::CallValue,
        dup1(),
        Instruction::IsZero,
        push2(bvi(0x0010)),
        jumpi(),
        push1(bvi(0x00)),
        dup1(),
        Instruction::Revert,
        Instruction::JumpDest,
        Instruction::Pop,
        push1(bvi(0x05)),
        push1(bvi(0x00)),
        dup1(),
        push1(bvi(0x01)),
        dup2(),
        Instruction::MStore,
        push1(bvi(0x20)),
        Instruction::Add,
        Instruction::Swap1,
        dup2(),
        Instruction::MStore,
        push1(bvi(0x20)),
        Instruction::Add,
        push1(bvi(0x00)),
        Instruction::Sha3,
        dup2(),
        Instruction::Swap1,
        Instruction::SStore,
        Instruction::Pop,
        push2(bvi(0x0197))

    ];


    let pgm_first_30 = (&pgm[..33]).to_vec();
    assert_eq!(expected_first_30, pgm_first_30);
}