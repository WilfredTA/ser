use std::collections::HashMap;

use crate::{bvi, instruction::*, smt::BitVec};
use hex::decode;
use revm::{opcode::OpCode, OPCODE_JUMPMAP};
use ruint::Uint;
#[derive(Default)]
pub struct Parser<'a> {
    pgm: &'a str,
    parsed: Program,
}

impl<'a> Parser<'a> {
    pub fn with_pgm(pgm: &'a str) -> Self {
        Self {
            pgm,
            ..Default::default()
        }
    }

    pub fn parse(&self) -> Program {
        let bytes = decode(self.pgm).unwrap();
        let mut opcodes: Vec<Instruction> = vec![];

        let mut pgm = vec![];
        let mut skip_size = 0;
        let mut idx = 0_usize;
        let mut pgm_map = HashMap::new();
        let mut pgm_bytes = vec![];
        for b in &bytes {
            let b = *b;
            let b_bv: [u8;1] = [b];
            let bv_b: BitVec<1> = BitVec::from(b_bv);
            pgm_bytes.push(bv_b);
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
                pgm_map.insert(idx, inst.clone());
                idx += skip_size as usize;
                pgm.push(inst);
            } else if is_dup(b) {
                let inst = parse_dup(b);
                pgm_map.insert(idx, inst.clone());
                pgm.push(inst);
            } else if is_swap(b) {
                let inst = parse_swap(b);
                pgm_map.insert(idx, inst.clone());
                pgm.push(inst);
            } else {
                let inst = Instruction::from(b);
                pgm_map.insert(idx, inst.clone());
                pgm.push(inst);
            }
            idx += 1;
        }
        Program {
            map: pgm_map,
            pgm,
            size: idx + 1,
            bytes: pgm_bytes
        }
    }
}

