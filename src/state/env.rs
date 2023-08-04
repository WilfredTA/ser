use z3_ext::ast::{Array, Ast, AstKind, BV};

use z3_ext::FuncDecl;
use z3_ext::Sort;

use crate::random_bv_arg;
use crate::smt::{ctx, BitVec};

/**
    Note: Some of these functions in EVM have no arguments.
    The reason they are passed an argument here is because a zero argument function is
    mathematically considered a constant. Yet, the EVM equivalent functions are *not* constants;
    rather, they depend on the state of the machine at the time of execution. By providing a function domain,
    we can generate unique values. One example is 'GAS'. While GAS takes zero arguments from the stack,
    it is parameterized over the state of the machine. We must therefore provide an argument to the function
    in z3 in order to ensure that GAS is not treated as a constant.
*/
pub fn chain_id<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "chainid", &[], &Sort::bitvector(ctx, 256))
}
pub fn call_data_load<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(
        ctx,
        "calldataload",
        &[&Sort::bitvector(ctx, 256)],
        &Sort::bitvector(ctx, 256),
    )
}

pub fn sha3<'ctx>(size: u32) -> FuncDecl<'ctx> {
    let id = uuid::Uuid::new_v4();
    let func = FuncDecl::new(
        ctx(),
        format!("sha3_{}", id).as_str(),
        &[&Sort::bitvector(ctx(), size)],
        &Sort::bitvector(ctx(), 256),
    );

    eprintln!("SHA3 FUNC: {:#?}", func);
    func
}

pub fn call_value<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "callvalue", &[], &Sort::bitvector(ctx, 256))
}

pub fn call_data_size<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "calldatasize", &[], &Sort::bitvector(ctx, 256))
}

pub fn caller<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "caller", &[], &Sort::bitvector(ctx, 256))
}

pub fn origin<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "origin", &[], &Sort::bitvector(ctx, 256))
}

pub fn address() -> BitVec<20> {
    random_bv_arg()
}

// Takes random bitvec as argument so that gas is not treated as a constant function.
pub fn gas<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(
        ctx,
        "gas",
        &[&Sort::bitvector(ctx, 256)],
        &Sort::bitvector(ctx, 256),
    )
}

pub fn gas_lim<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "gaslimit", &[], &Sort::bitvector(ctx, 256))
}

pub fn gas_price<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "extcodesize", &[], &Sort::bitvector(ctx, 256))
}

pub fn coinbase<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "coinbase", &[], &Sort::bitvector(ctx, 256))
}

pub fn block_hash<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(
        ctx,
        "blockhash",
        &[&Sort::bitvector(ctx, 256)],
        &Sort::bitvector(ctx, 256),
    )
}

pub fn ext_code_size<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(
        ctx,
        "extcodesize",
        &[&Sort::bitvector(ctx, 256)],
        &Sort::bitvector(ctx, 256),
    )
}

pub fn block_num<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "blocknumber", &[], &Sort::bitvector(ctx, 256))
}

pub fn timestamp<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "timestamp", &[], &Sort::bitvector(ctx, 256))
}

pub fn difficulty<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "difficulty", &[], &Sort::bitvector(ctx, 256))
}
pub fn ext_code_hash<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(
        ctx,
        "extcodehash",
        &[&Sort::bitvector(ctx, 256)],
        &Sort::bitvector(ctx, 256),
    )
}

// We add an extra argument here because the balance of an address is not necessarily the same during
// every step of a contract's execution.
pub fn balance<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(
        ctx,
        "balance",
        &[&Sort::bitvector(ctx, 256), &Sort::bitvector(ctx, 256)],
        &Sort::bitvector(ctx, 256),
    )
}
