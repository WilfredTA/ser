use ruint::{uint, Uint, ToUintError, FromUintError, Bits};
use rlp::{Encodable, Decodable};
use z3_ext::ast::Ast;
use serde::{Deserialize, Serialize};
use crate::{BV, BVType, BitVec, bvc, bvi, ctx, bvi_8byte};

impl From<Uint<256, 4>> for BitVec<32> {
    fn from(value: Uint<256, 4>) -> Self {
       let mut bv: BV<'static> = BV::from_u64(ctx(), 0, 8);
       let bytes: [u8; 32] = value.clone().to_be_bytes();

       for i in bytes.iter() {
           let new_bv: BV<'static> = bvi::<1>(*i).into();
           bv = bv.concat(&new_bv).simplify();
       }
       bv.extract(256 - 8 - 1, 0).simplify().into()

    }
}

impl From<BitVec<32>> for Uint<256, 4> {
    fn from(value: BitVec<32>) -> Self {
        let value: BV<'static> = value.as_ref().clone();
        let mut numbits = [0u8; 32];

        for i in (0..32_u32) {
            let offset = 256 - (i * 8) - 1;
            let byte_extract: BV<'static> = value.extract(offset, offset - 7).simplify();
            // since byte_extract is a single byte, downcasting to u8 will not change the number
            let byte =  byte_extract.as_u64().unwrap() as u8;
            numbits[i as usize] = byte;

            
        }
        Bits::from_be_bytes(numbits).as_uint().clone()
        
    }
}


#[test]
fn test_u256_to_bytes() {
    let num = uint!(0xc85ef7d79691fe79573b1a7064c19c1a9819ebdbd1faaab1a8ec92344438aaf4_U256);

    let mut buf = [0u8; 32];

    let bytes: [u8; 32] = num.to_be_bytes();

    let numbit: Bits<256, 4 >= Bits::from_be_bytes(bytes);
    let newnum: Uint<256, 4> = numbit.as_uint().clone();
    assert_eq!(num, newnum);
}

#[test]
fn test_to_bv() {
    let num = uint!(0xc85ef7d79691fe79573b1a7064c19c1a9819ebdbd1faaab1a8ec92344438aaf4_U256);
    let mut bv: BitVec<32> = num.into();

    let small_num = uint!(0x0000000000000000000000000000000000000000000000000000000000000009_U256);
    let mut bv_2 = BitVec::from(small_num);
    bv_2.simplify();
    let mut expected = bvi(9);
    expected.simplify();
    assert_eq!(expected, bv_2);

}

#[test]
fn test_from_bv() {
    let bv = bvi(327000);
    let num = uint!(0x000000000000000000000000000000000000000000000000000000000004FD58_U256);
    let bv_to_num: Uint<256, 4> = bv.into();
    assert_eq!(num, bv_to_num);

}