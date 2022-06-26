use crate::error::CalcError;
use crate::message::{ProcMsg, QueryMsg, QueryRsp};
use kelk_env::context::Context;
use kelk_lib::alloc::vec::Vec;
use kelk_lib::collections::bst::tree::StorageBST;

fn transfer(ctx: Context, from: Vec<u8>, to: Vec<u8>, amount: i64) -> Result<(), CalcError> {
    let bst :StorageBST<Vec<u8>, Vec<u8>> = StorageBST::lazy_load(ctx.api, 4).unwrap(); // FIXME: no unwrap
    todo!()
}

fn query_result(ctx: Context) -> Result<i32, CalcError> {
    ctx.api.sread_i32(0).map_err(|_| CalcError::KelkError)
}

#[cfg(target_arch = "wasm32")]
mod __wasm_export_process_msg {
    #[no_mangle]
    extern "C" fn process_msg(msg_ptr: u64) -> u64 {
        kelk_env::do_process_msg(&super::process_msg, msg_ptr)
    }
}

#[cfg(target_arch = "wasm32")]
mod __wasm_export_query {
    #[no_mangle]
    extern "C" fn query(msg_ptr: u64) -> u64 {
        kelk_env::do_query(&super::query, msg_ptr)
    }
}

// #[kelk_derive(process_msg)]
pub fn process_msg(ctx: Context, msg: ProcMsg) -> Result<(), CalcError> {
    match msg {
        ProcMsg::Transfer { from, to, amount } => transfer(ctx, from, to, amount),
    }
}

// #[kelk_derive(query)]
pub fn query(ctx: Context, msg: QueryMsg) -> Result<QueryRsp, CalcError> {
    let res = match msg {
        QueryMsg::LastResult => query_result(ctx),
    }?;

    Ok(QueryRsp::Result { res })
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod contract_test;
