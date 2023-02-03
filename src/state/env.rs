use z3_ext::ast::{Ast, AstKind};

use z3_ext::FuncDecl;
use z3_ext::Sort;

use crate::smt::ctx;

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

pub fn address<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(ctx, "address", &[], &Sort::bitvector(ctx, 256))
}

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
pub fn balance<'ctx>() -> FuncDecl<'ctx> {
    let ctx = ctx();
    FuncDecl::new(
        ctx,
        "balance",
        &[&Sort::bitvector(ctx, 256)],
        &Sort::bitvector(ctx, 256),
    )
}
