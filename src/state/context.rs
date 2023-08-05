// use crate::smt::BitVec;

// #[derive(Debug, Clone)]
// pub struct ExecutionEnv {
//     code: HashMap<Address, Option<Program>>,
//     block: Block,
//     tx: TransactionContext,
//     result: Option<EvmResult>,
//     logs: Vec<Log>
// }

// // Note: tradeoffs between Log with LogTopic enum vs top-lvl enum for Log vs merely a struct
// // with a [Option<BitVec<32>>; 4] array for topics...
// #[derive(Debug, Clone)]
// pub struct Log {
//     data: Vec<BitVec<1>>,
//     topics: LogTopic
// }

// #[derive(Debug, Clone)]
// pub enum LogTopic{
//     One(BitVec<32>),
//     Two(BitVec<32>, BitVec<32>),
//     Three(BitVec<32>, BitVec<32>, BitVec<32>),
//     Four(BitVec<32>, BitVec<32>, BitVec<32>, BitVec<32>),
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum EvmResult {
//     Failed {
//         msg: String
//     },
//     Success {
//         ret_val: BitVec<32>
//     }
// }
// #[derive(Debug, Clone)]
// pub struct TransactionContext {
//     calldata: Option<Vec<BitVec<1>>>,
//     caller: Option<Address>,
//     callvalue: Option<BitVec<32>>,
// }

// #[derive(Debug, Clone)]
// pub struct Block {
//     base_fee: Option<BitVec<32>>,
//     chain_id: Option<BitVec<32>>,
//     coinbase: Option<Address>,
//     difficulty: Option<BitVec<32>>,
//     gaslimit: Option<BitVec<32>>,
//     number: Option<BitVec<32>>,
//     timestamp: Option<BitVec<32>>
// }