// Returns instruction + any left over bytes in the slice after extracting the opcode & opcode args
fn parse_push(bytes: &[u8]) -> (Instruction, u8, Vec<u8>) {
    let instruction_byte = *bytes.first().unwrap();
    let push_size = push_size(instruction_byte);
    //eprintln!("PUSH SIZE IS {}", push_size);
    //eprintln!("Bytes len is {}", bytes.len());
    if bytes.len() - 1 < push_size as usize {
        let pad_len = push_size as usize - bytes.len() + 1;
        //  eprintln!("pad len: {}", pad_len);
        let mut new_bytes = bytes.to_vec();
        for _ in (0..pad_len) {
            new_bytes.push(0);
        }
        let push_val = &new_bytes[1..(push_size + 1) as usize];
        (
            push_op(push_size, push_val),
            push_size,
            new_bytes[(push_size) as usize..new_bytes.len()].to_vec(),
        )
    } else {
        let push_val = &bytes[1..(push_size + 1) as usize];
        //eprintln!("PUSH VAL IS {:#?}", push_val);
        (
            push_op(push_size, push_val),
            push_size,
            bytes[(push_size) as usize..bytes.len()].to_vec(),
        )
    }
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

// Add zeroes to the left side of a byte array until the byte array is a certain
pub fn zero_extend<const SZ: usize>(bytes: &[u8]) -> [u8; SZ] {
    let mut extended_bytes: [u8; SZ] = [0u8; SZ];
    if bytes.len() < SZ {
        let pad_size = SZ - bytes.len();
        let last_idx = SZ - 1;
        let start_idx = last_idx - (bytes.len() - 1);
        extended_bytes[start_idx..].clone_from_slice(&bytes);
        let padding_len = SZ - bytes.len();
        for i in (0..pad_size - 1) {
            extended_bytes[i] = 0;
        }
    } else {
        extended_bytes.copy_from_slice(bytes);
    }
    extended_bytes
}

fn push_op(sz: u8, val: &[u8]) -> Instruction {
    // let val = if val.len() < 8 {
    //     let mut zero_len = if val.len() < 8 {8 - val.len()} else {0};
    //     let mut buf = vec![];
    //     for _ in (0..zero_len) {
    //         buf.push(0);
    //     }
    //     buf.extend_from_slice(val);

    //     let mut sliced = [0u8; 8];
    //     sliced.copy_from_slice(&buf);
    //     u64::from_be_bytes(sliced)
    // } else {
    //     let mut buf = vec![];
    //     buf.copy_from_slice()
    //     u64::from_be_bytes(val)
    // };

    match sz {
        1 => {
            let val = zero_extend::<1>(val);
            push1(val.into())
        }
        2 => {
            let val = zero_extend::<2>(val).into();
            push2(val)
        }
        3 => {
            let val = zero_extend::<3>(val).into();
            push3(val)
        }
        4 => {
            let val = zero_extend::<4>(val).into();
            push4(val)
        }
        5 => {
            let val = zero_extend::<5>(val).into();
            push5(val)
        }
        6 => {
            let val = zero_extend::<6>(val).into();
            push6(val)
        }
        7 => {
            let val = zero_extend::<7>(val).into();
            push7(val)
        }
        8 => {
            let val = zero_extend::<8>(val).into();
            push8(val)
        }
        9 => {
            let val = zero_extend::<9>(val).into();
            push9(val)
        }
        10 => {
            let val = zero_extend::<10>(val).into();
            push10(val)
        }
        11 => {
            let val = zero_extend::<11>(val).into();
            push11(val)
        }
        12 => {
            let val = zero_extend::<12>(val).into();
            push12(val)
        }
        13 => {
            let val = zero_extend::<13>(val).into();
            push13(val)
        }
        14 => {
            let val = zero_extend::<14>(val).into();
            push14(val)
        }
        15 => {
            let val = zero_extend::<15>(val).into();
            push15(val)
        }
        16 => {
            let val = zero_extend::<16>(val).into();
            push16(val)
        }
        17 => {
            let val = zero_extend::<17>(val).into();
            push17(val)
        }
        18 => {
            let val = zero_extend::<18>(val).into();
            push18(val)
        }
        19 => {
            let val = zero_extend::<19>(val).into();
            push19(val)
        }
        20 => {
            let val = zero_extend::<20>(val).into();
            push20(val)
        }
        21 => {
            let val = zero_extend::<21>(val).into();
            push21(val)
        }
        22 => {
            let val = zero_extend::<22>(val).into();
            push22(val)
        }
        23 => {
            let val = zero_extend::<23>(val).into();
            push23(val)
        }
        24 => {
            let val = zero_extend::<24>(val).into();
            push24(val)
        }
        25 => {
            let val = zero_extend::<25>(val).into();
            push25(val)
        }
        26 => {
            let val = zero_extend::<26>(val).into();
            push26(val)
        }
        27 => {
            let val = zero_extend::<27>(val).into();
            push27(val)
        }
        28 => {
            let val = zero_extend::<28>(val).into();
            push28(val)
        }
        29 => {
            let val = zero_extend::<29>(val).into();
            push29(val)
        }
        30 => {
            let val = zero_extend::<30>(val).into();
            push30(val)
        }
        31 => {
            let val = zero_extend::<31>(val).into();
            push31(val)
        }
        32 => {
            let val = zero_extend::<32>(val).into();
            push32(val)
        }
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
        _ => todo!(),
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
        _ => todo!(),
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

#[derive(Default, Debug, Clone)]
pub struct Program {
    pub map: HashMap<usize, Instruction>,
    pgm: Vec<Instruction>,
    pub size: usize,
    pub bytes: Vec<BitVec<1>>
}

impl Program {
    pub fn get(&self, pc: usize) -> Option<Instruction> {
        self.map.get(&pc).cloned()
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

#[test]
fn is_push_works() {
    for b in (0x60_u8..0x7f) {
        assert!(is_push(b));
    }

    for b in (0x00_u8..0x5f) {
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
    // eprintln!("SIXTY FOUR: {:#?}", sixty_four);
    let expected = vec![
        Instruction::Push1(bvi(0x42)),
        Instruction::Push1(bvi(0)),
        Instruction::MStore,
        Instruction::Push1(bvi(0x20)),
        Instruction::Push1(bvi(0)),
        push2(bvi(0x0040)),
        Instruction::Return,
    ];

    assert_eq!(expected, pgm.pgm);
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
        push2(bvi(0x0197)),
    ];

    let inst10 = pgm.pgm.get(7).unwrap().clone();
    let inst10map = pgm.get(11).unwrap();
    assert_eq!(inst10, inst10map);
    let pgm_first_30 = (&pgm.pgm[..33]).to_vec();
    assert_eq!(expected_first_30, pgm_first_30);
    let pgm_map = pgm.map.into_iter().collect::<Vec<_>>();

    eprintln!("PROGRAM MAP INDICES: {:#?}", pgm_map);
    let pgm_enumd = pgm_first_30.into_iter().enumerate().collect::<Vec<_>>();
    eprintln!("Enumerated pgm indices: {:#?}", pgm_enumd);
    // assert_eq!(pgm_enumd, pgm_map);
}
