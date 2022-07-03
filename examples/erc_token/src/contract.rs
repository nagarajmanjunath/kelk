use crate::error::TokenError;
use crate::message::{ProcMsg, QueryMsg, QueryRsp};
use kelk_env::context::Context;
use kelk_lib::alloc::vec::Vec;
use kelk_lib::collections::bst::tree::StorageBST;

fn transfer(ctx: Context, from: [u8; 4], to: [u8; 4], amount: i64) -> Result<(), TokenError> {
    let mut bst: StorageBST<[u8; 4], i64> = StorageBST::lazy_load(ctx.api, 0).unwrap(); // FIXME: no unwrap
    let tx_balance = match bst.find(&from).unwrap() {
        Some(balance) => balance,
        None => 0,
    };

    let rx_balance = match bst.find(&to).unwrap() {
        Some(balance) => balance,
        None => 0,
    };

    if tx_balance < amount {
        return Err(TokenError::InsufficientAmount);
    }

    bst.insert(from, tx_balance - amount).unwrap();
    bst.insert(to, rx_balance + amount).unwrap();

    Ok(())
}

fn query_result(ctx: Context) -> Result<i32, TokenError> {
    ctx.api.sread_i32(0).map_err(|_| TokenError::KelkError)
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
pub fn process_msg(ctx: Context, msg: ProcMsg) -> Result<(), TokenError> {
    match msg {
        ProcMsg::Transfer { from, to, amount } => transfer(ctx, from, to, amount),
    }
}

// #[kelk_derive(query)]
pub fn query(ctx: Context, msg: QueryMsg) -> Result<QueryRsp, TokenError> {
    let res = match msg {
        QueryMsg::LastResult => query_result(ctx),
    }?;

    Ok(QueryRsp::Result { res })
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod contract_test;
