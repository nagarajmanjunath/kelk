//! Kelk-lib is the standard library for wasm based contracts in [Zarb](https://zarb.network blockchain.
//!
#![no_std]
#![deny(
    missing_docs,
    bad_style,
    bare_trait_objects,
    const_err,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates
)]

pub mod collections;
pub mod error;
pub mod mock;
pub mod storage;

pub extern crate alloc;

#[macro_use]
extern crate doc_comment;
