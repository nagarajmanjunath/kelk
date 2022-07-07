use crate::error::TokenError;
use crate::message::{Address, InstansiteMsg, ProcMsg, QueryMsg, QueryRsp};
use kelk_env::context::Context;
use kelk_env::storage::{sread_struct, swrite_struct};
use kelk_lib::collections::bst::tree::StorageBST;

fn transfer(ctx: Context, to: Address, amount: i64) -> Result<(), TokenError> {
    let from = sread_struct::<Address>(ctx.storage, 0)?;
    transfer_from(ctx, from, to, amount)
}

fn name(ctx: Context) -> Result<String, TokenError> {
    let res = ctx.storage.sread(5, 64).unwrap();
    let name = core::str::from_utf8(&res).expect("Found invalid UTF-8");
    Ok(name.to_string())
}

fn symbol(ctx: Context) -> Result<String, TokenError> {
    let res = ctx.storage.sread(69, 76).unwrap();
    let symbol = core::str::from_utf8(&res).expect("Found invalid UTF-8");
    Ok(symbol.to_string())
}

fn transfer_from(ctx: Context, from: Address, to: Address, amount: i64) -> Result<(), TokenError> {
    let mut bst: StorageBST<Address, i64> = StorageBST::lazy_load(ctx.storage, 128).unwrap(); // FIXME: no unwrap
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

#[cfg(target_arch = "wasm32")]
mod __wasm_export_process_msg {
    #[no_mangle]
    extern "C" fn process_msg(msg_ptr: u64) -> u64 {
        kelk_env::do_process_msg(&super::process_msg, msg_ptr)
    }
}

#[cfg(target_arch = "wasm32")]
mod __wasm_export_instantiate {
    #[no_mangle]
    extern "C" fn instantiate(msg_ptr: u64) -> u64 {
        kelk_env::do_instantiate(&super::instantiate, msg_ptr)
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
        ProcMsg::Transfer { to, amount } => transfer(ctx, to, amount),
        ProcMsg::TransferFrom { from, to, amount } => transfer_from(ctx, from, to, amount),
    }
}

// #[kelk_derive(instantiate)]
pub fn instantiate(ctx: Context, msg: InstansiteMsg) -> Result<(), TokenError> {
    if msg.name.len() > 64 {
        return Err(TokenError::InvalidMsg);
    }
    if msg.symbol.len() > 8 {
        return Err(TokenError::InvalidMsg);
    }
    swrite_struct::<Address>(ctx.storage, 0, &msg.owner).unwrap();
    ctx.storage.swrite(5, msg.name.as_bytes()).unwrap();
    ctx.storage.swrite(69, msg.symbol.as_bytes()).unwrap();
    ctx.storage.swrite_i64(77, msg.total_supply).unwrap();
    let mut bst: StorageBST<Address, i64> = StorageBST::create(ctx.storage, 128, 1000).unwrap();
    // FIXME unwrap()
    bst.insert(msg.owner, msg.total_supply).unwrap();
    Ok(())
}
// #[kelk_derive(query)]
pub fn query_result(ctx: Context, msg: QueryMsg) -> Result<QueryRsp, TokenError> {
    let res = match msg {
        QueryMsg::Name => QueryRsp::NameRsp { res: name(ctx)? },
    };

    Ok(res)
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod contract_test;
