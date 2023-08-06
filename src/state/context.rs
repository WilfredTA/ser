use std::collections::HashMap;

use ruint::aliases::U256;
use z3_ext::ast::{Ast, Bool};

use crate::{smt::BitVec, storage::Address, parser::Program, bvi, conversion::bitvec_array_to_bv, random_bv_arg};

use super::env::{call_data_size, call_data_load, caller};

#[derive(Debug, Clone, Default)]
pub struct ExecutionEnv<'ctx> {
    code: HashMap<Address, Program>,
    block: Block,
    tx: TransactionContext,
    result: Option<EvmResult>,
    logs: Vec<Log>,
    balances: HashMap<Address, U256>,
    constraints: Vec<Bool<'ctx>>
}


impl<'ctx> ExecutionEnv<'ctx> {

    pub fn gen_random_address_20() -> Address {
        random_bv_arg()
    }

    // Zero-padded 32-byte address
    pub fn gen_random_address_32() -> BitVec<32> {
        let bv: BitVec<20> = random_bv_arg();
        let bv_ref = bv.as_ref();
        let bv_new = bv_ref.zero_ext((12 * 8)).simplify();
        bv_new.into()
    }

    pub fn set_balance(mut self, addr: Address, bal: impl Into<U256>) -> Self {
        self.balances.insert(addr, bal.into());
        self
    }

    pub fn set_caller(mut self, caller: Address) -> Self {
        self.tx.caller = Some(caller);
        self
    }

    pub fn set_calldata(mut self, calldata: &str) -> Self {
        let cd_bytes = hex::decode(calldata).expect("Unable to decode calldata string");
        let cd_bytes: Vec<BitVec<1>> = cd_bytes.into_iter().map(|b| BitVec::from([b; 1])).collect();
        self.tx.calldata = Some(cd_bytes);
        self
    } 

    pub fn caller(&self) -> BitVec<32> {
        if let Some(ref caller) = self.tx.caller {
            caller.as_ref().clone().into()
        } else {
            caller().apply(&[]).as_bv().unwrap().into()
        }
    }
    
    pub fn get_contract_code(&self, addr: &Address) -> Option<&Program> {
        self.code.get(addr)
    }

    pub fn calldatasize(&self) -> BitVec<32> {
        if let Some(ref cd) = self.tx.calldata {
            bvi(cd.len() as i32)
        } else {
            let call_data_sz = call_data_size().apply(&[]).as_bv().unwrap();
            call_data_sz.into()
        }
    }

    pub fn calldataload(&self, offset: &BitVec<32>) -> BitVec<32> {
        
        if let Some(ref cd) = self.tx.calldata {
            let offset = offset.clone().into();
            let bv_arr = cd[offset..].to_vec();
            let bv = bitvec_array_to_bv(bv_arr);
            let bv = BitVec::from(bv);
            bv
        } else {
            call_data_load().apply(&[offset.as_ref()]).as_bv().unwrap().into()
        }
    }

}
// Note: tradeoffs between Log with LogTopic enum vs top-lvl enum for Log vs merely a struct
// with a [Option<BitVec<32>>; 4] array for topics...
#[derive(Debug, Clone)]
pub struct Log {
    data: Vec<BitVec<1>>,
    topics: LogTopic
}

#[derive(Debug, Clone)]
pub enum LogTopic{
    One(BitVec<32>),
    Two(BitVec<32>, BitVec<32>),
    Three(BitVec<32>, BitVec<32>, BitVec<32>),
    Four(BitVec<32>, BitVec<32>, BitVec<32>, BitVec<32>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvmResult {
    Failed {
        msg: String
    },
    Success {
        ret_val: BitVec<32>
    }
}

impl Default for EvmResult {
    fn default() -> Self {
        Self::Failed { msg: "Execution unfinished".to_string() }
    }
}
#[derive(Debug, Clone, Default)]
pub struct TransactionContext {
    calldata: Option<Vec<BitVec<1>>>,
    caller: Option<Address>,
    callvalue: Option<BitVec<32>>,
}

#[derive(Debug, Clone, Default)]
pub struct Block {
    base_fee: Option<BitVec<32>>,
    chain_id: Option<BitVec<32>>,
    coinbase: Option<Address>,
    difficulty: Option<BitVec<32>>,
    gaslimit: Option<BitVec<32>>,
    number: Option<BitVec<32>>,
    timestamp: Option<BitVec<32>>
}